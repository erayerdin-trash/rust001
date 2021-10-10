pub fn get_version<'a>() -> &'a str {
    env!("CARGO_PKG_VERSION")
}
