pub const FAE: &str = "FAE";
pub const PREFIX: &str = FAE;

pub const PRIMITIVE: &str = "Primitive";
pub const RATIO: &str = "Ratio";
pub const SUM: &str = "Sum";

pub const BIODIESEL: &str = "Biodiesel";
pub const METABOLIC: &str = "Metabolic";
pub const NUTRITIONAL: &str = "Nutritional";

pub mod sum {
    pub const CFA: &str = "Sum_Conjugated";
    pub const D12: &str = "Sum_Delta12";
    pub const D9: &str = "Sum_Delta9";
    pub const EPA_AND_DHA: &str = "Sum_EicosapentaenoicAndDocosahexaenoic";
    pub const LCFA: &str = "Sum_LongChain";
    pub const MCFA: &str = "Sum_MediumChain";
    pub const MUFA: &str = "Sum_MonoUnsaturated";
    pub const NUFA: &str = "Sum_NUnsaturated";
    pub const O3: &str = "Sum_Omega-3";
    pub const O6: &str = "Sum_Omega-6";
    pub const O9: &str = "Sum_Omega-9";
    pub const PUFA: &str = "Sum_PolyUnsaturated";
    pub const SCFA: &str = "Sum_ShortChain";
    pub const SFA: &str = "Sum_Saturated";
    pub const TFA: &str = "Sum_Trans";
    pub const UFA: &str = "Sum_Unsaturated";
    pub const VLCFA: &str = "Sum_VeryLongChain";
}

pub mod ratio {
    pub mod biodiesel {
        pub const CFPP: &str = "Ratio_Biodiesel_ColdFilterPluggingPoint";
        pub const CN: &str = "Ratio_Biodiesel_CetaneNumber";
        pub const DU: &str = "Ratio_Biodiesel_DegreeOfUnsaturation";
        pub const IV: &str = "Ratio_Biodiesel_IodineValue";
        pub const LCSF: &str = "Ratio_Biodiesel_LongChainSaturatedFactor";
        pub const OS: &str = "Ratio_Biodiesel_OxidationStability";
    }

    pub mod metabolic {
        pub const D9DI: &str = "Ratio_Metabolic_Delta9DesaturaseIndex";
        pub const EI: &str = "Ratio_Metabolic_ElongaseIndex";
        pub const KAI: &str = "Ratio_Metabolic_KineticActivityIndex";
        pub const TI: &str = "Ratio_Metabolic_ThioesteraseIndex";
    }

    pub mod nutritional {
        pub const AI: &str = "Ratio_Nutritional_AtherogenicIndex";
        pub const CI: &str = "Ratio_Nutritional_CholesterolIndex";
        pub const FLQ: &str = "Ratio_Nutritional_FishLipidQuality";
        pub const HPI: &str = "Ratio_Nutritional_HealthPromotingIndex";
        pub const NVI: &str = "Ratio_Nutritional_NutritionalValueIndex";
        pub const SI: &str = "Ratio_Nutritional_SaturationIndex";
        pub const TI: &str = "Ratio_Nutritional_ThrombogenicIndex";
        pub const UI: &str = "Ratio_Nutritional_UnsaturationIndex";

        pub const HH: &str = CI;
        pub const HHI: &str = CI;

        pub const LA_TO_ALA: &str = "Ratio_Nutritional_LinoleicToAlphaLinolenic";
        pub const O6PUFA_TO_O3PUFA: &str =
            "Nutritional_Omega-6PolyunsaturatedToOmega-3Polyunsaturated";
        pub const PUFA_TO_SFA: &str = "Ratio_Nutritional_PolyunsaturatedToSaturated";
    }
}
