# Axum Auth Demo

A demo application using `axum` as backend which implements an OAuth 2.0
authentication flow.

The authentication is mainly implemented in `src/auth.rs`. `FromRequest` is
implemented for `AuthenticatedUser` to simplify usage. See
`crate::handler::greet::handler` and `crate::handler::info::handler`
for usage.

You can test this application using the demo front end in `/pages`:

```shell
$ cd pages
$ pnpm install
$ pnpm dev
```

## Requirements

This application uses authentik as an external identity provider. You can
follow the [instructions][1] to setup authentik.

[1]: https://docs.goauthentik.io/install-config/install/docker-compose/

Then you should follow the [document][2] to create an application in authentik.

[2]: https://docs.goauthentik.io/install-config/first-steps/

### Extra Requirements for Logout

OAuth 2.0 demand the usage of HTTPS scheme in revocation URL. If you want to
use the logout handler, you need to setup an HTTPS reverse proxy server. You
can use [Caddy][caddy] with the following `Caddyfile`:

[caddy]: https://caddyserver.com

```caddyfile
localhost

reverse_proxy localhost:9000
```

and start a automatical HTTPS server with:

```shell
$ caddy run --config Caddyfile
```

## Settings

This application loads settings at `config.toml`.

### `server` section

- `host`: **Optional**, default to `"localhost"`.
- `port`: **Optional**, default to `3000`.

### `app` section

- `slug`: **Required**. Application slug.
- `client-id`: **Required**. Client ID of the provider for the authentik.
  application.
- `client-secret`: **Required**. Client secret of the proider.
- `authentik-base-url`: **Optional**. Base URL of authentik service, default to
  `http://localhost:9000`.
- `redirect-uri`: **Required**. The redirect URI for the OAuth flow.

