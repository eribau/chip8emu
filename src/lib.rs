pub struct Chip8 {
    opcode: u8,                     // opcode
    memory: [u8; 4096],             // memory
    V: [u8; 16],                    // Registers V0 to VE plus carry flag
    I: u16,                         // Index register
    pc: u16,                        // Program counter
    gfx: [u8; 2048],                // Graphics, size 64x32
    delay_timer: u8,                // timers
    sound_timer: u8,
    stack: [u16; 16],               // Stack
    sp: u16,                        // Stack pointer
    key: [u8; 16],                  // Keyboard
}

impl Chip8 {
    pub fn new( ) -> Chip8 {
        let mut mem = [0; 4096];
        let fs = Chip8_fontset::new();
        let mut i = 0;
        while i < 80 {
            mem[i] = fs.fontset[i];
            i = i + 1;
        }
        Chip8 {
            opcode: 0,
            memory: mem,
            V: [0; 16],
            I: 0,
            pc: 0x200,
            gfx: [0; 2048],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
        }
    }

    pub fn loadGame(&self, filename: String) {

    }
}

struct Chip8_fontset {
    fontset: [u8; 80],            // Fontset
}

impl Chip8_fontset {
    fn new() -> Chip8_fontset {
        Chip8_fontset {
            fontset: [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80  // F
                ],
            }
    }
}
