# Development

Install Node.js 22.12+ (or 20.19+) and npm. Alternatively, use `nix develop` or `direnv allow` with Nix flakes enabled. From the repository root:

```sh
make install       # npm ci in apps/app
make dev           # start Vite; open the URL it prints
make build         # production build in apps/app/dist
make typecheck     # tsrs --noEmit
make lint          # oxlint
make format        # format with oxfmt
make format-check  # check formatting without modifying files
```

The equivalent npm scripts (`dev`, `build`, `typecheck`, `lint`, `format`, and `format:check`) can be run from `apps/app/` with `npm run <script>`. tsrs is an experimental TypeScript checker; TypeScript is also installed for editor support.

To enable the hooks, install pre-commit, run `make install`, then `pre-commit install` from the repository root. The hooks run type checking, linting and formatting checks when application files change; `pre-commit run --all-files` runs them manually. Run `make format` to fix formatting before committing.
