use const_format::formatcp;

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
    use super::*;

    pub mod biodiesel {
        use super::*;

        pub const CFPP: &str = "ColdFilterPluggingPoint";
        pub const CN: &str = "CetaneNumber";
        pub const DU: &str = "DegreeOfUnsaturation";
        pub const IV: &str = "IodineValue";
        pub const LCSF: &str = "LongChainSaturatedFactor";
        pub const OS: &str = "OxidationStability";

        pub const CETANE_NUMBER: &str = formatcp!("{FAE}_{BIODIESEL}_{CN}");
        pub const COLD_FILTER_PLUGGING_POINT: &str = formatcp!("{FAE}_{BIODIESEL}_{CFPP}");
        pub const DEGREE_OF_UNSATURATION: &str = formatcp!("{FAE}_{BIODIESEL}_{DU}");
        pub const IODINE_VALUE: &str = formatcp!("{FAE}_{BIODIESEL}_{IV}");
        pub const LONG_CHAIN_SATURATED_FACTOR: &str = formatcp!("{FAE}_{BIODIESEL}_{LCSF}");
        pub const OXIDATION_STABILITY: &str = formatcp!("{FAE}_{BIODIESEL}_{OS}");
    }

    pub mod metabolic {
        pub const D9DI: &str = "Delta9DesaturaseIndex";
        pub const EI: &str = "ElongaseIndex";
        pub const KAI: &str = "KineticActivityIndex";
        pub const TI: &str = "ThioesteraseIndex";
    }

    pub mod nutritional {
        pub const AI: &str = "AtherogenicIndex";
        pub const CI: &str = "CholesterolIndex";
        pub const FLQ: &str = "FishLipidQuality";
        pub const HPI: &str = "HealthPromotingIndex";
        pub const NVI: &str = "NutritionalValueIndex";
        pub const SI: &str = "SaturationIndex";
        pub const TI: &str = "ThrombogenicIndex";

        pub const HH: &str = CI;
        pub const HHI: &str = CI;

        pub const LA_TO_ALA: &str = "LinoleicToAlphaLinolenic";
        pub const O6PUFA_TO_O3PUFA: &str = "Omega-6PolyunsaturatedToOmega-3Polyunsaturated";
        pub const PUFA_TO_SFA: &str = "PolyunsaturatedToSaturated";
    }
}
