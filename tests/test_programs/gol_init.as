// define boundary 224 // 11100000 32x32
define boundary 248 // 11111000 8x8
// define boundary 252 // 11111100 4x4

// r1 - x
// r2 - y
// r3 - neighbor sum
// r4 - state
// r5 - neighbor x
// r6 - neighbor y
// r7 - gen counter
// r8 - temp I/O
// r9 - boundary
// r11 - scratch
// r12 - buffer cell state
// r13 - scratch
// r14 - scratch
// r15 - I/0

// RAM 0-31 first row buffer
// RAM 32-63 second row buffer

define pixel_x_port -8
define pixel_y_port -7
define draw_pixel_port -6
define clear_pixel_port -5
define load_pixel_port -4
define buffer_screen_port -3
define clear_screen_buffer_port -2
define write_char_port -1
define buffer_chars_port 0
define clear_chars_buffer_port 1
define show_number_port 2
define clear_number_port 3
define signed_mode_port 4
define unsigned_mode_port 5
define rng_port 6
define controller_input_port 7

  LDI r15 buffer_chars // 248
  LDI r9 boundary
// write "LIFE"
  STR r15 r0 clear_chars_buffer_port
  LDI r14 "L"
  STR r15 r14 write_char_port
  LDI r14 "I"
  STR r15 r14 write_char_port
  LDI r14 "F"
  STR r15 r14 write_char_port
  LDI r14 "E"
  STR r15 r14 write_char_port
  STR r15 r0 buffer_chars_port
// gen counter to 0
  STR r15 r0 unsigned_mode_port
  STR r15 r0 show_number_port


// initial pattern
  STR r15 r0 clear_screen_buffer_port
  CAL .initial_pattern
  STR r15 r0 buffer_screen_port

// init x/y
  LDI r1 0
  LDI r2 0

  hlt

.initial_pattern
  LDI r14 0
  STR r15 r14 pixel_x_port
  LDI r14 0
  STR r15 r14 pixel_y_port
  STR r15 r0 draw_pixel_port

  LDI r14 1
  STR r15 r14 pixel_x_port
  LDI r14 1
  STR r15 r14 pixel_y_port
  STR r15 r0 draw_pixel_port

  LDI r14 1
  STR r15 r14 pixel_x_port
  LDI r14 2
  STR r15 r14 pixel_y_port
  STR r15 r0 draw_pixel_port

  LDI r14 2
  STR r15 r14 pixel_x_port
  LDI r14 0
  STR r15 r14 pixel_y_port
  STR r15 r0 draw_pixel_port

  LDI r14 2
  STR r15 r14 pixel_x_port
  LDI r14 1
  STR r15 r14 pixel_y_port
  STR r15 r0 draw_pixel_port

  RET