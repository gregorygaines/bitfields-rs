use bitfields::bitflag;

#[bitflag(u8)]
enum Flags {
    #[base]
    Unknown = 0,
    #[base]
    A = 1,
}

fn main() {}

