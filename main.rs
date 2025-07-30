use minifb::{Key, Window, WindowOptions};
use rust_vm::io_devices::screen::Screen;

const PIXEL_SIZE: usize = 15;
const GRID_WIDTH: usize = 32;
const GRID_HEIGHT: usize = 32;

const DISPLAY_WIDTH: usize = GRID_WIDTH * PIXEL_SIZE; // 480
const DISPLAY_HEIGHT: usize = GRID_HEIGHT * PIXEL_SIZE; // 480
const HUD_HEIGHT: usize = DISPLAY_HEIGHT / 16; // Enough for 1 line of scaled text

const WINDOW_WIDTH: usize = DISPLAY_WIDTH;
const WINDOW_HEIGHT: usize = DISPLAY_HEIGHT + HUD_HEIGHT;

const TICKS_PER_FRAME: usize = 150; // 9000 instructions per second
fn main() {
    let mut vm = rust_vm::VM::new();
    vm.load_program("programs/dvd.as").unwrap();

    let mut buffer: Vec<u32> = vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    let width = DISPLAY_WIDTH;
    let height = DISPLAY_HEIGHT;

    let window_options = WindowOptions {
        resize: true,
        scale_mode: minifb::ScaleMode::AspectRatioStretch,
        ..WindowOptions::default()
    };

    // Initialize the window
    let mut window = Window::new("Rust VM", WINDOW_WIDTH, WINDOW_HEIGHT, window_options)
        .expect("Unable to create window");

    window.set_target_fps(60);
    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..TICKS_PER_FRAME {
            vm.clock();
        }

        screen_to_buffer(vm.io_devices.screen, &mut buffer[..]);

        // Update the window with the buffer
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

fn screen_to_buffer(screen: Screen, buffer: &mut [u32]) {
    let height = screen.active.len();
    for (i, row) in screen.active.iter().enumerate() {
        let flipped_i = height - 1 - i;
        for (j, &pixel) in row.iter().enumerate() {
            let color = if pixel { 0xFFFFFFFF } else { 0xFFFF0000 };
            for dy in 0..PIXEL_SIZE {
                for dx in 0..PIXEL_SIZE {
                    let x = j * PIXEL_SIZE + dx;
                    let y = flipped_i * PIXEL_SIZE + dy;
                    if x < DISPLAY_WIDTH && y < DISPLAY_HEIGHT {
                        buffer[y * DISPLAY_WIDTH + x] = color;
                    }
                }
            }
        }
    }
}
