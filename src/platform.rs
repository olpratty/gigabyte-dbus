use std::path::PathBuf;
use udev::Device;
use zbus::dbus_interface;

pub struct CtrlPlatform {
    pub path: PathBuf,
}

impl CtrlPlatform {
    pub fn new() -> Result<Self, ()> {
        let mut enumerator = udev::Enumerator::new().map_err(|err| {
            eprintln!("Could not create platform enumerator: {err}");
        })?;

        enumerator.match_subsystem("platform").map_err(|err| {
            eprintln!("Could not match platform subsystem: {err}");
        })?;
        enumerator.match_sysname("aorus_laptop").map_err(|err| {
            eprintln!("Could not match AORUS platform device: {err}");
        })?;
        if let Some(device) = (enumerator
            .scan_devices()
            .map_err(|err| println!("Could not scan devices: {:?}", err))?)
        .next()
        {
            return Ok(Self {
                path: device.syspath().to_path_buf(),
            });
        }
        Err(())
    }
}

#[dbus_interface(name = "com.gigabyte.Platform")]
impl CtrlPlatform {
    // TODO: Implement
    fn set_fan_mode(&self, value: i32) -> zbus::fdo::Result<i32> {
        if !matches!(value, 0 | 5) {
            return Err(zbus::fdo::Error::InvalidArgs(
                "This AORUS integration supports fan modes 0 and 5".into(),
            ));
        }

        std::fs::write(self.path.join("fan_mode"), value.to_string()).map_err(|err| {
            zbus::fdo::Error::Failed(format!("Failed to write fan_mode={value}: {err}"))
        })?;

        Ok(0)
    }
    fn set_fan_speed(&self, value: i32) -> zbus::fdo::Result<i32> {
        // The plugin's tested 10-100% range maps to 23-227.
        if !(23..=227).contains(&value) {
            return Err(zbus::fdo::Error::InvalidArgs(
                "This AORUS integration requires fan speed 23 through 227".into(),
            ));
        }

        std::fs::write(self.path.join("fan_custom_speed"), value.to_string()).map_err(|err| {
            zbus::fdo::Error::Failed(format!("Failed to write fan_custom_speed={value}: {err}"))
        })?;

        Ok(0)
    }
    fn set_charge_mode(&self, value: i32) -> i32 {
        let mut device = Device::from_syspath(&self.path).unwrap();
        device
            .set_attribute_value("charge_mode", &(value).to_string())
            .unwrap();
        0
    }
    fn set_charge_limit(&self, value: i32) -> i32 {
        let mut device = Device::from_syspath(&self.path).unwrap();
        device
            .set_attribute_value("charge_limit", &(value).to_string())
            .unwrap();
        0
    }
    fn set_fan_curve_index(&self, value: i32) -> i32 {
        let mut device = Device::from_syspath(&self.path).unwrap();
        device
            .set_attribute_value("fan_curve_index", &(value).to_string())
            .unwrap();
        0
    }
    fn set_fan_curve_data(&self, speed: i32, temp: i32) -> i32 {
        let value = speed << 8 | temp;
        let mut device = Device::from_syspath(&self.path).unwrap();
        device
            .set_attribute_value("fan_curve_data", &(value).to_string())
            .unwrap();
        0
    }
    fn set_gpu_boost(&self, value: i32) -> i32 {
        let mut device = Device::from_syspath(&self.path).unwrap();
        device
            .set_attribute_value("gpu_boost", &(value).to_string())
            .unwrap();
        0
    }
    // get methods
    fn get_fan_mode(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let fan_mode = device.attribute_value("fan_mode").unwrap();
        fan_mode.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_fan_speed(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let fan_speed = device.attribute_value("fan_custom_speed").unwrap();
        fan_speed.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_charge_mode(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let charge_mode = device.attribute_value("charge_mode").unwrap();
        charge_mode.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_charge_limit(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let charge_limit = device.attribute_value("charge_limit").unwrap();
        charge_limit.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_fan_curve_index(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let fan_curve_index = device.attribute_value("fan_curve_index").unwrap();
        fan_curve_index.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_fan_curve_data(&self) -> (i32, i32) {
        let device = Device::from_syspath(&self.path).unwrap();
        let fan_curve_data = device.attribute_value("fan_curve_data").unwrap();
        let mut parts = fan_curve_data.to_str().unwrap().split_whitespace();
        let temp = parts.next().unwrap().parse::<i32>().unwrap();
        let speed = parts.next().unwrap().parse::<i32>().unwrap();
        (temp, speed)
    }
    fn get_battery_cycle(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let battery_cycle = device.attribute_value("battery_cycle").unwrap();
        battery_cycle.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_usb_s3_toggle(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let usb_s3_toggle = device.attribute_value("usb_charge_s3_toggle").unwrap();
        usb_s3_toggle.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_usb_s4_toggle(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let usb_s4_toggle = device.attribute_value("usb_charge_s4_toggle").unwrap();
        usb_s4_toggle.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_gpu_boost(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let gpu_boost = device.attribute_value("gpu_boost").unwrap();
        gpu_boost.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_power_on_time(&self) -> (i32, i32, i32, i32, i32) {
        let device = Device::from_syspath(&self.path).unwrap();
        let power_on_time = device.attribute_value("power_on_time").unwrap();
        let mut parts = power_on_time.to_str().unwrap().split_whitespace();
        let day = parts.next().unwrap().parse::<i32>().unwrap();
        let hour = parts.next().unwrap().parse::<i32>().unwrap();
        let minute = parts.next().unwrap().parse::<i32>().unwrap();
        let month = parts.next().unwrap().parse::<i32>().unwrap();
        let year = parts.next().unwrap().parse::<i32>().unwrap();
        (day, hour, minute, month, year)
    }
    fn get_light_sensor(&self) -> (i32, i32, i32, i32) {
        let device = Device::from_syspath(&self.path).unwrap();
        let light_sensor = device.attribute_value("light_sensor").unwrap();
        // TODO: Check for older models, as they don't use the same format
        let mut parts = light_sensor.to_str().unwrap().split_whitespace();
        let version = parts.next().unwrap().parse::<i32>().unwrap();
        let lux_low = parts.next().unwrap().parse::<i32>().unwrap();
        let lux_med = parts.next().unwrap().parse::<i32>().unwrap();
        let lux_high = parts.next().unwrap().parse::<i32>().unwrap();
        (version, lux_low, lux_med, lux_high)
    }
    fn get_fan_pwm(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let fan_pwm = device.attribute_value("fan_pwm").unwrap();
        fan_pwm.to_str().unwrap().parse::<i32>().unwrap()
    }
}
