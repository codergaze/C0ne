#[derive(Copy, Clone)]
#[repr(u8)]
pub enum VgaColors {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

#[inline]
pub fn vga_entry_color(fg: VgaColors, bg: VgaColors) -> u8 {
    (fg as u8) | ((bg as u8) << 4)
}

#[inline]
pub fn vga_entry(character: u8, color: u8) -> u16 {
    (character as u16) | ((color as u16) << 8)
}

pub unsafe fn strlen(mut ptr: *const u8) -> usize {
    let mut len = 0;
    while *ptr != 0 {
        len += 1;
        ptr = ptr.add(1);
    }
    len
}

pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;
pub const VGA_MEMORY: usize = 0xB8000;

static mut TERMINAL_ROW: usize = 0;
static mut TERMINAL_COLUMN: usize = 0;
static mut TERMINAL_COLOR: u8 = 0;
static mut TERMINAL_BUFFER: *mut u16 = VGA_MEMORY as *mut u16;

pub fn terminal_initialize() {
    unsafe {
        TERMINAL_ROW = 0;
        TERMINAL_COLUMN = 0;
        TERMINAL_COLOR = vga_entry_color(VgaColors::LightGrey, VgaColors::Black);

        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let index = y * VGA_WIDTH + x;
                *TERMINAL_BUFFER.add(index) = vga_entry(b' ', TERMINAL_COLOR);
            }
        }
    }
}

pub unsafe fn terminal_setcolor(color: u8) {
    TERMINAL_COLOR = color;
}

pub unsafe fn terminal_put_entry_at(c: u8, color: u8, x: usize, y: usize) {
    let index = y * VGA_WIDTH + x;
    *TERMINAL_BUFFER.add(index) = vga_entry(c, color);
}

pub fn terminal_put_char(c: u8) {
    unsafe {
        terminal_put_entry_at(c, TERMINAL_COLOR, TERMINAL_COLUMN, TERMINAL_ROW);
        TERMINAL_COLUMN += 1;
        if TERMINAL_COLUMN == VGA_WIDTH {
            TERMINAL_COLUMN = 0;
            TERMINAL_ROW += 1;
            if TERMINAL_ROW == VGA_HEIGHT {
                TERMINAL_ROW = 0;
            }
        }
    }
}

pub fn terminal_write_string(data: &str) {
    for byte in data.bytes() {
        terminal_put_char(byte);
    }
}

pub fn terminal_write(data: &[u8], size: usize) {
    for i in 0..size {
        if i < data.len() {
            terminal_put_char(data[i]);
        }
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() {
    terminal_initialize();
    terminal_write_string("Hello goon World!\n");
}
