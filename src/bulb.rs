use gpio::{self, sysfs, GpioIn, GpioOut, GpioValue};

const PIN: u16 = 2;

#[derive(Debug)]
pub enum Status {
    On,
    Off,
}

pub struct Bulb {
    input: sysfs::SysFsGpioInput,
    output: sysfs::SysFsGpioOutput,
}

impl Bulb {
    pub fn flip(&mut self) -> Status {
        let cur = self.input.read_value().expect("GPIO read failed");
        match cur {
            GpioValue::Low => {
                self.output.set_high().expect("GPIO write failed");
                return Status::On;
            }
            GpioValue::High => {
                self.output.set_low().expect("GPIO read failed");
                return Status::Off; }
        }
    }

    pub fn status(&mut self) -> Status {
        let cur = self.input.read_value().expect("GPIO read failed");
        match cur {
            GpioValue::Low => Status::Off,
            GpioValue::High => Status::On,
        }
    }
}

pub fn bulb() -> Bulb {
    return Bulb {
        input: sysfs::SysFsGpioInput::open(PIN).expect("GPIO open failed"),
        output: sysfs::SysFsGpioOutput::open(PIN).expect("GPIO open failed"),
    }
}