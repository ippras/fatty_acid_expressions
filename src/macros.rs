#[macro_export]
macro_rules! markdown {
    ($language:literal, $path:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/markdown/",
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
            const_format::formatcp!("{FAE}_c16 = {}", markdown!($language, "Primitive/c16.md")),
            const_format::formatcp!("{FAE}_c18 = {}", markdown!($language, "Primitive/c18.md")),
            const_format::formatcp!("{FAE}_c18c9 = {}", markdown!($language, "Primitive/c18c9.md")),
            const_format::formatcp!("{FAE}_c18c9c12 = {}", markdown!($language, "Primitive/c18c9c12.md")),
            const_format::formatcp!("{FAE}_c18c9c12c15 = {}", markdown!($language, "Primitive/c18c9c12c15.md")),
            const_format::formatcp!("{FAE}_c20 = {}", markdown!($language, "Primitive/c20.md")),
            const_format::formatcp!("{FAE}_c20c5c8c11c14c17 = {}", markdown!($language, "Primitive/c20c5c8c11c14c17.md")),
            const_format::formatcp!("{FAE}_c22 = {}", markdown!($language, "Primitive/c22.md")),
            const_format::formatcp!("{FAE}_c22c4c7c10c13c16c19 = {}", markdown!($language, "Primitive/c22c4c7c10c13c16c19.md")),
            const_format::formatcp!("{FAE}_c22c13 = {}", markdown!($language, "Primitive/c22c13.md")),
            const_format::formatcp!("{FAE}_c24 = {}", markdown!($language, "Primitive/c24.md")),
            const_format::formatcp!("{FAE}_c24c15 = {}", markdown!($language, "Primitive/c24c15.md")),
            // Sum
            const_format::formatcp!("{FAE}_{} = {}", sum::LCFA, markdown!($language, "Sum/ByChainLength/Long.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::MCFA, markdown!($language, "Sum/ByChainLength/Medium.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::SCFA, markdown!($language, "Sum/ByChainLength/Short.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::VLCFA, markdown!($language, "Sum/ByChainLength/VeryLong.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::MUFA, markdown!($language, "Sum/ByUnsaturatedBounds/ByCount/MonoUnsaturated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::NUFA, markdown!($language, "Sum/ByUnsaturatedBounds/ByCount/NUnsaturated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::PUFA, markdown!($language, "Sum/ByUnsaturatedBounds/ByCount/PolyUnsaturated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::SFA, markdown!($language, "Sum/ByUnsaturatedBounds/ByCount/Saturated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::UFA, markdown!($language, "Sum/ByUnsaturatedBounds/ByCount/Unsaturated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::D12, markdown!($language, "Sum/ByUnsaturatedBounds/ByOffset/Delta12.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::D9, markdown!($language, "Sum/ByUnsaturatedBounds/ByOffset/Delta9.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::O3, markdown!($language, "Sum/ByUnsaturatedBounds/ByOffset/Omega-3.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::O6, markdown!($language, "Sum/ByUnsaturatedBounds/ByOffset/Omega-6.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::O9, markdown!($language, "Sum/ByUnsaturatedBounds/ByOffset/Omega-9.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::TFA, markdown!($language, "Sum/ByUnsaturatedBounds/ByParity/Trans.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::CFA, markdown!($language, "Sum/ByUnsaturatedBounds/ByPattern/Conjugated.md")),
            const_format::formatcp!("{FAE}_{} = {}", sum::EPA_AND_DHA, markdown!($language, "Sum/EicosapentaenoicAndDocosahexaenoic.md")),
            // Ratio
            // Biodiesel
            const_format::formatcp!("{FAE}_{BIODIESEL}_CetaneNumber = CetaneNumber1"),
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::CN, markdown!($language, "Ratio/Biodiesel/CetaneNumber.md")),
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::CFPP, markdown!($language, "Ratio/Biodiesel/ColdFilterPluggingPoint.md")),
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::DU, markdown!($language, "Ratio/Biodiesel/DegreeOfUnsaturation.md")),
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::IV, markdown!($language, "Ratio/Biodiesel/IodineValue.md")),
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::LCSF, markdown!($language, "Ratio/Biodiesel/LongChainSaturatedFactor.md")),
            const_format::formatcp!("{FAE}_{BIODIESEL}_{} = {}", biodiesel::OS, markdown!($language, "Ratio/Biodiesel/OxidationStability.md")),
            // Metabolic
            const_format::formatcp!("{FAE}_{METABOLIC}_{} = {}", metabolic::D9DI, markdown!($language, "Ratio/Metabolic/Delta9DesaturaseIndex.md")),
            const_format::formatcp!("{FAE}_{METABOLIC}_{} = {}", metabolic::EI, markdown!($language, "Ratio/Metabolic/ElongaseIndex.md")),
            const_format::formatcp!("{FAE}_{METABOLIC}_{} = {}", metabolic::KAI, markdown!($language, "Ratio/Metabolic/KineticActivityIndex.md")),
            const_format::formatcp!("{FAE}_{METABOLIC}_{} = {}", metabolic::TI, markdown!($language, "Ratio/Metabolic/ThioesteraseIndex.md")),
            // Nutritional
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::LA_TO_ALA, markdown!($language, "Ratio/Nutritional/LinoleicToAlphaLinolenic.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::PUFA_TO_SFA, markdown!($language, "Ratio/Nutritional/PolyunsaturatedToSaturated.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::PUFAO6_TO_PUFAO3, markdown!($language, "Ratio/Nutritional/PolyunsaturatedToSaturated.md")),

            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::AI, markdown!($language, "Ratio/Nutritional/AtherogenicIndex.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::CI, markdown!($language, "Ratio/Nutritional/CholesterolIndex.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::FLQ, markdown!($language, "Ratio/Nutritional/FishLipidQuality.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::HPI, markdown!($language, "Ratio/Nutritional/HealthPromotingIndex.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::NVI, markdown!($language, "Ratio/Nutritional/NutritionalValueIndex.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::SI, markdown!($language, "Ratio/Nutritional/SaturationIndex.md")),
            const_format::formatcp!("{FAE}_{NUTRITIONAL}_{} = {}", nutritional::TI, markdown!($language, "Ratio/Nutritional/ThrombogenicIndex.md"))
        ]
    }};
}
