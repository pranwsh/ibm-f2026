# Signal Pipeline

A minimal React and TypeScript application scaffold, built with Vite and TanStack Router. It does not yet connect to an API. Ingestion and detection are deferred to separate work.

## Quick start

Requires Node.js 22.12+ (or 20.19+) and npm. From the repository root:

```sh
make install
make dev
```

Open the local URL printed by Vite. See [Development](docs/development.md) for all commands and pre-commit setup.

## Repository layout

- `apps/app/`: frontend application
- `content/rules/`: reserved for detection rules
- `docs/`: development documentation
- `fixtures/`: reserved for sample data

Future independently running services belong under `services/api/` and `services/detector/`; shared packages, if needed, belong under `packages/` (for example `packages/contracts/`). Those components are not part of this scaffold.
