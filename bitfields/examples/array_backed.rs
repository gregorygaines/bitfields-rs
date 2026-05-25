use bitfields::bitfield;

#[bitfield([u8; 4])]
struct Packet {
    src_port: u16,
    dst_port: u16,
}

fn main() {
    let mut pkt = Packet::new();
    pkt.set_src_port(8080);
    pkt.set_dst_port(443);

    assert_eq!(pkt.src_port(), 8080);
    assert_eq!(pkt.dst_port(), 443);

    let bytes = pkt.into_bytes();
    let restored = Packet::from_bytes(bytes);
    assert_eq!(restored.src_port(), 8080);
    assert_eq!(restored.dst_port(), 443);

    println!("src_port = {}, dst_port = {}", restored.src_port(), restored.dst_port());
}
