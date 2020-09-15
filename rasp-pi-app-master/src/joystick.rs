/**
 * The monitor for any joystick movements.
 * Run this asynchronously so we can always respond to it.
 */
use log::{info, warn};
use sensehat_stick::{JoyStick, JoyStickEvent, Action, Direction};
use crate::manager::{Tx, Action as DisplayAction};

// tag::run[]
pub fn run(mut tx: Tx) {
    let stick = JoyStick::open().unwrap();
    run_on_loop(stick, tx);    
}

fn run_on_loop( mut stick: JoyStick,
                mut tx: Tx) {
    use tokio::task;

    info!("Run Async Calls on the joystick");
    // Use Spawn Blocking since Stick Events is a blocking call, otherwise we risk blocking
    // the current thread
    task::spawn_blocking(move || {          // <1>
        loop {
            // TODO : Add some logic to break up the time if not you hold the button down
            // And you may get it displaying 5 times
            for ev in &stick.events().unwrap() {
                info!("Stick -- {:?}", ev);
                // Create a response based on events 
                // can be blank since the processing is inside
                if check_temp_event(&ev) {          // <2>
                    info!("Check Temperature Event");
                    send(&mut tx, DisplayAction::ShowTemperature)
                }
                // TODO we will add more complexity later to this
                else {
                    // let's just display a question mark
                    warn!("Not Supported Event");
                }
            }
        }
    });
}
// end::run[]

fn send(tx: &mut Tx, action: DisplayAction) {
    use futures::executor::block_on;

    // Async returns a future so we are blocking on it.
    block_on(async {
        if let Err(_) = tx.send(action).await {
            info!("receiver dropped");
            return;
        }
        info!("...joystick event sent");
    });
 }
 
/**
 * This displays ths temperature for an up down event.
 */
// tag::check[]
fn check_temp_event(ev: &JoyStickEvent) -> bool {
    // When the button is held down.
    if ev.action == Action::Hold
        && ev.direction == Direction::Enter {            
            return true;            
    }
    return false;
}
// end::check[]