#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct AluConfig {
    pub(crate) invert_a: bool,
    pub(crate) invert_b: bool,
    pub(crate) carry_in: bool,
    pub(crate) flood_carry: bool,
    pub(crate) xor_to_or: bool,
    pub(crate) is_rshift: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub(crate) enum AluSettings {
    #[default]
    Add,
    Sub,
    Xor,
    Xnor,
    Or,
    Nor,
    And,
    Nand,
    Implies,
    Nimplies,
    Rshift,
}

impl AluSettings {
    pub(crate) fn config(&self, setting: AluSettings) -> AluConfig {
        match setting {
            AluSettings::Add => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: false,
                xor_to_or: false,
                is_rshift: false,
            },
            AluSettings::Sub => AluConfig {
                invert_a: false,
                invert_b: true,
                carry_in: true,
                flood_carry: false,
                xor_to_or: false,
                is_rshift: false,
            },
            AluSettings::Xor => AluConfig {
                invert_a: false,
                invert_b: true,
                carry_in: false,
                flood_carry: true,
                xor_to_or: false,
                is_rshift: false,
            },
            AluSettings::Xnor => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: true,
                xor_to_or: false,
                is_rshift: false,
            },
            AluSettings::Or => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: false,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::Nor => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: true,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::And => AluConfig {
                invert_a: true,
                invert_b: true,
                carry_in: false,
                flood_carry: true,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::Nand => AluConfig {
                invert_a: true,
                invert_b: true,
                carry_in: false,
                flood_carry: false,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::Implies => AluConfig {
                invert_a: true,
                invert_b: false,
                carry_in: false,
                flood_carry: false,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::Nimplies => AluConfig {
                invert_a: true,
                invert_b: false,
                carry_in: false,
                flood_carry: true,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::Rshift => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: false,
                xor_to_or: false,
                is_rshift: true,
            },
        }
    }
}
