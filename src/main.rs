extern crate chip8emu;

use chip8emu::Chip8;

fn main() {
    // Setup graphics
    // Setup input

    // Initialize the Chip8 system and load the game into memory
    let mut chip8 = Chip8::new();
    //loadGame();
    

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
