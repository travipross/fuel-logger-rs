# vehicle-logbook-rs
Vehicle Logbook API written in Rust, built with [axum](https://github.com/tokio-rs/axum).


## Getting Started
### Install Dev Dependencies and Initialize Project
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Just](https://github.com/casey/just)
- Run `just bootstrap` to initialize project and install other dev dependencies

### Build and Run
After installing dev dependencies and initializing the project, run the following to build and execute the app:

```bash
just r
```
See other helpful dev commands by inspecting the [`justfile`](./justfile) or by running `just` with no other arguments.

### Alternative - Dev Container
For convenience, a DevContainer configuration has been provided. Run in VSCode using the [Dev Containers Extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers).

On first launch, dev dependencies will automatically be installed via `just bootstrap` as described above.

See [here](https://code.visualstudio.com/docs/devcontainers/containers) for more information on configuring Dev Containers in VSCode.

## Configuration
Initialize a configuration file using the following command (substituting your own values for any tokens wrapped in `{{}}`):
```
just r -- --init-config > {{your-yaml-config-path}}
```

Run the server using this configuration file by setting the appropriate environment variable to point at this file:
```
export CONFIG_FILE={{your-yaml-config-path}}
just r
```

Note that any configuration parameter understood by the config file can be overridden by environment variables using the prefix `VL__`. Structured config heirarchy is denoted using `_`. For example:

- `server.port` can be overridden by setting `VL__SERVER_PORT`
- `database.url` can be overridden by setting `VL__DATABASE_URL`

## Changelog & Commits
Changelog generation is performed via [`git-cliff`](https://git-cliff.org/docs/), by parsing conventional commit messages.

Please ensure commits follow the [conventional commit format](https://www.conventionalcommits.org/en/v1.0.0/) in order for them to be included in changelogs.
