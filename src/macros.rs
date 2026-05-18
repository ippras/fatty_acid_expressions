#[macro_export]
macro_rules! asset {
    ($language:literal, $path:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/",
            $language,
            "/",
            $path
        ))
    };
}

#[macro_export]
macro_rules! sources {
    ($language:literal) => {{
        use crate::r#const::{
            BIODIESEL, FAE, METABOLIC, NUTRITIONAL,
            ratio::{biodiesel, metabolic, nutritional},
            sum,
        };
        #[rustfmt::skip]
        [
            // Primitive
            const_format::formatcp!("{FAE}_c16 = {}", asset!($language, "Primitive/c16.md")),
            const_format::formatcp!("{FAE}_c18 = {}", asset!($language, "Primitive/c18.md")),
            const_format::formatcp!("{FAE}_c18c9 = {}", asset!($language, "Primitive/c18c9.md")),
            const_format::formatcp!("{FAE}_c18c9c12 = {}", asset!($language, "Primitive/c18c9c12.md")),
            const_format::formatcp!("{FAE}_c18c9c12c15 = {}", asset!($language, "Primitive/c18c9c12c15.md")),
            const_format::formatcp!("{FAE}_c20 = {}", asset!($language, "Primitive/c20.md")),
            const_format::formatcp!("{FAE}_c20c5c8c11c14c17 = {}", asset!($language, "Primitive/c20c5c8c11c14c17.md")),
            const_format::formatcp!("{FAE}_c22 = {}", asset!($language, "Primitive/c22.md")),
            const_format::formatcp!("{FAE}_c22c4c7c10c13c16c19 = {}", asset!($language, "Primitive/c22c4c7c10c13c16c19.md")),
            const_format::formatcp!("{FAE}_c22c13 = {}", asset!($language, "Primitive/c22c13.md")),
            const_format::formatcp!("{FAE}_c24 = {}", asset!($language, "Primitive/c24.md")),
            const_format::formatcp!("{FAE}_c24c15 = {}", asset!($language, "Primitive/c24c15.md")),
            // Sum
            const_format::formatcp!("{FAE}_{} = {}", sum::LCFA, asset!($language, "Sum/ByChainLength/Long.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::MCFA, asset!($language, "Sum/ByChainLength/Medium.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::SCFA, asset!($language, "Sum/ByChainLength/Short.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::VLCFA, asset!($language, "Sum/ByChainLength/VeryLong.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::MUFA, asset!($language, "Sum/ByUnsaturatedBounds/ByCount/MonoUnsaturated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::NUFA, asset!($language, "Sum/ByUnsaturatedBounds/ByCount/NUnsaturated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::PUFA, asset!($language, "Sum/ByUnsaturatedBounds/ByCount/PolyUnsaturated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::SFA, asset!($language, "Sum/ByUnsaturatedBounds/ByCount/Saturated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::UFA, asset!($language, "Sum/ByUnsaturatedBounds/ByCount/Unsaturated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::D12, asset!($language, "Sum/ByUnsaturatedBounds/ByOffset/Delta12.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::D9, asset!($language, "Sum/ByUnsaturatedBounds/ByOffset/Delta9.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::O3, asset!($language, "Sum/ByUnsaturatedBounds/ByOffset/Omega-3.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::O6, asset!($language, "Sum/ByUnsaturatedBounds/ByOffset/Omega-6.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::O9, asset!($language, "Sum/ByUnsaturatedBounds/ByOffset/Omega-9.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::TFA, asset!($language, "Sum/ByUnsaturatedBounds/ByParity/Trans.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::CFA, asset!($language, "Sum/ByUnsaturatedBounds/ByPattern/Conjugated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::EPA_AND_DHA, asset!($language, "Sum/EicosapentaenoicAndDocosahexaenoic.md")),
            // Ratio
            // Biodiesel
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::CN, asset!($language, "Ratio/Biodiesel/CetaneNumber.md")),
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::CFPP, asset!($language, "Ratio/Biodiesel/ColdFilterPluggingPoint.md")),
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::DU, asset!($language, "Ratio/Biodiesel/DegreeOfUnsaturation.md")),
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::IV, asset!($language, "Ratio/Biodiesel/IodineValue.md")),
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::LCSF, asset!($language, "Ratio/Biodiesel/LongChainSaturatedFactor.md")),
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::OS, asset!($language, "Ratio/Biodiesel/OxidationStability.md")),
            // Metabolic
            const_format::formatcp!("{FAE}_{METABOLIC}_{} = {}", metabolic::D9DI, asset!($language, "Ratio/Metabolic/Delta9DesaturaseIndex.md")),
            const_format::formatcp!("{FAE}_{METABOLIC}_{} = {}", metabolic::EI, asset!($language, "Ratio/Metabolic/ElongaseIndex.md")),
            const_format::formatcp!("{FAE}_{METABOLIC}_{} = {}", metabolic::KAI, asset!($language, "Ratio/Metabolic/KineticActivityIndex.md")),
            const_format::formatcp!("{FAE}_{METABOLIC}_{} = {}", metabolic::TI, asset!($language, "Ratio/Metabolic/ThioesteraseIndex.md")),
            // Nutritional
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::LA_TO_ALA, asset!($language, "Ratio/Nutritional/LinoleicToAlphaLinolenic.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::PUFA_TO_SFA, asset!($language, "Ratio/Nutritional/PolyunsaturatedToSaturated.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::PUFAO6_TO_PUFAO3, asset!($language, "Ratio/Nutritional/PolyunsaturatedToSaturated.md")),

            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::AI, asset!($language, "Ratio/Nutritional/AtherogenicIndex.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::CI, asset!($language, "Ratio/Nutritional/CholesterolIndex.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::FLQ, asset!($language, "Ratio/Nutritional/FishLipidQuality.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::HPI, asset!($language, "Ratio/Nutritional/HealthPromotingIndex.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::NVI, asset!($language, "Ratio/Nutritional/NutritionalValueIndex.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::SI, asset!($language, "Ratio/Nutritional/SaturationIndex.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::TI, asset!($language, "Ratio/Nutritional/ThrombogenicIndex.md"))
        ]
    }};
}
