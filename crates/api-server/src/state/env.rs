use tracing::{trace, warn};


pub fn extract_variable(variable: &str, default: &str) -> String {
    let fallback = || -> String {
        trace!(
            var = variable,
            value = default,
            "[ENV] using default"
        );
        default.to_owned()
    };

    match std::env::var(variable) {
        Ok(value) => {
            if value.trim().is_empty() {
                fallback()
            } else {
                value
            }
        }
        Err(e) => match e {
            std::env::VarError::NotPresent => {
                warn!(variable = variable, "[ENV] Not present");
                fallback()
            }
            std::env::VarError::NotUnicode(_) => {
                warn!(variable = variable, "[ENV] Not unicode");
                fallback()
            }
        },
    }
}
