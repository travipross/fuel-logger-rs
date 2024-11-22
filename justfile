set dotenv-path := "dev/.env"
dotenv-path := "dev/.env"
docker-compose-path := "dev/docker-compose.yml"

# List commands
default:
    just --list

# Set up development environment
[group('setup')]
[group('database')]
bootstrap: && init-env
    # Install dev dependencies of project
    cargo install cargo-watch sqlx-cli git-cliff

# Perform linting with clippy
[group('dev')]
clippy *args:
    cargo  clippy --all-features --all-targets {{args}}
alias lint := clippy

# Check code
[group('dev')]
check *args:
    cargo  check --all-features --all-targets {{args}}
alias ch := check

# Build executable
[group('dev')]
build *args:
    cargo build {{args}}
alias b := build

# Run tests
[group('dev')]
test *args:
    cargo test --all-features --all-targets {{args}}
alias t := test

# Build and run program
run *args:
    cargo run {{args}}
alias r := run

# Run command while watching for changes
[group('dev')]
watch *args='-- just run':
    cargo watch {{args}}
alias w := watch

# Check formatting
[group('dev')]
fmt *args:
    cargo fmt {{args}}
alias f := fmt
alias format := fmt

# Clean build artifacts
[group('dev')]
clean:
    cargo clean
alias cl := clean

# Update Changelog
[group('dev')]
update-changelog *args:
    git-cliff {{args}}
alias chl := update-changelog

# Enable git hooks
[group('setup')]
enable-git-hooks:
    git config core.hooksPath .git-hooks

# Disable git hooks
[group('setup')]
disable-git-hooks:
    git config --unset core.hooksPath

# Start database
[group('database')]
db-start:
    docker compose -f {{docker-compose-path}} up -d  --wait db

# Start database
[group('database')]
db-stop *args:
    docker compose -f {{docker-compose-path}} stop {{args}} db

# Init database
[group('database')]
db-init: db-start && db-migrate
    sqlx database create

# Reset database to original state
[group('database')]
db-reset: && db-init
    docker compose -f {{docker-compose-path}} down -v db

# Run migrations
[group('database')]
db-migrate:
    sqlx migrate run

# Revert last migration
[group('database')]
db-migrate-down:
    sqlx migrate revert

# Connect to database
[group('database')]
db-connect:
    docker compose -f {{docker-compose-path}} exec db psql ${VL__DATABASE_URL}

# Create new migration
[group('database')]
db-migrate-new name:
    sqlx migrate add -r {{name}}

# Prepare sqlx queries
[group('database')]
db-prepare:
    cargo sqlx prepare

# Initializes env and settings.json
[group('setup')]
init-env:
    #!/bin/bash
    set -eu

    if [ -f "{{dotenv-path}}" ] && [ "${FORCE_RESET_ENV:-}" != "1" ]; then
        >&2 echo ".env already exists at {{dotenv-path}}. Use FORCE_RESET_ENV=1 to override."
    else
        POSTGRES_PORT=5432
        POSTGRES_DB=fuel
        POSTGRES_PASSWORD=$(openssl rand -base64 12 | tr "+/" "-_" )
        POSTGRES_USER=$(openssl rand -base64 12 | tr "+/" "-_" )
        DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${POSTGRES_PORT}/${POSTGRES_DB}

        SETTINGS_JSON=$(cat <<EOF
    {
        "rust-analyzer.cargo.extraEnv": {
            "VL__DATABASE_URL": "${DATABASE_URL}"
        },
        "rust-analyzer.runnables.extraEnv": {
            "VL__DATABASE_URL": "${DATABASE_URL}"
        },
        "rust-analyzer.cargo.extraArgs": [
            "--all-features"
        ],
        "rust-analyzer.runnables.extraArgs": [
            "--all-features"
        ]
    }
    
    EOF)

        if [ -f ".vscode/settings.json" ]; then
            echo ".vscode/settings.json already exists; Please add the following snippet:"
            echo -e "${SETTINGS_JSON}"
        else
            echo "${SETTINGS_JSON}" > .vscode/settings.json
        fi
    
        cat <<EOF | tee > {{dotenv-path}}
    POSTGRES_PORT=${POSTGRES_PORT}
    POSTGRES_DB=${POSTGRES_DB}
    POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
    POSTGRES_USER=${POSTGRES_USER}
    DATABASE_URL=${DATABASE_URL}
    VL__DATABASE_URL=${DATABASE_URL}
    EOF
    fi
