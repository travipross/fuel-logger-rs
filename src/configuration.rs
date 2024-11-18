use crate::{
    error::ApiError,
    types::{configuration::ServerHost, ServerPort},
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, fake::Dummy, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    pub port: ServerPort,
    pub host: ServerHost,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, fake::Dummy, Default)]
#[serde(deny_unknown_fields)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, fake::Dummy, Default)]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    #[serde(default)]
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

pub fn read_config() -> Result<Configuration, ApiError> {
    let config_file_path = std::env::var("CONFIG_FILE").unwrap_or("config.yml".into());
    let raw_config = config::Config::builder()
        .add_source(config::File::with_name(&config_file_path).required(false))
        .add_source(
            config::Environment::with_prefix("VL")
                .separator("_")
                .prefix_separator("__"),
        )
        .build()?;

    Ok(raw_config.try_deserialize::<Configuration>()?)
}

#[cfg(test)]
mod test_utils {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    pub(crate) fn write_config_to_temp_yaml_file(config: &Configuration) -> NamedTempFile {
        let mut tmp_config =
            tempfile::NamedTempFile::with_suffix(".yaml").expect("could not create tempfile");
        let config_yaml_string =
            serde_yaml2::to_string(config).expect("could not build yaml string");
        tmp_config
            .write_all(config_yaml_string.as_bytes())
            .expect("could not write to file");
        tmp_config
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;
    use fake::{Fake, Faker};
    use test_utils::write_config_to_temp_yaml_file;

    #[test]
    fn test_read_from_env() {
        // Arrange
        let port = (1024..49151).fake::<u16>();
        let host = "0.0.0.1";
        let db_url = "psql://user:pass@localhost/fuel:5432";

        temp_env::with_vars(
            [
                ("VL__SERVER_PORT", Some(port.to_string())),
                ("VL__SERVER_HOST", Some(host.to_owned())),
                ("VL__DATABASE_URL", Some(db_url.to_owned())),
            ],
            || {
                // Act
                let config = read_config().expect("could not read config");

                // Assert
                assert_eq!(port, config.server.port);
                assert_eq!(host, config.server.host.as_str());
                assert_eq!(db_url, config.database.url);
            },
        )
    }

    #[test]
    fn test_read_from_yaml() {
        // Arrange
        let config = Faker.fake::<Configuration>();
        let tmp_config = write_config_to_temp_yaml_file(&config);

        temp_env::with_vars(
            [
                ("CONFIG_FILE", Some(tmp_config.path())),
                ("VL__DATABASE_URL", None),
            ],
            || {
                // Act
                let loaded_config = read_config().expect("could not read config");

                // Assert
                assert_eq!(loaded_config, config);
            },
        )
    }

    #[test]
    fn env_takes_precedence_over_file() {
        // Arrange
        let config = Faker.fake::<Configuration>();
        let tmp_config = write_config_to_temp_yaml_file(&config);
        let port_override = (1024..49151).fake::<u16>();
        temp_env::with_vars(
            [
                (
                    "CONFIG_FILE",
                    Some(format!("{}", tmp_config.path().display())),
                ),
                ("VL__SERVER_PORT", Some(port_override.to_string())),
            ],
            || {
                // Act
                let loaded_config = read_config().expect("could not read config");

                // Assert
                assert_ne!(loaded_config, config);
                assert_eq!(loaded_config.server.port, port_override);
            },
        )
    }

    #[test]
    fn ignores_file_if_nonexistent() {
        // Arrange
        let config = Faker.fake::<Configuration>();

        temp_env::with_vars(
            [
                ("CONFIG_FILE", Some(Faker.fake())),
                ("VL__SERVER_PORT", Some(config.server.port.to_string())),
                ("VL__SERVER_HOST", Some(config.server.host.to_string())),
                ("VL__DATABASE_URL", Some(config.database.url.to_string())),
            ],
            || {
                // Act
                let loaded_config = read_config().expect("could not read config");

                // Assert
                assert_eq!(loaded_config, config);
            },
        )
    }

    #[test]
    fn uses_defaults_if_not_set() {
        // Arrange
        let db_url = Faker.fake::<String>();

        temp_env::with_var("VL__DATABASE_URL", Some(db_url.clone()), || {
            // Act
            let loaded_config = read_config().expect("could not read config");

            // Assert
            assert_eq!(
                loaded_config,
                Configuration {
                    database: DatabaseConfig { url: db_url },
                    ..Default::default()
                }
            )
        })
    }

    #[test]
    fn example_config_is_valid() {
        // Arrange
        let config = config::Config::builder()
            .add_source(config::File::with_name("dev/example-config.yml"))
            .build()
            .expect("could not read from file");
        let deserialized = config.try_deserialize::<Configuration>();
        assert!(deserialized.is_ok(), "{deserialized:?}")
    }
}
