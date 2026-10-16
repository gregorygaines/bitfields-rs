use bitfields::bitflag;

#[bitflag(copy = false)]
enum Flags {
    #[base]
    Zero = 0,
}

fn main() {}
