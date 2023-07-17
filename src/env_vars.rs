use regex::Regex;

use crate::error::{Error, KonfigurationResult};

thread_local! {
    static REGEX: Regex = Regex::new(r#"\$\{([^:]+)(?::([^}]+))?\}"#).unwrap();
}

pub fn expand_env_var(placeholder: &mut str) -> KonfigurationResult<Option<String>> {
    match REGEX.with(|re| re.captures(placeholder)) {
        Some(captures) => {
            let env = captures.get(1).unwrap().as_str();
            let default = captures.get(2).map(|m| m.as_str());

            let val = match std::env::var(env) {
                Ok(val) => val,
                Err(_) => default
                    .ok_or(Error::DefaultMissing {
                        env: env.to_string(),
                    })?
                    .to_string(),
            };

            Ok(Some(val))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_expand_env_var() {
        let mut placeholder = String::from("${TEST_ENV_VAR}");

        std::env::set_var("TEST_ENV_VAR", "test_value");

        let expanded = super::expand_env_var(&mut placeholder).unwrap();

        assert_eq!(expanded, Some("test_value".to_string()));
    }

    #[test]
    fn should_use_default_if_env_not_present() {
        let mut placeholder = String::from("${OTHER_VAR:default_value}");

        let expanded = super::expand_env_var(&mut placeholder).unwrap();

        assert_eq!(expanded, Some("default_value".to_string()));
    }

    #[test]
    fn should_return_error_if_env_not_present_and_no_default() {
        let mut placeholder = String::from("${ANOTHER_VAR}");

        let result = super::expand_env_var(&mut placeholder);

        assert!(result.is_err());
    }
}
