use bitfields::bitflag;

#[bitflag(u8, into_endian = invalid)]
enum Flags {
    #[base]
    Unknown = 0,
    A = 1,
}

fn main() {}

