mod config;
mod log;
mod template;

use async_compat::CompatExt;
use dotenv_rs::dotenv;
use mastodon_async::prelude::*;
use serde::Deserialize;
use std::sync::Arc;
use tide::{Request, StatusCode};
use tracing::{error, info, warn};
use valuable::Valuable;

#[derive(Clone, Debug)]
struct State {
    config: Arc<config::Config>,
    mastodon: Arc<Mastodon>,
}

#[derive(Debug, Deserialize, Valuable)]
struct Input {
    kind: config::AlertKind,
    group: String,
    name: String,
    description: Option<String>,
}

#[tokio::main]
async fn main() -> mastodon_async::Result<()> {
    dotenv().ok();
    let _ = log::setup();
    info!("Starting...");
    let config = config::init().expect("Invalid configuration.");
    let mastodon = Mastodon::from(Data {
        base: config.host.clone().into(),
        client_id: config.client_id.clone().into(),
        client_secret: config.client_secret.clone().into(),
        redirect: Default::default(),
        token: config.token.clone().into(),
    });

    let you = mastodon.verify_credentials().await?;
    info!("Authenticated with {} as {}.", &config.host, &you.acct);

    let mut app = tide::with_state(State {
        config: Arc::new(config),
        mastodon: Arc::new(mastodon),
    });
    app.with(tide::log::LogMiddleware::new());
    app.at("/").post(process);
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}

async fn process(mut req: Request<State>) -> tide::Result {
    let input: Input = req.body_json().await.map_err(|e| {
        warn!(error = e.to_string(), "Failed reading in request.");
        e
    })?;
    let state = req.state();
    info!(input = input.as_value(), "Got input.");
    let msg = template::render(
        state
            .config
            .tpl_config
            .template_for(&input.group, &input.name)
            .get(input.kind),
        &input.group,
        &input.name,
        state
            .config
            .tpl_config
            .service(&input.group, &input.name)
            .and_then(|svc| svc.friendly_name.as_deref()),
    )
    .map_err(|e| {
        error!(
            error = e,
            input = input.as_value(),
            "Failed rendering template."
        );
        tide::Error::from_str(
            StatusCode::InternalServerError,
            "Failed rendering template.",
        )
    })?;
    info!(
        input = input.as_value(),
        msg = msg,
        "Got event. - Preparing to post."
    );

    if state.config.live {
        post(&state.mastodon, msg).await.map_err(|err| {
            error!(
                error = err.to_string(),
                input = input.as_value(),
                "Failed posting to Mastodon."
            );
            tide::Error::from_str(
                StatusCode::InternalServerError,
                "Failed posting to Mastodon.",
            )
        })?;
    } else {
        warn!("Live not enabled. Not posting.");
    }

    Ok("ok".into())
}

async fn post(mastodon: impl AsRef<Mastodon>, msg: String) -> mastodon_async::Result<Status> {
    mastodon
        .as_ref()
        .new_status(NewStatus {
            status: Some(msg),
            in_reply_to_id: None,
            media_ids: None,
            sensitive: None,
            spoiler_text: None,
            visibility: Some(Visibility::Public),
            language: None,
            content_type: None,
        })
        .compat()
        .await
}
