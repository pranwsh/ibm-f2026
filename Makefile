SHELL := /bin/bash
COMPOSE = docker compose --env-file .env -f infra/docker-compose.yml
.PHONY: up down test lint fmt
up:
	@test -f .env || cp .env.example .env
	$(COMPOSE) up -d --build --wait --wait-timeout 300

down:
	$(COMPOSE) down

test:
	cd backend && uv sync --group dev && uv run pytest -q
	cd detector && cargo test
	cd frontend && bun test

lint:
	cd backend && uv sync --group dev && uv run ruff check .
	cd detector && cargo clippy --all-targets -- -D warnings
	cd frontend && bun run lint

fmt:
	cd backend && uv sync --group dev && uv run ruff format .
	cd detector && cargo fmt
	cd frontend && bun run format
