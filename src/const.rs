pub const FAE: &str = "FAE";
pub const PREFIX: &str = FAE;

// pub const PRIMITIVE: &str = "Primitive";
// pub const RATIO: &str = "Ratio";
// pub const SUM: &str = "Sum";

// pub const BIODIESEL: &str = "Biodiesel";
// pub const METABOLIC: &str = "Metabolic";
// pub const NUTRITIONAL: &str = "Nutritional";

pub const SUM: [&str; 17] = [
    sum::CFA,
    sum::D12,
    sum::D9,
    sum::EPA_AND_DHA,
    sum::LCFA,
    sum::MCFA,
    sum::MUFA,
    sum::NUFA,
    sum::O3,
    sum::O6,
    sum::O9,
    sum::PUFA,
    sum::SCFA,
    sum::SFA,
    sum::TFA,
    sum::UFA,
    sum::VLCFA,
];

pub mod sum {
    pub const CFA: &str = "FAE_Sum_Conjugated";
    pub const D12: &str = "FAE_Sum_Delta12";
    pub const D9: &str = "FAE_Sum_Delta9";
    pub const EPA_AND_DHA: &str = "FAE_Sum_EicosapentaenoicAndDocosahexaenoic";
    pub const LCFA: &str = "FAE_Sum_LongChain";
    pub const MCFA: &str = "FAE_Sum_MediumChain";
    pub const MUFA: &str = "FAE_Sum_MonoUnsaturated";
    pub const NUFA: &str = "FAE_Sum_NUnsaturated";
    pub const O3: &str = "FAE_Sum_Omega-3";
    pub const O6: &str = "FAE_Sum_Omega-6";
    pub const O9: &str = "FAE_Sum_Omega-9";
    pub const PUFA: &str = "FAE_Sum_PolyUnsaturated";
    pub const SCFA: &str = "FAE_Sum_ShortChain";
    pub const SFA: &str = "FAE_Sum_Saturated";
    pub const TFA: &str = "FAE_Sum_Trans";
    pub const UFA: &str = "FAE_Sum_Unsaturated";
    pub const VLCFA: &str = "FAE_Sum_VeryLongChain";
}

pub mod ratio {
    pub const BIODIESEL: [&str; 6] = [
        biodiesel::CFPP,
        biodiesel::CN,
        biodiesel::DU,
        biodiesel::IV,
        biodiesel::LCSF,
        biodiesel::OS,
    ];

    pub const METABOLIC: [&str; 4] = [
        metabolic::D9DI,
        metabolic::EI,
        metabolic::KAI,
        metabolic::TI,
    ];

    pub const NUTRITIONAL: [&str; 11] = [
        nutritional::AI,
        nutritional::CI,
        nutritional::FLQ,
        nutritional::HPI,
        nutritional::NVI,
        nutritional::SI,
        nutritional::TI,
        nutritional::UI,

        nutritional::LA_TO_ALA,
        nutritional::O6PUFA_TO_O3PUFA,
        nutritional::PUFA_TO_SFA,
    ];

    pub mod biodiesel {
        pub const CFPP: &str = "FAE_Ratio_Biodiesel_ColdFilterPluggingPoint";
        pub const CN: &str = "FAE_Ratio_Biodiesel_CetaneNumber";
        pub const DU: &str = "FAE_Ratio_Biodiesel_DegreeOfUnsaturation";
        pub const IV: &str = "FAE_Ratio_Biodiesel_IodineValue";
        pub const LCSF: &str = "FAE_Ratio_Biodiesel_LongChainSaturatedFactor";
        pub const OS: &str = "FAE_Ratio_Biodiesel_OxidationStability";
    }

    pub mod metabolic {
        pub const D9DI: &str = "FAE_Ratio_Metabolic_Delta9DesaturaseIndex";
        pub const EI: &str = "FAE_Ratio_Metabolic_ElongaseIndex";
        pub const KAI: &str = "FAE_Ratio_Metabolic_KineticActivityIndex";
        pub const TI: &str = "FAE_Ratio_Metabolic_ThioesteraseIndex";
    }

    pub mod nutritional {
        pub const AI: &str = "FAE_Ratio_Nutritional_AtherogenicIndex";
        pub const CI: &str = "FAE_Ratio_Nutritional_CholesterolIndex";
        pub const FLQ: &str = "FAE_Ratio_Nutritional_FishLipidQuality";
        pub const HPI: &str = "FAE_Ratio_Nutritional_HealthPromotingIndex";
        pub const NVI: &str = "FAE_Ratio_Nutritional_NutritionalValueIndex";
        pub const SI: &str = "FAE_Ratio_Nutritional_SaturationIndex";
        pub const TI: &str = "FAE_Ratio_Nutritional_ThrombogenicIndex";
        pub const UI: &str = "FAE_Ratio_Nutritional_UnsaturationIndex";

        pub const HH: &str = CI;
        pub const HHI: &str = CI;

        pub const LA_TO_ALA: &str = "FAE_Ratio_Nutritional_LinoleicToAlphaLinolenic";
        pub const O6PUFA_TO_O3PUFA: &str =
            "FAE_Ratio_Nutritional_Omega-6PolyunsaturatedToOmega-3Polyunsaturated";
        pub const PUFA_TO_SFA: &str = "FAE_Ratio_Nutritional_PolyunsaturatedToSaturated";
    }
}
