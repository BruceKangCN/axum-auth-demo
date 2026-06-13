# Axum Auth Demo

A demo backend application using `axum` which implements an OAuth2
authentication process.

The authentication is mainly implemented in `src/auth.rs`. `FromRequest` is
implemented for `AuthenticatedUser` to simplify usage. See
`crate::handler::greet::greet_handler` and `crate::handler::info::info_handler`
for usage.

## Requirement

This application requires an external authorization provider. You can follow
the [instructions][1] to setup authentik as provider.

[1]: https://docs.goauthentik.io/install-config/install/docker-compose/

Then you should follow the [document][2] to create an application in authentik.

[2]: https://docs.goauthentik.io/install-config/first-steps/

## Settings

This application loads settings at `config.toml`.

### `server` section

- `host`: **Optional**, default to `"localhost"`.
- `port`: **Optional**, default to `3000`.

### `app` section

- `client-id`: **Required**. Client ID of the provider for the authentik
  application.
- `client-secret`: **Required**. Client secret of the proider. Not used yet.
  You can simply pass an empty string.
- `jwk-set-url`: **Optional**. JSON Web Key Set API URL of the provider,
  default to `http://localhost:9000/application/o/demo/jwks/`.
