#[macro_export]
macro_rules! ftl {
    ($path:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/ftl/", $path))
    };
    ($language:literal, $path:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ftl/",
            $language,
            "/",
            $path
        ))
    };
}
