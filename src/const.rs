use const_format::formatcp;

pub const PREFIX: &str = "FAE";

pub const EXPRESSION: &str = formatcp!("{PREFIX}_Expression");

pub const PRIMITIVE: &str = formatcp!("{PREFIX}_Primitive");
pub const RATIO: &str = formatcp!("{PREFIX}_Ratio");
pub const SUM: &str = formatcp!("{PREFIX}_Sum");

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

    pub const CFA: &str = formatcp!("{SUM}_Conjugated");
    pub const D12: &str = formatcp!("{SUM}_Delta12");
    pub const D9: &str = formatcp!("{SUM}_Delta9");
    pub const EPA_AND_DHA: &str = formatcp!("{SUM}_EicosapentaenoicAndDocosahexaenoic");
    pub const LCFA: &str = formatcp!("{SUM}_LongChain");
    pub const MCFA: &str = formatcp!("{SUM}_MediumChain");
    pub const MUFA: &str = formatcp!("{SUM}_MonoUnsaturated");
    pub const NUFA: &str = formatcp!("{SUM}_NUnsaturated");
    pub const O3: &str = formatcp!("{SUM}_Omega-3");
    pub const O6: &str = formatcp!("{SUM}_Omega-6");
    pub const O9: &str = formatcp!("{SUM}_Omega-9");
    pub const PUFA: &str = formatcp!("{SUM}_PolyUnsaturated");
    pub const SCFA: &str = formatcp!("{SUM}_ShortChain");
    pub const SFA: &str = formatcp!("{SUM}_Saturated");
    pub const TFA: &str = formatcp!("{SUM}_Trans");
    pub const UFA: &str = formatcp!("{SUM}_Unsaturated");
    pub const VLCFA: &str = formatcp!("{SUM}_VeryLongChain");
}

pub mod ratio {
    use super::*;

    pub const BIODIESEL: &str = formatcp!("{RATIO}_Biodiesel");
    pub const METABOLIC: &str = formatcp!("{RATIO}_Metabolic");
    pub const NUTRITIONAL: &str = formatcp!("{RATIO}_Nutritional");

    pub mod biodiesel {
        use super::*;

        pub const RATIOS: [&str; 6] = [CN, CFPP, DU, IV, LCSF, OS];

        pub const CFPP: &str = formatcp!("{BIODIESEL}_ColdFilterPluggingPoint");
        pub const CN: &str = formatcp!("{BIODIESEL}_CetaneNumber");
        pub const DU: &str = formatcp!("{BIODIESEL}_DegreeOfUnsaturation");
        pub const IV: &str = formatcp!("{BIODIESEL}_IodineValue");
        pub const LCSF: &str = formatcp!("{BIODIESEL}_LongChainSaturatedFactor");
        pub const OS: &str = formatcp!("{BIODIESEL}_OxidationStability");
    }

    pub mod metabolic {
        use super::*;

        pub const RATIOS: [&str; 4] = [D9DI, EI, KAI, TI];

        pub const D9DI: &str = formatcp!("{METABOLIC}_Delta9DesaturaseIndex");
        pub const EI: &str = formatcp!("{METABOLIC}_ElongaseIndex");
        pub const KAI: &str = formatcp!("{METABOLIC}_KineticActivityIndex");
        pub const TI: &str = formatcp!("{METABOLIC}_ThioesteraseIndex");
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

        pub const AI: &str = formatcp!("{NUTRITIONAL}_AtherogenicIndex");
        pub const CI: &str = formatcp!("{NUTRITIONAL}_CholesterolIndex");
        pub const FLQ: &str = formatcp!("{NUTRITIONAL}_FishLipidQuality");
        pub const HPI: &str = formatcp!("{NUTRITIONAL}_HealthPromotingIndex");
        pub const NVI: &str = formatcp!("{NUTRITIONAL}_NutritionalValueIndex");
        pub const SI: &str = formatcp!("{NUTRITIONAL}_SaturationIndex");
        pub const TI: &str = formatcp!("{NUTRITIONAL}_ThrombogenicIndex");
        pub const UI: &str = formatcp!("{NUTRITIONAL}_UnsaturationIndex");

        pub const HH: &str = CI;
        pub const HHI: &str = CI;

        pub const LA_TO_ALA: &str = formatcp!("{NUTRITIONAL}_LinoleicToAlphaLinolenic");
        pub const O6PUFA_TO_O3PUFA: &str =
            formatcp!("{NUTRITIONAL}_Omega-6PolyunsaturatedToOmega-3Polyunsaturated");
        pub const PUFA_TO_SFA: &str = formatcp!("{NUTRITIONAL}_PolyunsaturatedToSaturated");
    }
}
