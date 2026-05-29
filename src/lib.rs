pub mod l10n {
    use egui_l10n::ftl;

    pub const EN: &[&str] = &[ftl!("en/generated.ftl"), ftl!("en/main.ftl")];

    pub const RU: &[&str] = &[ftl!("ru/generated.ftl"), ftl!("ru/main.ftl")];
}

pub mod r#const;
