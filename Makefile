SHELL := /bin/bash
COMPOSE = docker compose --env-file .env -f infra/docker-compose.yml
.PHONY: up down test lint fmt
up:
	@test -f .env || cp .env.example .env
	$(COMPOSE) up -d --build --wait

down:
	$(COMPOSE) down

test:
	cd backend && uv run pytest
	cd detector && cargo test
	cd frontend && bun test

lint:
	cd backend && uv run ruff check .
	cd detector && cargo clippy --all-targets -- -D warnings
	cd frontend && bun run lint

fmt:
	cd backend && uv run ruff format .
	cd detector && cargo fmt
	cd frontend && bun run format
