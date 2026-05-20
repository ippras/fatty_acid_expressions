use icu::locale::subtags::{Language, language};

pub const EN: Language = language!("en");
pub const RU: Language = language!("ru");

pub const fn sources(language: Language) -> &'static [&'static str] {
    match language {
        #[cfg(feature = "en")]
        EN => &[ftl!("en.ftl")],
        #[cfg(feature = "ru")]
        RU => &[ftl!("ru.ftl")],
        _ => &[ftl!("en.ftl")],
    }
}

pub mod r#const;

mod macros;
