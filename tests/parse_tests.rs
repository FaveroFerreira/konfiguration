use std::collections::HashMap;

use konfiguration::Konfiguration;

#[derive(Debug, serde::Deserialize)]
pub struct ConfigFileRepresentation {
    s: String,
    s_with_no_default: String,
    s_with_default: String,
    s_with_no_default_env_unset: Option<String>,
    s_with_no_default_env_set: Option<String>,

    i: i32,
    i_with_no_default: i32,
    i_with_default: i32,
    i_with_no_default_env_unset: Option<i32>,
    i_with_no_default_env_set: Option<i32>,

    f: f32,
    f_with_no_default: f32,
    f_with_default: f32,
    f_with_no_default_env_unset: Option<f32>,
    f_with_no_default_env_set: Option<f32>,

    b: bool,
    b_with_no_default: bool,
    b_with_default: bool,
    b_with_no_default_env_unset: Option<bool>,
    b_with_no_default_env_set: Option<bool>,

    array: Vec<i32>,
    array_with_no_default: Vec<i32>,
    array_with_default: Vec<i32>,
    array_with_no_default_env_unset: Option<Vec<i32>>,
    array_with_no_default_env_set: Option<Vec<i32>>,

    hash: HashMap<String, i32>,
    hash_with_no_default: HashMap<String, i32>,
    hash_with_default: HashMap<String, i32>,
    hash_with_no_default_env_unset: Option<HashMap<String, i32>>,
    hash_with_no_default_env_set: Option<HashMap<String, i32>>,

    list: Vec<List>,

    nested: Nested,
    nested_do_not_exist: Option<Nested>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct List {
    s: Option<String>,
    s_with_no_default: Option<String>,
    i: i32,
    f: f32,
    b: bool,
    array: Vec<i32>,
    hash: HashMap<String, i32>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Nested {
    s: String,
    i: i32,
    f: f32,
    b: bool,
    array: Vec<i32>,
    hash: HashMap<String, i32>,
}

#[test]
fn can_parse_configs() {
    std::env::set_var("S_WITH_DEFAULT", "im a string");
    std::env::set_var("I_WITH_DEFAULT", "42");
    std::env::set_var("F_WITH_DEFAULT", "42.42");
    std::env::set_var("B_WITH_DEFAULT", "true");
    std::env::set_var("ARRAY_WITH_DEFAULT", "[1, 2, 3]");
    std::env::set_var("HASH_WITH_DEFAULT", "{ a = 1, b = 2, c = 3 }");

    std::env::set_var("S_WITH_NO_DEFAULT", "im a string");
    std::env::set_var("S_WITH_NO_DEFAULT_ENV_SET", "im a string");
    std::env::set_var("I_WITH_NO_DEFAULT", "42");
    std::env::set_var("I_WITH_NO_DEFAULT_ENV_SET", "42");
    std::env::set_var("F_WITH_NO_DEFAULT", "42.42");
    std::env::set_var("F_WITH_NO_DEFAULT_ENV_SET", "42.42");
    std::env::set_var("B_WITH_NO_DEFAULT", "true");
    std::env::set_var("B_WITH_NO_DEFAULT_ENV_SET", "true");
    std::env::set_var("ARRAY_WITH_NO_DEFAULT", "[1, 2, 3]");
    std::env::set_var("ARRAY_WITH_NO_DEFAULT_ENV_SET", "[1, 2, 3]");
    std::env::set_var("HASH_WITH_NO_DEFAULT", "{ a = 1, b = 2, c = 3 }");
    std::env::set_var("HASH_WITH_NO_DEFAULT_ENV_SET", "{ a = 1, b = 2, c = 3 }");

    let config = Konfiguration::from_file("test_files/config.toml")
        .parse::<ConfigFileRepresentation>()
        .unwrap();

    assert_eq!(config.s, "im a string", "simple string failed");
    assert_eq!(
        config.s_with_no_default, "im a string",
        "string with no default failed"
    );
    assert_eq!(
        config.s_with_default, "im a string",
        "string with default failed"
    );
    assert_eq!(
        config.s_with_no_default_env_unset, None,
        "string with no default env unset failed"
    );
    assert_eq!(
        config.s_with_no_default_env_set,
        Some("im a string".to_string()),
        "string with no default env set failed"
    );

    assert_eq!(config.i, 42, "simple integer failed");
    assert_eq!(
        config.i_with_no_default, 42,
        "integer with no default failed"
    );
    assert_eq!(config.i_with_default, 42, "integer with default failed");
    assert_eq!(
        config.i_with_no_default_env_unset, None,
        "integer with no default env unset failed"
    );
    assert_eq!(
        config.i_with_no_default_env_set,
        Some(42),
        "integer with no default env set failed"
    );

    assert_eq!(config.f, 42.42, "simple float failed");
    assert_eq!(
        config.f_with_no_default, 42.42,
        "float with no default failed"
    );
    assert_eq!(config.f_with_default, 42.42, "float with default failed");
    assert_eq!(
        config.f_with_no_default_env_unset, None,
        "float with no default env unset failed"
    );
    assert_eq!(
        config.f_with_no_default_env_set,
        Some(42.42),
        "float with no default env set failed"
    );

    assert_eq!(config.b, true, "simple boolean failed");
    assert_eq!(
        config.b_with_no_default, true,
        "boolean with no default failed"
    );
    assert_eq!(config.b_with_default, true, "boolean with default failed");
    assert_eq!(
        config.b_with_no_default_env_unset, None,
        "boolean with no default env unset failed"
    );
    assert_eq!(
        config.b_with_no_default_env_set,
        Some(true),
        "boolean with no default env set failed"
    );

    assert_eq!(config.array, vec![1, 2, 3], "simple array failed");
    assert_eq!(
        config.array_with_no_default,
        vec![1, 2, 3],
        "array with no default failed"
    );
    assert_eq!(
        config.array_with_default,
        vec![1, 2, 3],
        "array with default failed"
    );
    assert_eq!(
        config.array_with_no_default_env_unset, None,
        "array with no default env unset failed"
    );
    assert_eq!(
        config.array_with_no_default_env_set,
        Some(vec![1, 2, 3]),
        "array with no default env set failed"
    );

    let mut hash = HashMap::new();
    hash.insert("a".to_string(), 1);
    hash.insert("b".to_string(), 2);
    hash.insert("c".to_string(), 3);

    assert_eq!(config.hash, hash, "simple hash failed");
    assert_eq!(
        config.hash_with_no_default, hash,
        "hash with no default failed"
    );
    assert_eq!(config.hash_with_default, hash, "hash with default failed");
    assert_eq!(
        config.hash_with_no_default_env_unset, None,
        "hash with no default env unset failed"
    );
    assert_eq!(
        config.hash_with_no_default_env_set,
        Some(hash.clone()),
        "hash with no default env set failed"
    );

    assert_eq!(config.nested.s, "im a nested string");
    assert_eq!(config.nested.i, 42, "nested integer failed");
    assert_eq!(config.nested.f, 42.42, "nested float failed");
    assert_eq!(config.nested.b, true, "nested boolean failed");
    assert_eq!(config.nested.array, vec![1, 2, 3], "nested array failed");
    assert_eq!(config.nested.hash, hash, "nested hash failed");
    assert_eq!(
        config.nested_do_not_exist, None,
        "nested do not exist failed"
    )
}
