/// Simple 3x5 font for a-z, ., !, ?, and space
#[allow(dead_code)]
const CHAR_FONT: phf::Map<char, [u8; 15]> = phf::phf_map! {
    'a' => [0,1,0, 1,0,1, 1,1,1, 1,0,1, 1,0,1],
    'b' => [1,1,0, 1,0,1, 1,1,0, 1,0,1, 1,1,0],
    'c' => [0,1,1, 1,0,0, 1,0,0, 1,0,0, 0,1,1],
    'd' => [1,1,0, 1,0,1, 1,0,1, 1,0,1, 1,1,0],
    'e' => [1,1,1, 1,0,0, 1,1,1, 1,0,0, 1,1,1],
    'f' => [1,1,1, 1,0,0, 1,1,1, 1,0,0, 1,0,0],
    'g' => [0,1,1, 1,0,0, 1,0,1, 1,0,1, 0,1,1],
    'h' => [1,0,1, 1,0,1, 1,1,1, 1,0,1, 1,0,1],
    'i' => [1,1,1, 0,1,0, 0,1,0, 0,1,0, 1,1,1],
    'j' => [0,0,1, 0,0,1, 0,0,1, 1,0,1, 0,1,0],
    'k' => [1,0,1, 1,0,1, 1,1,0, 1,0,1, 1,0,1],
    'l' => [1,0,0, 1,0,0, 1,0,0, 1,0,0, 1,1,1],
    'm' => [1,0,1, 1,1,1, 1,0,1, 1,0,1, 1,0,1],
    'n' => [1,0,1, 1,1,1, 1,1,1, 1,0,1, 1,0,1],
    'o' => [0,1,0, 1,0,1, 1,0,1, 1,0,1, 0,1,0],
    'p' => [1,1,0, 1,0,1, 1,1,0, 1,0,0, 1,0,0],
    'q' => [0,1,0, 1,0,1, 1,0,1, 1,1,1, 0,1,1],
    'r' => [1,1,0, 1,0,1, 1,1,0, 1,0,1, 1,0,1],
    's' => [0,1,1, 1,0,0, 0,1,0, 0,0,1, 1,1,0],
    't' => [1,1,1, 0,1,0, 0,1,0, 0,1,0, 0,1,0],
    'u' => [1,0,1, 1,0,1, 1,0,1, 1,0,1, 0,1,0],
    'v' => [1,0,1, 1,0,1, 1,0,1, 0,1,0, 0,1,0],
    'w' => [1,0,1, 1,0,1, 1,0,1, 1,1,1, 1,0,1],
    'x' => [1,0,1, 0,1,0, 0,1,0, 0,1,0, 1,0,1],
    'y' => [1,0,1, 1,0,1, 0,1,0, 0,1,0, 0,1,0],
    'z' => [1,1,1, 0,0,1, 0,1,0, 1,0,0, 1,1,1],
    '.' => [0,0,0, 0,0,0, 0,0,0, 0,1,0, 0,1,0],
    '!' => [0,1,0, 0,1,0, 0,1,0, 0,0,0, 0,1,0],
    '?' => [1,1,1, 0,0,1, 0,1,1, 0,0,0, 0,1,0],
    ' ' => [0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0],
};
use minifb::{Key, Window, WindowOptions};
use rust_vm::io_devices::screen::Screen;

const PIXEL_SIZE: usize = 16;
const GRID_WIDTH: usize = 32;
const GRID_HEIGHT: usize = 32;

const DISPLAY_WIDTH: usize = GRID_WIDTH * PIXEL_SIZE; // 16*32=480
const DISPLAY_HEIGHT: usize = GRID_HEIGHT * PIXEL_SIZE; // 16*32=480
const HUD_HEIGHT: usize = DISPLAY_HEIGHT / 8; // 480/8 = 60

const WINDOW_WIDTH: usize = DISPLAY_WIDTH;
const WINDOW_HEIGHT: usize = DISPLAY_HEIGHT + HUD_HEIGHT;

const TICKS_PER_FRAME: usize = 150; // 9000 instructions per second
fn main() {
    let mut vm = rust_vm::VM::new();

    // completed programs:
    // vm.load_program("programs/dvd.as").unwrap();
    // vm.load_programs("programs/gol.as").unwrap();
    // vm.load_program("programs/maze.as").unwrap();

    //working, but waiting for character display:
    // vm.load_program("programs/helloworld.as").unwrap();

    // working screen programs:
    // vm.load_program("programs/calculator.as").unwrap();
    // vm.load_program("programs/2048.as").unwrap();
    // vm.load_program("programs/connect4.as").unwrap();

    vm.load_program("programs/maze.as").unwrap();

    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let width = WINDOW_WIDTH;
    let height = WINDOW_HEIGHT;

    let window_options = WindowOptions {
        resize: true,
        scale_mode: minifb::ScaleMode::AspectRatioStretch,
        ..WindowOptions::default()
    };

    // Initialize the window
    let mut window = Window::new("Rust VM", WINDOW_WIDTH, WINDOW_HEIGHT, window_options)
        .expect("Unable to create window");

    window.set_target_fps(60);

    let mut is_halted = false;
    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if !is_halted {
            for _ in 0..TICKS_PER_FRAME {
                if vm.clock() == rust_vm::OPCODE_HLT {
                    is_halted = true;
                    break;
                }
            }
        }

        handle_controller_input(&mut vm, &window);

        screen_to_buffer_with_hud(vm.io_devices.screen, &mut buffer[..]);

        // Update the window with the buffer
        window.update_with_buffer(&buffer, width, height).unwrap();

        vm.io_devices.character_display.display();
        vm.io_devices.number_display.display();
    }
}

fn screen_to_buffer_with_hud(screen: Screen, buffer: &mut [u32]) {
    // Fill the HUD area (top HUD_HEIGHT rows) with a background color (e.g., dark gray)
    for y in 0..HUD_HEIGHT {
        for x in 0..WINDOW_WIDTH {
            buffer[y * WINDOW_WIDTH + x] = 0x222222; // HUD background color
        }
    }

    // Draw the screen below the HUD
    let screen_height = screen.active.len();
    for (i, row) in screen.active.iter().enumerate() {
        let flipped_i = screen_height - 1 - i;
        for (j, &pixel) in row.iter().enumerate() {
            let color = if pixel { 0xF5CBA7 } else { 0x8B4513 };
            for dy in 0..PIXEL_SIZE {
                for dx in 0..PIXEL_SIZE {
                    let x = j * PIXEL_SIZE + dx;
                    let y = HUD_HEIGHT + flipped_i * PIXEL_SIZE + dy;
                    if x < WINDOW_WIDTH && y < WINDOW_HEIGHT {
                        buffer[y * WINDOW_WIDTH + x] = color;
                    }
                }
            }
        }
    }
}

fn handle_controller_input(vm: &mut rust_vm::VM, window: &Window) {
    vm.io_devices
        .controller
        .set_up(window.is_key_down(Key::Up) || window.is_key_down(Key::W));
    vm.io_devices
        .controller
        .set_down(window.is_key_down(Key::Down) || window.is_key_down(Key::S));
    vm.io_devices
        .controller
        .set_left(window.is_key_down(Key::Left) || window.is_key_down(Key::A));
    vm.io_devices
        .controller
        .set_right(window.is_key_down(Key::Right) || window.is_key_down(Key::D));
}
