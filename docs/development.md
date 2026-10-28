# Development

## Requirements

- Docker Engine and Docker Compose v2 for the local stack.
- Nix with flakes enabled for the development toolchain, or the tools listed in `flake.nix` installed locally.
- direnv is optional.

## Setup

```sh
cp .env.example .env
```

To use the Nix environment:

```sh
nix develop
```

Or, with direnv installed:

```sh
direnv allow
```

Start and stop the services:

```sh
make up
make down
```

The current `test`, `lint`, and `fmt` Make targets are scaffolding and require their respective language toolchains. Application tests and full lint/format workflows are not yet configured.

Local credentials in `.env.example` are for development only. Do not use them outside a local environment.
