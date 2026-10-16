use bitfields::bitflag;

#[bitflag]
#[repr(C)]
enum Flags {
    #[base]
    Zero = 0,
}

fn main() {}
