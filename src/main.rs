extern crate chip8emu;

use chip8emu::Chip8;

fn main() {
    // Setup graphics
    // Setup input

    // Initialize the Chip8 system and load the game into memory
    let mut chip8 = Chip8::new();
    chip8.loadGame("PONG".to_string());
    chip8.printMemoryLoc(512);


    // Game loop
    loop {
        // Emulate one cycle
        //emulateCycle();

        // If the draw flag is set, update the screen
        //if drawFlag() {
        //    drawGraphics();
        //}

        // Store key press state (Press and Release)
        //setKeys();
    }
}
