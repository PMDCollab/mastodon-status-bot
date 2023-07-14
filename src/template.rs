use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct Context<'a> {
    name: &'a str,
    group: &'a str,
    friendly_name: Option<&'a str>,
}

pub fn render(
    tpl: &str,
    group: &str,
    name: &str,
    friendly_name: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("template", tpl)?;
    tt.render(
        "template",
        &Context {
            name,
            group,
            friendly_name,
        },
    )
    .map_err(Into::into)
}
