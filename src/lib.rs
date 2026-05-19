#![feature(stmt_expr_attributes)]

use icu::locale::subtags::{language, Language};

pub const EN: Language = language!("en");
pub const RU: Language = language!("ru");

pub const fn sources(language: Language) -> &'static [&'static str] {
    match language {
        #[cfg(feature = "en")]
        EN => &[include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ftl/en.ftl"
        ))],
        #[cfg(feature = "ru")]
        RU => &[include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ftl/ru.ftl"
        ))],
        _ => &sources!("en"),
    }
}
// pub const fn sources(language: Language) -> &'static [&'static str] {
//     match language {
//         #[cfg(feature = "en")]
//         EN => &sources!("en"),
//         #[cfg(feature = "ru")]
//         RU => &sources!("ru"),
//         _ => &sources!("en"),
//     }
// }

pub mod r#const;

mod macros;
