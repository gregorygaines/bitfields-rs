use bitfields::bitfield;
use bitfields::bitflag;

#[bitfield(u8)]
struct DisplayControl {
    #[bits(4, default = RenderMode::Normal)]
    render_mode: RenderMode,
    #[bits(4, default = AudioMode::Stereo)]
    audio_mode: AudioMode,
}

#[bitflag(u8)]
#[derive(Debug, PartialEq)]
enum RenderMode {
    #[base]
    Normal = 0,
    Mirror = 1,
    Flip = 2,
    Hidden = 3,
}

#[bitflag(u8)]
#[derive(Debug, PartialEq)]
enum AudioMode {
    #[base]
    Stereo = 0,
    Mono = 1,
    Mute = 2,
    Surround = 3,
}

fn main() {
    let display = DisplayControlBuilder::new()
        .with_render_mode(RenderMode::Mirror)
        .with_audio_mode(AudioMode::Mute)
        .build();

    assert_eq!(display.render_mode(), RenderMode::Mirror);
    assert_eq!(display.audio_mode(), AudioMode::Mute);

    let default_display = DisplayControl::new();
    assert_eq!(default_display.render_mode(), RenderMode::Normal);
    assert_eq!(default_display.audio_mode(), AudioMode::Stereo);
}
