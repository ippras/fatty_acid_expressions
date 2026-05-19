pub const FAE: &str = "FAE";
pub const PREFIX: &str = FAE;

pub const PRIMITIVE: &str = "Primitive";
pub const RATIO: &str = "Ratio";
pub const SUM: &str = "Sum";

pub const BIODIESEL: &str = "Biodiesel";
pub const METABOLIC: &str = "Metabolic";
pub const NUTRITIONAL: &str = "Nutritional";

pub mod sum {
    pub const CFA: &str = "Conjugated";
    pub const D12: &str = "Delta12";
    pub const D9: &str = "Delta9";
    pub const EPA_AND_DHA: &str = "EicosapentaenoicAndDocosahexaenoic";
    pub const LCFA: &str = "LongChain";
    pub const MCFA: &str = "MediumChain";
    pub const MUFA: &str = "MonoUnsaturated";
    pub const NUFA: &str = "NUnsaturated";
    pub const O3: &str = "Omega-3";
    pub const O6: &str = "Omega-6";
    pub const O9: &str = "Omega-9";
    pub const PUFA: &str = "PolyUnsaturated";
    pub const SCFA: &str = "ShortChain";
    pub const SFA: &str = "Saturated";
    pub const TFA: &str = "Trans";
    pub const UFA: &str = "Unsaturated";
    pub const VLCFA: &str = "VeryLongChain";
}

pub mod ratio {
    pub mod biodiesel {
        pub const CFPP: &str = "FAE_Biodiesel_ColdFilterPluggingPoint";
        pub const CN: &str = "FAE_Biodiesel_CetaneNumber";
        pub const DU: &str = "FAE_Biodiesel_DegreeOfUnsaturation";
        pub const IV: &str = "FAE_Biodiesel_IodineValue";
        pub const LCSF: &str = "FAE_Biodiesel_LongChainSaturatedFactor";
        pub const OS: &str = "FAE_Biodiesel_OxidationStability";
    }

    pub mod metabolic {
        pub const D9DI: &str = "FAE_Metabolic_Delta9DesaturaseIndex";
        pub const EI: &str = "FAE_Metabolic_ElongaseIndex";
        pub const KAI: &str = "FAE_Metabolic_KineticActivityIndex";
        pub const TI: &str = "FAE_Metabolic_ThioesteraseIndex";
    }

    pub mod nutritional {
        pub const AI: &str = "FAE_Nutritional_AtherogenicIndex";
        pub const CI: &str = "FAE_Nutritional_CholesterolIndex";
        pub const FLQ: &str = "FAE_Nutritional_FishLipidQuality";
        pub const HPI: &str = "FAE_Nutritional_HealthPromotingIndex";
        pub const NVI: &str = "FAE_Nutritional_NutritionalValueIndex";
        pub const SI: &str = "FAE_Nutritional_SaturationIndex";
        pub const TI: &str = "FAE_Nutritional_ThrombogenicIndex";
        pub const UI: &str = "FAE_Nutritional_UnsaturationIndex";

        pub const HH: &str = CI;
        pub const HHI: &str = CI;

        pub const LA_TO_ALA: &str = "FAE_Nutritional_LinoleicToAlphaLinolenic";
        pub const O6PUFA_TO_O3PUFA: &str =
            "FAE_Nutritional_Omega-6PolyunsaturatedToOmega-3Polyunsaturated";
        pub const PUFA_TO_SFA: &str = "FAE_Nutritional_PolyunsaturatedToSaturated";
    }
}
