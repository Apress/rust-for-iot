pub mod recording;

use rumqtt::{Notification, Receiver};
use rumqtt::Notification::{Publish};
use std::{thread, str};
use log::{debug, error};

// Monitor the notifications from the notification receiver.
pub fn monitor_notifications(notifications: Receiver<Notification>, f: fn(String, Vec<u8>)) { // <2>
    thread::spawn(move || { // <3>
        for notification in notifications {
            debug!("Notification {:?}", notification);
            match notification {
                Publish(p) => { // <4>
                    // Retrieve the payload and convert it to a string.
                    //let pl = String::from_utf8(p.payload.to_vec());
                    let pl = p.payload.to_vec();
                    debug!("The Payload :: PKID: {:?}, QOS: {:?}, Duplicate: {:?}", p.pkid, p.qos, p.dup);
                    f(p.topic_name, pl);  // <5>
                },
                _ => debug!("n/a")
            }
        }
    });
}