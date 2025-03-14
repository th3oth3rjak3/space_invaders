use macroquad::prelude::*;

#[macroquad::main("Space Invaders")]
async fn main() {
    loop {
        clear_background(Color::from_hex(0x008080));
        next_frame().await
    }
}
