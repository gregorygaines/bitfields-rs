use bitfields::bitflag;

#[bitflag(u8, foo = bar)]
enum Flags {
    #[base]
    Unknown = 0,
    A = 1,
}

fn main() {}

