use bitfields::bitflag;

#[bitflag]
#[repr(usize)]
enum Flags {
    #[base]
    Zero = 0,
}

fn main() {}
