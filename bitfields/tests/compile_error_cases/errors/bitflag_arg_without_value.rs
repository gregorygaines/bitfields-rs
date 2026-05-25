use bitfields::bitflag;

#[bitflag(u8, from_endian)]
enum Flags {
    #[base]
    Unknown = 0,
    A = 1,
}

fn main() {}

