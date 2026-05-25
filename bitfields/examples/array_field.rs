use bitfields::bitfield;

fn main() {
    #[bitfield(u32)]
    pub struct Ipv4Address {
        octets: [u8; 4],
    }

    let mut addr = Ipv4Address::new();
    addr.set_octets([192, 168, 1, 1]);
    assert_eq!(addr.octets(), [192, 168, 1, 1]);
    println!("IPv4 address octets: {:?}", addr.octets());
    println!("Raw bits: {:#010X}", addr.into_bits());

    #[bitfield(u64)]
    pub struct NetworkPacket {
        src_port: u16,
        dst_port: u16,
        payload: [u8; 4],
    }

    let mut pkt = NetworkPacket::new();
    pkt.set_src_port(8080);
    pkt.set_dst_port(443);
    pkt.set_payload([0xDE, 0xAD, 0xBE, 0xEF]);

    assert_eq!(pkt.src_port(), 8080);
    assert_eq!(pkt.dst_port(), 443);
    assert_eq!(pkt.payload(), [0xDE, 0xAD, 0xBE, 0xEF]);
    println!(
        "src_port={}, dst_port={}, payload={:?}",
        pkt.src_port(),
        pkt.dst_port(),
        pkt.payload()
    );
}
