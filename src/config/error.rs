// Config loading failures stay separate from process runtime failures.
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    InvalidYaml(String),
    InvalidConfig(String),
    ReadFailed(String),
}
