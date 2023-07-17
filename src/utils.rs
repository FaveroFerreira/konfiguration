pub(crate) fn is_numeric(str: &str) -> bool {
    str.parse::<i64>().is_ok() || str.parse::<f64>().is_ok()
}
