# Mastodon Status Bot

Bot to publish PMDCollab website status updates on Mastodon.

## Configuration

### Environment Variables

- `MSB_HOST`: Mastodon Host, incl. protocol
- `MSB_CLIENT_KEY`: API Client Key
- `MSB_CLIENT_SECRET`: API Client Secret
- `MSB_ACCESS_TOKEN`: Bot user access token
- `MSB_CONFIG_FILE`: Path to the config TOMl file
- `MSB_LIVE`: If set to `true` posts to Mastodon are set. In other cases, they are only logged (dry-run).
- `RUST_LOG`: Control logging level

Automatically reads in `.env` files in the current working directory.

### Config file
See `config.example.toml`.

### API
The entire app works via one API endpoint at `POST /` that takes JSON in this format:

```json5
{
  // What kind of alert: TRIGGERED for start or RESOLVED for resolved.
 "kind": "...",
 // Name of the group of the alert. Matched against.
 "group": "...",
 // Name of the alert. Matched against.
 "name": "...",
 // Description that should be included in the message. Non-existent entries, null or empty strings are ignored.
 "description": "..."
}
```

If group and name can not be matched, a generic fallback message that can be configured
is used instead.
