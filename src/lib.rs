use std::ffi::{c_void, CString};
use wokwi_chip_ll::{debugPrint, i2cInit, pinInit, I2CConfig, INPUT};

struct Chip {
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

#[derive(Clone, Copy)]
enum Register {
    WhoAmI = 0x75,
    Uninitialized = 0xFF,
}

impl Register {
    fn from_address(address: u8) -> Register {
        match address {
            0x75 => Register::WhoAmI,
            _ => Register::Uninitialized,
        }
    }
}

const ADDRESS: u32 = 0x68;

const PRODUCT_ID_HI: u8 = 0x60;
const PRODUCT_ID_LO: u8 = 0x00;

static mut CHIP_VEC: Vec<Chip> = Vec::new();

#[no_mangle]
pub unsafe extern "C" fn chipInit() {
    debugPrint(CString::new("Initializing ICM42670P").unwrap().into_raw());
    let chip = Chip {
        internal_address: Register::Uninitialized,
        state: State::ExpectingConnect,
    };
    CHIP_VEC.push(chip);

    let i2c_config: I2CConfig = I2CConfig {
        user_data: (CHIP_VEC.len() - 1) as *const c_void,
        address: ADDRESS,
        scl: pinInit(CString::new("SCL").unwrap().into_raw(), INPUT),
        sda: pinInit(CString::new("SDA").unwrap().into_raw(), INPUT),
        connect: on_i2c_connect as *const c_void,
        read: on_i2c_read as *const c_void,
        write: on_i2c_write as *const c_void,
        disconnect: on_i2c_disconnect as *const c_void,
    };
    i2cInit(&i2c_config);

    debugPrint(CString::new("Chip initialized!").unwrap().into_raw());
}

pub unsafe fn on_i2c_connect(user_ctx: *const c_void, address: u32, read: bool) -> bool {
    let msg: String = format!("on_i2c_connect: address: {:#02x}, read: {}", address, read);
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
                chip.state = State::ExpectingReadByte2;
                return PRODUCT_ID_HI;
            }
            _ => {
                chip.state = State::ExpectingConnect;
            }
        },
        State::ExpectingReadByte2 => match chip.internal_address {
            Register::WhoAmI => {
                chip.state = State::ExpectingConnect;
                return PRODUCT_ID_LO;
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
    let msg = format!("on_i2c_write: {:#02x}", data);
    debugPrint(CString::new(msg).unwrap().into_raw());

    let chip: &mut Chip = &mut CHIP_VEC[user_ctx as usize];
    chip.internal_address = Register::from_address(data);

    match chip.state {
        State::ExpectingWriteByte1 => {
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
