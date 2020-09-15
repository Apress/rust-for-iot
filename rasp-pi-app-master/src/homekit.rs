
use log::{debug,info};
use hap::{
    accessory::{valve, Category, Information, motion_sensor, temperature_sensor},
    characteristic::{Characteristic, Readable, Updatable},
    transport::{IpTransport, Transport},
    Config,
    HapType,
};

use crate::manager::{Tx, TempRx, MotionRx, Action};
use futures::Future;

// Motion Sensor
// tag::motion[]
pub struct Motion {
    rx: MotionRx,
    tx: Tx,
}

impl Motion {
    fn new(mut tx: Tx, rx: MotionRx) -> Motion {
        Motion {
            rx,
            tx
        }
    }
}

impl Readable<bool> for Motion {
    fn on_read(&mut self, _: HapType) -> Option<bool> {
        debug!("On read motion.");

        //let value = get_temperature(self.tx, &mut self.rx);
        let val : bool = get_motion(&mut self.tx, &mut self.rx);

        Some(val)
    }
}

#[tokio::main]
async fn get_motion(tx: &mut Tx, rx: &mut MotionRx) -> bool {
    send(tx, Action::SendMotion).await;

    let val = rx.recv().await;
    val.unwrap()
}
// end::motion[]

// Temperature
//#[derive(Clone)]
// tag::temp[]
pub struct Temperature {
    rx: TempRx,
    tx: Tx,
}

impl Temperature {
    fn new(mut tx: Tx, rx: TempRx) -> Temperature {
        Temperature {   // <1>
            tx,
            rx
        }
    }
}

impl Readable<f32> for Temperature {
    fn on_read(&mut self, _: HapType) -> Option<f32> {
        debug!("On read temp.");

        //let value = get_temperature(self.tx, &mut self.rx);
        let val : f32 = get_temperature(&mut self.tx, &mut self.rx); // <2>

        Some(val)
    }
}

#[tokio::main]
async fn get_temperature(tx: &mut Tx, rx: &mut TempRx) -> f32 {
    send(tx, Action::SendTemperature).await;    // <3>

    let val = rx.recv().await;                      // <4>
    val.unwrap()
}
// end::temp[]

async fn send(tx: &mut Tx, action: Action) {
    if let Err(_) = tx.send(action).await {
        info!("receiver dropped");
        return;
    }
}

pub fn initialize(motion_cmd_tx: Tx, motion_rx: MotionRx, temp_cmd_tx: Tx, temp_rx: TempRx) {
    let mut thermo = temperature_sensor::new(Information {
        name: "Thermostat".into(),
        ..Default::default()
    }).unwrap();

    // Temperature sets need to be in celsius
    // tag::temp_inst[]
    let thermometer = Temperature::new(temp_cmd_tx, temp_rx);
    thermo.inner.temperature_sensor.inner.current_temperature.set_readable(thermometer).unwrap();
    // end::temp_inst[]

    // Motion sensor
    // tag::motion_inst[]
    let mut motion = motion_sensor::new(Information {
        name: "Motion".into(),
        ..Default::default()
    }).unwrap();
    let motion_detect = Motion::new(motion_cmd_tx, motion_rx);
    motion.inner.motion_sensor.inner.motion_detected.set_readable(motion_detect).unwrap();
    // end::motion_inst[]

    // default is : 11122333 .. cant be overwritten right now
    // pin: "11122334".into(), -- doesn't work in the kit though
    // tag::config[]
    let config = Config {           // <1>
        name: "Rasp Pi".into(),
        category: Category::Thermostat,
        ..Default::default()
    };
    debug!("Whats the pin :: {:?}", config.pin);

    // Adds our transport layer to start
    let mut ip_transport = IpTransport::new(config).unwrap();   // <2>
    ip_transport.add_accessory(thermo).unwrap();            // <3>
    ip_transport.add_accessory(motion).unwrap();            // <4>

    // Spawn the start of the homekit monitor
    tokio::spawn(async move {
        debug!("IP Transport started");
        ip_transport.start().unwrap();                                   // <5>
    });
    // end::config[]
}