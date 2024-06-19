# List commands
default:
    just --list

# Set up development environment
bootstrap:
    cargo install cargo-watch

# Perform linting with clippy
clippy *args:
    cargo clippy {{args}}

# Check code
check *args:
    cargo check {{args}}

# Build executable
build *args:
    cargo build {{args}}

# Run tests
test *args:
    cargo test {{args}}

# Build and run program
run *args:
    cargo run {{args}}

# Run command while watching for changes
watch *args='-- just run':
    cargo watch {{args}}

# Check formatting
fmt *args:
    cargo fmt {{args}}

# Clean build artifacts
clean:
    cargo clean

# Aliases
alias b := build
alias t := test
alias r := run
alias ch := check
alias lint := check
alias cl := clean
alias f := fmt
alias format := fmt