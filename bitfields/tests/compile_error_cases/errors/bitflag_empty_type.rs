use bitfields::bitflag;

#[bitflag()]
enum Flags {
    #[base]
    Unknown = 0,
    A = 1,
}

fn main() {}

