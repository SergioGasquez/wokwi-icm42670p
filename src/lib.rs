// Wokwi Custom Chips with Rust
//
// Very rough prototype by Uri Shaked
//
// Look at chipInit() at the bottom, and open Chrome devtools console to see the debugPrint().

use std::ffi::{c_void, CString};
use wokwi_chip_ll::{
    debugPrint, pinInit, pinWatch, pinWrite, I2CConfig, I2CDevId, PinId, BOTH, HIGH, INPUT, LOW,
    OUTPUT,
};

struct Chip {
    internal_address: I2CDevId,
}

static mut CHIP_VEC: Vec<Chip> = Vec::new();

#[no_mangle]
pub unsafe extern "C" fn chipInit() {
    debugPrint(CString::new("Hello Rust!").unwrap().into_raw());

    let i2c_config = I2CConfig {
        user_data: 0 as *const c_void,
        address: 0x42,
        scl: pinInit(CString::new("SCL").unwrap().into_raw(), INPUT),
        sda: pinInit(CString::new("SDA").unwrap().into_raw(), INPUT),
        connect: on_i2c_connect as *const c_void,
        read: on_i2c_read as *const c_void,
        write: on_i2c_write as *const c_void,
        disconnect: on_i2c_disconnect as *const c_void,
    };

    // let chip = Chip {
    //     internal_address: i2cInit(),
    // };
    // CHIP_VEC.push(chip);
    // let chip = CHIP_VEC.last().unwrap();

    // let watch_config = WatchConfig {
    //     user_data: (CHIP_VEC.len() - 1) as *const c_void,
    //     edge: BOTH,
    //     pin_change: on_pin_change as *const c_void,
    // };

    // pinWatch(chip.pin_in, &watch_config);
}

pub unsafe fn on_i2c_connect(user_ctx: *const c_void, address: u32, read: bool) -> bool {
    let chip = &CHIP_VEC[user_ctx as usize];
    // if value == HIGH {
    //     pinWrite(chip.pin_out, LOW);
    // } else {
    //     pinWrite(chip.pin_out, HIGH);
    // }
    true
}

pub unsafe fn on_i2c_read(user_ctx: *const c_void, data: u8) -> u8 {
    let chip = &CHIP_VEC[user_ctx as usize];
    // if value == HIGH {
    //     pinWrite(chip.pin_out, LOW);
    // } else {
    //     pinWrite(chip.pin_out, HIGH);
    // }
    8
}

pub unsafe fn on_i2c_write(user_ctx: *const c_void, data: u8) -> bool {
    let chip = &CHIP_VEC[user_ctx as usize];
    // if value == HIGH {
    //     pinWrite(chip.pin_out, LOW);
    // } else {
    //     pinWrite(chip.pin_out, HIGH);
    // }
    true
}

pub unsafe fn on_i2c_disconnect(user_ctx: *const c_void, data: u8) {
    // Do nothing
}
