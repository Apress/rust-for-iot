use sensehat::SenseHat;
use std::fs;
use log::{debug, info};

const THERMAL_TEMP: &str = "/sys/class/thermal/thermal_zone0/temp";     // <1>

const WUNDERGROUND_ADJUSTMENT_FACTOR: f64 = 5.466;

pub struct Atmospheric {
    hat: SenseHat<'static>          // <2>
}

impl Atmospheric {
    pub fn new() -> Atmospheric {
        Atmospheric {
            hat: SenseHat::new().unwrap()               // <3>
        }
    }

    pub fn get_temperature(&mut self) -> String {
        // Get the temperature from the humidity
        // we could also do pressure
        let temp = self.hat.get_temperature_from_humidity().unwrap().as_celsius();  // <4>
        let thermal_tmp = fs::read_to_string(THERMAL_TEMP.to_string()).unwrap();    // <5>
        let thermal_tmp_str = thermal_tmp.as_str().trim();

        // CPU temp needs to be divided by a 1000 to get the actual Celsius temperature,
        // It supplies it like : 55991
        let cpu_temp: f64 = thermal_tmp_str.parse().unwrap();           // <6>
        let calculated_temp = temp - (((cpu_temp * 0.001)- temp)/5.466) - 6.0;  // <7>
        let calc_temp_f = calculated_temp * 1.8 + 32.0;         // <8>

        debug!("Calculated Temp: {:?} C", calculated_temp);
        info!("Calculated Temp: {:?} F", calc_temp_f);

        format!("{:.1} F", calc_temp_f)     // <9>
    }

    pub fn get_temperature_in_c(&mut self) -> f32 {
        // Get the temperature from the humidity
        // we could also do pressure
        let temp = self.hat.get_temperature_from_humidity().unwrap().as_celsius();
        let thermal_tmp = fs::read_to_string(THERMAL_TEMP.to_string()).unwrap();
        let thermal_tmp_str = thermal_tmp.as_str().trim();

        // acquire CPU temp
        let cpu_temp: f64 = thermal_tmp_str.parse::<f64>().unwrap() * 0.001;
        let calculated_temp = temp - ((cpu_temp - temp) / WUNDERGROUND_ADJUSTMENT_FACTOR );

        // F32 is the type needed by hap current_temperature
        calculated_temp as f32
    }
}