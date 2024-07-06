#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;

const BOUNDARY: [(usize, usize); 25] = [
    (0,0), (0,1), (0,2), (0,3), (0,4), (1,4), (2,4), (3,4), (4,4),
    (4,3), (4,2), (4,1), (4,0), (3,0), (2,0), (1,0), (1,1), (1,2),
    (1,3), (2,3), (3,3), (3,2), (3,1), (2,1), (2,2)
];

const SNAKE_LENGTH: usize = 5;
const DELAY_LENGTH_MS: u32 = 75;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut light_matrix: [[u8; 5]; 5];
    let mut lead_light_index: usize = 0;
    let mut tail_light_index: usize = SNAKE_LENGTH;
    let mut display = Display::new(board.display_pins);

    // infinite loop; just so we don't leave this stack frame
    loop {
        // fresh light matrix
        light_matrix = [[0; 5]; 5];
        for index in lead_light_index..tail_light_index{
            light_matrix[BOUNDARY[index % BOUNDARY.len()].0][BOUNDARY[index % BOUNDARY.len()].1] = 1;
        }
        display.show(&mut timer, light_matrix, DELAY_LENGTH_MS);
        lead_light_index = if lead_light_index == BOUNDARY.len() {0} else {
            lead_light_index + 1
        };
        tail_light_index = if lead_light_index == 0 {tail_light_index % BOUNDARY.len()} else {
            tail_light_index + 1
        };
    }
}
