use bitfields::bitflag;

#[bitflag(u8)]
enum Flags {
    #[base]
    Unknown = 0,
    A,
}

fn main() {}

