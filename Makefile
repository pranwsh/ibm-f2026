SHELL := bash
APP := apps/app
.PHONY: install dev build typecheck lint format format-check fmt
install:
	npm ci --prefix $(APP)
dev:
	npm run dev --prefix $(APP)
build:
	npm run build --prefix $(APP)
typecheck:
	npm run typecheck --prefix $(APP)
lint:
	npm run lint --prefix $(APP)
format:
	npm run format --prefix $(APP)
format-check:
	npm run format:check --prefix $(APP)
fmt: format
