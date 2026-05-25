use bitfields::bitflag;

#[bitflag(u8, from_endian = invalid)]
enum Flags {
    #[base]
    Unknown = 0,
    A = 1,
}

fn main() {}

