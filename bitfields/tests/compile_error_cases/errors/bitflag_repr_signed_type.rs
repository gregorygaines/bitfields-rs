use bitfields::bitflag;

#[bitflag]
#[repr(i8)]
enum Flags {
    #[base]
    Zero = 0,
}

fn main() {}
