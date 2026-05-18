#![feature(const_trait_impl)]
#![feature(const_cmp)]

pub const EN: &str = "en";
pub const RU: &str = "ru";

pub const fn sources(language: &str) -> &'static [&'static str] {
    match language {
        // EN => &[
        //     asset!("/ftl/en/main.ftl"),
        //     asset!("/ftl/en/main.selectors.ftl"),
        //     asset!("/ftl/en/aocs.org.ftl"),
        //     asset!("/ftl/en/aocs.org.ext.ftl"),
        // ],
        RU => &[
            // source!("ru", "Ratio", "Biodiesel", "CetaneNumber"),
        ],
        _ => &[
            source!("en", "Ratio", "Biodiesel", "CetaneNumber"),
            source!("en", "Ratio", "Biodiesel", "ColdFilterPluggingPoint"),
            source!("en", "Ratio", "Biodiesel", "DegreeOfUnsaturation"),
            source!("en", "Ratio", "Biodiesel", "IodineValue"),
            source!("en", "Ratio", "Biodiesel", "LongChainSaturatedFactor"),
            source!("en", "Ratio", "Biodiesel", "OxidationStability"),
            source!("en", "Ratio", "Metabolic", "Delta9DesaturaseIndex"),
            source!("en", "Ratio", "Metabolic", "ElongaseIndex"),
            source!("en", "Ratio", "Metabolic", "KineticActivityIndex"),
            source!("en", "Ratio", "Metabolic", "ThioesteraseIndex"),
            source!("en", "Ratio", "Nutritional", "AtherogenicIndex"),
            source!("en", "Ratio", "Nutritional", "FishLipidQuality"),
            source!("en", "Ratio", "Nutritional", "HealthPromotingIndex"),
            source!(
                "en",
                "Ratio",
                "Nutritional",
                "HypocholesterolemicToHypercholesterolemicIndex"
            ),
            source!("en", "Ratio", "Nutritional", "NutritionalValueIndex"),
            source!("en", "Ratio", "Nutritional", "SaturationIndex"),
            source!("en", "Ratio", "Nutritional", "ThrombogenicIndex"),
        ],
    }
}

pub mod r#const;

mod macros;
