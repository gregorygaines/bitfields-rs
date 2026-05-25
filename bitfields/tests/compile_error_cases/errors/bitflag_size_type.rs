use bitfields::bitflag;

#[bitflag(usize)]
enum Flags {
    #[base]
    Unknown = 0,
    A = 1,
}

fn main() {}

