#[macro_export]
macro_rules! asset {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path))
    };
}

#[macro_export]
macro_rules! source {
    ($language:literal, $expression:literal, $tag:literal, $name:literal) => {
        concat!(
            $expression,
            "_",
            $tag,
            "_",
            $name,
            " = ",
            asset!(concat!(
                $language,
                "/",
                "FattyAcids",
                "/",
                $expression,
                "/",
                $tag,
                "/",
                $name,
                ".md"
            ))
        )
    };
}
