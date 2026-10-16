use bitfields::bitflag;

#[bitflag(u8)]
#[repr(u16)]
enum Flags {
    #[base]
    Zero = 0,
}

fn main() {}
