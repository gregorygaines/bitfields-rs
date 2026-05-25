use bitfields::bitfield;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    /// Convert raw bits into a `Color`.
    const fn from_bits(bits: u8) -> Self {
        Self {
            red: bits & 0b0000_0111,
            green: (bits >> 3) & 0b0000_0111,
            blue: (bits >> 6) & 0b0000_0011,
        }
    }

    /// Pack a `Color` back into bits.
    const fn into_bits(self) -> u8 {
        (self.red & 0b0000_0111)
            | ((self.green & 0b0000_0111) << 3)
            | ((self.blue & 0b0000_0011) << 6)
    }
}

const DEFAULT_COLOR: Color = Color {
    red: 5,
    green: 3,
    blue: 1,
};

#[bitfield(u16)]
struct FrameConfig {
    #[bits(8, default = DEFAULT_COLOR)]
    background: Color,

    #[bits(8)]
    foreground: Color,
}

fn main() {
    let cfg = FrameConfig::default();

    assert_eq!(cfg.background(), DEFAULT_COLOR);
    assert_eq!(
        cfg.foreground(),
        Color {
            red: 0,
            green: 0,
            blue: 0
        }
    );
}
