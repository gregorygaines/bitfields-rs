use bitfields::bitflag;

#[bitflag(f32)]
enum Flags {
    #[base]
    Unknown = 0,
    A = 1,
}

fn main() {}

