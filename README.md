# Axum Auth Demo

A demo backend application using `axum` which implements an OAuth2
authentication process.

The authentication is mainly implemented in `src/auth.rs`. `FromRequest` is
implemented for `AuthenticatedUser` to simplify usage. See
`crate::handler::greet::greet_handler` and `crate::handler::info::info_handler`
for usage.

## Requirement

This application uses authentik as an external identity provider. You can
follow the [instructions][1] to setup authentik.

[1]: https://docs.goauthentik.io/install-config/install/docker-compose/

Then you should follow the [document][2] to create an application in authentik.

[2]: https://docs.goauthentik.io/install-config/first-steps/

You can get a token with username and app password using `/application/o/token`
endpoint.

> [!NOTE]
> App password is not your account password. It can be generated at Credentials
> tab in your user settings page.

Example Python script:

```python
import requests


AUTHENTIK_BASE_URL = "http://localhost:9000"
CLIENT_ID = "zaimI2..."
USERNAME = "foo"
APP_PASSWORD = "UHcG5b..."

token_url = f"{AUTHENTIK_BASE_URL}/application/o/token/"
data = {
    "grant_type": "password",
    "username": USERNAME,
    "password": APP_PASSWORD,
    "client_id": CLIENT_ID,
    "scope": "openid profile email",
}
headers = {
    "Content-Type": "application/x-www-form-urlencoded",
}
response = requests.post(
    token_url,
    data=data,
    headers=headers,
)
response.raise_for_status()

print(response.json()["access_token"])
```

## Settings

This application loads settings at `config.toml`.

### `server` section

- `host`: **Optional**, default to `"localhost"`.
- `port`: **Optional**, default to `3000`.

### `app` section

- `slug`: **Required**. Application slug.
- `client-id`: **Required**. Client ID of the provider for the authentik
  application.
- `client-secret`: **Required**. Client secret of the proider. Not used yet.
  You can simply pass an empty string.
- `authentik-base-url`: **Optional**. Base URL of authentik service, default to
  `http://localhost:9000`.
- `redirect-uri`: **Required**. The redirect URI for the OAuth flow. Not used
  yet. You can simply pass an empty string.
