/**
 * This is our daily manager runner.
 */
use tokio::prelude::*;
use tokio::time::{interval_at, Duration, Instant};

use chrono::prelude::*;
use log::{info, debug};
use crate::manager::{Tx, Action, Display};

// tag::run[]
const INTERVAL_IN_SECONDS: u64 = 60 * 60;

 pub fn run(mut tx: Tx) {
    use std::ops::Add;

    let local: DateTime<Local> = Local::now();  // <1>
    let min = local.minute();    

    // Determine the time till the top of the hour
    let time_from_hour = 60 - min;                  // <2>
    debug!("Min from hour : {:?}", time_from_hour);
    let time_at_hour = Instant::now();
    time_at_hour.add(Duration::from_secs((60 * time_from_hour).into()));    // <3>

    // Compute the interval
    let mut interval = interval_at(time_at_hour, Duration::from_secs(INTERVAL_IN_SECONDS)); // <4>
    tokio::spawn(async move {           // <5>
        // run on initial start up then timers after
        run_initial(&mut tx).await;     // <6>

        loop {            
            interval.tick().await;      // <7>
            info!("Fire the Timer Checker; ");
            display_special(&mut tx);   // <8>
        }
    });    
 }

async fn send(tx: &mut Tx, action: Action) {    // <9>
    if let Err(_) = tx.send(action).await {
        info!("receiver dropped");
        return;
    }
}
// end::run[]

 /**
  * Run on initial start.
  */
 // tag::initial[]
async fn run_initial(tx: &mut Tx) {
    let local: DateTime<Local> = Local::now();
    if is_christmas(&local) {            
        send(tx, Action::Print(Display::Christmas)).await;
    }
    else if is_halloween(&local) {
        send(tx, Action::Print(Display::Halloween)).await;
    }
}
// end::initial[]

 /**
  * Calculate the daily special that we should send.
  */
 // tag::special[]
 async fn display_special(tx: &mut Tx) {
    let local: DateTime<Local> = Local::now();

    // now switch based o the variable to display
    // we will only call this on the hour so we don't need to check the minute
    // also could be a delay so better to not be that precise
    if local.hour() == 8 {
        //display_weather(tx);
        send(tx, Action::ShowTemperature).await;
    }
    else if local.hour() == 12 {
        if is_christmas(&local) {            
            send(tx, Action::Print(Display::Christmas)).await;
        }
        else if is_halloween(&local) {
            send(tx, Action::Print(Display::Halloween)).await;
        }
    }
 }
// end::special[]

// tag::is[]
 fn is_halloween(local: &DateTime<Local>) -> bool {
     local.month() == 10 && local.day() == 31
 }

 fn is_christmas(local: &DateTime<Local>) -> bool {
     // Any day in Christmas before the 25th
    local.month() == 12 && local.day() <= 25
}
// end::is[]
 

// #[cfg(test)]
// mod tests {
//     use super::{Instant, SystemTime, Duration, UNIX_EPOCH};

//     #[test]
//     fn instant_add_check() {
//         let a = Instant::now();
//         a.add(other: Duration)
//         let b = Instant::now();
//         assert!(b >= a);
//     }
