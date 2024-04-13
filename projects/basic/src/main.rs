#![no_main]
#![no_std]

use cortex_m as _;
use rtt_target::{rprintln, rtt_init_print};

// Base offsets
const GPIO_BASE: u32 = 0x5000_0000;

const OUT_OFFSET: u32 = 0x504;
const IN_OFFSET: u32 = 0x510;
const PIN_CNF_OFFSET: u32 = 0x700;

// Const pointers
const OUT_PTR: *mut u32 = (GPIO_BASE + OUT_OFFSET) as *mut u32;
const IN_PTR: *mut u32 = (GPIO_BASE + IN_OFFSET) as *mut u32;
const PIN_CNF_PTR: *mut u32 = (GPIO_BASE + PIN_CNF_OFFSET) as *mut u32;

// PINS
const BTN1_PIN: u32 = 17;
const BTN2_PIN: u32 = 26;

const ROW1_PIN: u32 = 13;
const ROW2_PIN: u32 = 14;
const ROW3_PIN: u32 = 15;

const COL1_PIN: u32 = 4;
const COL2_PIN: u32 = 5;
const COL3_PIN: u32 = 6;
const COL4_PIN: u32 = 7;
const COL5_PIN: u32 = 8;
const COL6_PIN: u32 = 9;
const COL7_PIN: u32 = 10;
const COL8_PIN: u32 = 11;
const COL9_PIN: u32 = 12;

#[cortex_m_rt::entry]
fn main_2() -> ! {
    // Init
    rtt_init_print!(rtt_target::ChannelMode::NoBlockTrim);
    rprintln!("Starting");

    // Setup pins
    // Connect
    set_pin_connected(BTN1_PIN);
    set_pin_connected(BTN2_PIN);

    set_pin_connected(ROW1_PIN);
    set_pin_connected(ROW2_PIN);
    set_pin_connected(ROW3_PIN);

    set_pin_connected(COL1_PIN);
    set_pin_connected(COL2_PIN);
    set_pin_connected(COL3_PIN);
    set_pin_connected(COL4_PIN);
    set_pin_connected(COL5_PIN);
    set_pin_connected(COL6_PIN);
    set_pin_connected(COL7_PIN);
    set_pin_connected(COL8_PIN);
    set_pin_connected(COL9_PIN);

    // Output
    set_pin_output(ROW1_PIN);
    set_pin_output(ROW2_PIN);
    set_pin_output(ROW3_PIN);

    set_pin_output(COL1_PIN);
    set_pin_output(COL2_PIN);
    set_pin_output(COL3_PIN);
    set_pin_output(COL4_PIN);
    set_pin_output(COL5_PIN);
    set_pin_output(COL6_PIN);
    set_pin_output(COL7_PIN);
    set_pin_output(COL8_PIN);
    set_pin_output(COL9_PIN);

    // Config pull
    set_pin_push_pull(ROW1_PIN, 3, true);
    set_pin_push_pull(ROW2_PIN, 3, true);
    set_pin_push_pull(ROW3_PIN, 3, true);

    set_pin_push_pull(COL1_PIN, 2, true);
    set_pin_push_pull(COL2_PIN, 2, true);
    set_pin_push_pull(COL3_PIN, 2, true);
    set_pin_push_pull(COL4_PIN, 2, true);
    set_pin_push_pull(COL5_PIN, 2, true);
    set_pin_push_pull(COL6_PIN, 2, true);
    set_pin_push_pull(COL7_PIN, 2, true);
    set_pin_push_pull(COL8_PIN, 2, true);
    set_pin_push_pull(COL9_PIN, 2, true);

    let mut panel_state = false;

    loop {
        let mut state_changed = false;
        match panel_state {
            true => {
                if !read_pin_state(BTN2_PIN) {
                    panel_state = false;
                    state_changed = true;
                }
            }
            false => {
                if !read_pin_state(BTN1_PIN) {
                    panel_state = true;
                    state_changed = true;
                }
            }
        }

        if state_changed {
            rprintln!("The led state is: {}", panel_state);
            set_leds(panel_state);
            sleep_for(10_000);
        }
    }
}

fn nop() {
    unsafe { core::arch::asm!("nop") }
}

#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {
        for _ in 0..100_000 {
            nop();
        }
    }
}

fn sleep_for(n: u32) {
    for _ in 0..n {
        nop();
    }
}

fn set_bit(ptr: *mut u32, bit: u32, state: bool) {
    let current_state = unsafe { ptr.read_volatile() };
    let new_state;
    if state {
        new_state = current_state | 1 << bit;
    } else {
        new_state = current_state & !(1 << bit);
    }
    unsafe { ptr.write_volatile(new_state) };
}

fn set_pin_connected(pin: u32) {
    set_bit(unsafe { PIN_CNF_PTR.add(pin as usize) }, 1, false);
}

fn set_pin_output(pin: u32) {
    set_bit(unsafe { PIN_CNF_PTR.add(pin as usize) }, 0, true);
}

fn set_pin_push_pull(pin: u32, config: u32, state: bool) {
    set_bit(unsafe { PIN_CNF_PTR.add(pin as usize) }, config, state);
}

fn read_pin_state(pin: u32) -> bool {
    unsafe { IN_PTR.read_volatile() & 1 << pin != 0 }
}

fn set_pin_state(pin: u32, state: bool) {
    set_bit(OUT_PTR, pin, state);
}

fn set_leds(state: bool) {
    set_pin_state(ROW1_PIN, state);
    set_pin_state(ROW2_PIN, state);
    set_pin_state(ROW3_PIN, state);
    set_pin_state(COL1_PIN, !state);
    set_pin_state(COL2_PIN, !state);
    set_pin_state(COL3_PIN, !state);
    set_pin_state(COL4_PIN, !state);
    set_pin_state(COL5_PIN, !state);
    set_pin_state(COL6_PIN, !state);
    set_pin_state(COL7_PIN, !state);
    set_pin_state(COL8_PIN, !state);
    set_pin_state(COL9_PIN, !state);
}
