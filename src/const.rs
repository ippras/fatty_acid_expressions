use const_format::formatcp;

pub const PREFIX: &str = "FAE";

pub const EXPRESSION: &str = "Expression";

pub const PRIMITIVE: &str = "Primitive";
pub const RATIO: &str = "Ratio";
pub const SUM: &str = "Sum";

pub const BIODIESEL: &str = "Biodiesel";
pub const METABOLIC: &str = "Metabolic";
pub const NUTRITIONAL: &str = "Nutritional";

pub mod sum {
    use super::*;

    pub const SUMS: [&str; 16] = [
        EPA_AND_DHA,
        // By chain length
        SCFA,
        MCFA,
        LCFA,
        VLCFA,
        // By unsaturated bounds
        // By count
        MUFA,
        PUFA,
        SFA,
        UFA,
        // By offset
        D9,
        D12,
        O9,
        O6,
        O3,
        // By parity
        CFA,
        // By pattern
        TFA,
    ];

    pub const CFA: &str = formatcp!("{PREFIX}_{SUM}_Conjugated");
    pub const D12: &str = formatcp!("{PREFIX}_{SUM}_Delta12");
    pub const D9: &str = formatcp!("{PREFIX}_{SUM}_Delta9");
    pub const EPA_AND_DHA: &str = formatcp!("{PREFIX}_{SUM}_EicosapentaenoicAndDocosahexaenoic");
    pub const LCFA: &str = formatcp!("{PREFIX}_{SUM}_LongChain");
    pub const MCFA: &str = formatcp!("{PREFIX}_{SUM}_MediumChain");
    pub const MUFA: &str = formatcp!("{PREFIX}_{SUM}_MonoUnsaturated");
    pub const NUFA: &str = formatcp!("{PREFIX}_{SUM}_NUnsaturated");
    pub const O3: &str = formatcp!("{PREFIX}_{SUM}_Omega-3");
    pub const O6: &str = formatcp!("{PREFIX}_{SUM}_Omega-6");
    pub const O9: &str = formatcp!("{PREFIX}_{SUM}_Omega-9");
    pub const PUFA: &str = formatcp!("{PREFIX}_{SUM}_PolyUnsaturated");
    pub const SCFA: &str = formatcp!("{PREFIX}_{SUM}_ShortChain");
    pub const SFA: &str = formatcp!("{PREFIX}_{SUM}_Saturated");
    pub const TFA: &str = formatcp!("{PREFIX}_{SUM}_Trans");
    pub const UFA: &str = formatcp!("{PREFIX}_{SUM}_Unsaturated");
    pub const VLCFA: &str = formatcp!("{PREFIX}_{SUM}_VeryLongChain");
}

pub mod ratio {
    use super::*;

    pub mod biodiesel {
        use super::*;

        pub const RATIOS: [&str; 6] = [CN, CFPP, DU, IV, LCSF, OS];

        pub const CFPP: &str = formatcp!("{PREFIX}_{RATIO}_{BIODIESEL}_ColdFilterPluggingPoint");
        pub const CN: &str = formatcp!("{PREFIX}_{RATIO}_{BIODIESEL}_CetaneNumber");
        pub const DU: &str = formatcp!("{PREFIX}_{RATIO}_{BIODIESEL}_DegreeOfUnsaturation");
        pub const IV: &str = formatcp!("{PREFIX}_{RATIO}_{BIODIESEL}_IodineValue");
        pub const LCSF: &str = formatcp!("{PREFIX}_{RATIO}_{BIODIESEL}_LongChainSaturatedFactor");
        pub const OS: &str = formatcp!("{PREFIX}_{RATIO}_{BIODIESEL}_OxidationStability");
    }

    pub mod metabolic {
        use super::*;

        pub const RATIOS: [&str; 4] = [D9DI, EI, KAI, TI];

        pub const D9DI: &str = formatcp!("{PREFIX}_{RATIO}_{METABOLIC}_Delta9DesaturaseIndex");
        pub const EI: &str = formatcp!("{PREFIX}_{RATIO}_{METABOLIC}_ElongaseIndex");
        pub const KAI: &str = formatcp!("{PREFIX}_{RATIO}_{METABOLIC}_KineticActivityIndex");
        pub const TI: &str = formatcp!("{PREFIX}_{RATIO}_{METABOLIC}_ThioesteraseIndex");
    }

    pub mod nutritional {
        use super::*;

        pub const RATIOS: [&str; 11] = [
            AI,
            CI,
            FLQ,
            HPI,
            NVI,
            SI,
            TI,
            UI,
            LA_TO_ALA,
            O6PUFA_TO_O3PUFA,
            PUFA_TO_SFA,
        ];

        pub const AI: &str = formatcp!("{PREFIX}_{RATIO}_{NUTRITIONAL}_AtherogenicIndex");
        pub const CI: &str = formatcp!("{PREFIX}_{RATIO}_{NUTRITIONAL}_CholesterolIndex");
        pub const FLQ: &str = formatcp!("{PREFIX}_{RATIO}_{NUTRITIONAL}_FishLipidQuality");
        pub const HPI: &str = formatcp!("{PREFIX}_{RATIO}_{NUTRITIONAL}_HealthPromotingIndex");
        pub const NVI: &str = formatcp!("{PREFIX}_{RATIO}_{NUTRITIONAL}_NutritionalValueIndex");
        pub const SI: &str = formatcp!("{PREFIX}_{RATIO}_{NUTRITIONAL}_SaturationIndex");
        pub const TI: &str = formatcp!("{PREFIX}_{RATIO}_{NUTRITIONAL}_ThrombogenicIndex");
        pub const UI: &str = formatcp!("{PREFIX}_{RATIO}_{NUTRITIONAL}_UnsaturationIndex");

        pub const HH: &str = CI;
        pub const HHI: &str = CI;

        pub const LA_TO_ALA: &str =
            formatcp!("{PREFIX}_{RATIO}_{NUTRITIONAL}_LinoleicToAlphaLinolenic");
        pub const O6PUFA_TO_O3PUFA: &str = formatcp!(
            "{PREFIX}_{RATIO}_{NUTRITIONAL}_Omega-6PolyunsaturatedToOmega-3Polyunsaturated"
        );
        pub const PUFA_TO_SFA: &str =
            formatcp!("{PREFIX}_{RATIO}_{NUTRITIONAL}_PolyunsaturatedToSaturated");
    }
}
