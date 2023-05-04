// Wokwi Custom Chips with Rust
//
// Very rough prototype by Uri Shaked
//
// Look at chipInit() at the bottom, and open Chrome devtools console to see the debugPrint().

use std::ffi::{c_void, CString};
use wokwi_chip_ll::{debugPrint, i2cInit, pinInit, I2CConfig, I2CDevId, INPUT};

struct Chip {
    _id: I2CDevId,
    internal_address: Register,
    state: State,
}

enum State {
    ExpectingConnect,
    ExpectingReadByte1,
    ExpectingReadByte2,
    ExpectingWriteByte1,
    ExpectingWriteByte2,
}

enum Register {
    WhoAmI = 0x75,
    Uninitialized = 0xFF,
}

const ADDRESS: u32 = 0x68;

static mut CHIP_VEC: Vec<Chip> = Vec::new();

#[no_mangle]
pub unsafe extern "C" fn chipInit() {
    debugPrint(CString::new("Hello Rust!").unwrap().into_raw());

    let i2c_config = I2CConfig {
        user_data: (CHIP_VEC.len() - 1) as *const c_void,
        address: ADDRESS,
        scl: pinInit(CString::new("SCL").unwrap().into_raw(), INPUT),
        sda: pinInit(CString::new("SDA").unwrap().into_raw(), INPUT),
        connect: on_i2c_connect as *const c_void,
        read: on_i2c_read as *const c_void,
        write: on_i2c_write as *const c_void,
        disconnect: on_i2c_disconnect as *const c_void,
    };

    let chip = Chip {
        _id: i2cInit(&i2c_config),
        internal_address: Register::Uninitialized,
        state: State::ExpectingConnect,
    };

    CHIP_VEC.push(chip);
    // let chip = CHIP_VEC.last().unwrap();
}

pub unsafe fn on_i2c_connect(user_ctx: *const c_void, address: u32, read: bool) -> bool {
    let msg: String = format!("on_i2c_connect: add: {}, read: {}", address, read);
    debugPrint(CString::new(msg).unwrap().into_raw());
    let chip: &mut Chip = &mut CHIP_VEC[user_ctx as usize];
    if read {
        chip.state = State::ExpectingReadByte1;
    } else {
        chip.state = State::ExpectingWriteByte1;
    }
    true
}

pub unsafe fn on_i2c_read(user_ctx: *const c_void) -> u8 {
    debugPrint(CString::new("on_i2c_read").unwrap().into_raw());
    let chip: &mut Chip = &mut CHIP_VEC[user_ctx as usize];

    match chip.state {
        State::ExpectingReadByte1 => match chip.internal_address {
            Register::WhoAmI => {
                debugPrint(CString::new("WhoAmI").unwrap().into_raw());
                chip.state = State::ExpectingReadByte2;
                return 0x67;
            }
            _ => {
                debugPrint(CString::new("Other").unwrap().into_raw());
                chip.state = State::ExpectingConnect;
            }
        },
        State::ExpectingReadByte2 => match chip.internal_address {
            Register::WhoAmI => {
                chip.state = State::ExpectingConnect;
            }
            _ => {
                chip.state = State::ExpectingConnect;
            }
        },
        _ => {
            chip.state = State::ExpectingConnect;
        }
    }
    0x0
}

pub unsafe fn on_i2c_write(user_ctx: *const c_void, data: u8) -> bool {
    let msg = format!("on_i2c_write: {}", data);
    debugPrint(CString::new(msg).unwrap().into_raw());

    let chip: &mut Chip = &mut CHIP_VEC[user_ctx as usize];

    match chip.state {
        State::ExpectingWriteByte1 => {
            chip.internal_address = match data {
                0x75 => Register::WhoAmI,
                _ => Register::Uninitialized,
            };
            chip.state = State::ExpectingWriteByte2;
        }
        _ => {
            chip.state = State::ExpectingConnect;
        }
    }
    true
}

pub unsafe fn on_i2c_disconnect(_user_ctx: *const c_void, _data: u8) {
    // Do nothing
}
