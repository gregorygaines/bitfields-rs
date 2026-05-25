use bitfields::bitflag;

#[bitflag(i32)]
enum Flags {
    #[base]
    Unknown = 0,
    A = 1,
}

fn main() {}

