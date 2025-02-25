pub(crate) fn factor_id(key: &[String], subkey: &str) -> String {
    if key.is_empty() {
        subkey.to_string()
    } else {
        format!("{}_{}", key.join("_"), subkey)
    }
}