COMPOSE_DEV  = docker compose -f docker-compose.yaml -f docker-compose.dev.yaml
COMPOSE_PROD = docker compose -f docker-compose.yaml -f docker-compose.prod.yaml

# Dev falls back to a local default password unless it is set in the shell or .env
ifeq ($(shell grep -s '^POSTGRES_PASSWORD=' .env),)
dev dev-down dev-logs: export POSTGRES_PASSWORD ?= 123456789
endif

.PHONY: dev dev-down dev-logs prod prod-down prod-logs

dev:
	$(COMPOSE_DEV) up --build

dev-down:
	$(COMPOSE_DEV) down

dev-logs:
	$(COMPOSE_DEV) logs -f

prod:
	$(COMPOSE_PROD) up -d --build

prod-down:
	$(COMPOSE_PROD) down

prod-logs:
	$(COMPOSE_PROD) logs -f
