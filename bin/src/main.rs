use bin::app::App;
use eframe::egui;
use egui::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1000., 550.)),
        ..Default::default()
    };

    let app = App::new(
        r#"; Source:
; https://skilldrick.github.io/easy6502

LDX #$00
LDY #$00

define STA_ADDR $0200,Y

firstloop:
    TXA
    STA STA_ADDR
    PHA
    INX
    INY
    CPY #$10
    BNE firstloop

secondloop:
    PLA
    STA STA_ADDR
    INY
    CPY #$20
    BNE secondloop
"#,
    );

    eframe::run_native("6502 Emulator", options, Box::new(|_| Box::new(app)))
}
