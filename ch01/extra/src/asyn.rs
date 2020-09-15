// https://www.snoyman.com/blog/2018/12/rust-crash-course-07-async-futures-tokio
// https://www.snoyman.com/blog/2019/12/rust-crash-course-08-down-dirty-future
// https://www.snoyman.com/blog/2019/12/rust-crash-course-09-tokio-0-2

use std::io;
use tokio::task;
use futures::executor::block_on;

use tokio::prelude::*;
use tokio::time::{delay_for, interval_at, Duration, Instant};

use std::{thread, time};

const INTERVAL_IN_SECONDS: u64 = 5;

// We need interval for the futures
// This method will run items synchronously since its not marked
pub fn run() {
    println!("Run 1");
    basic_run();
    println!("Run 2");
    spawn_run_block();
    println!("Run 3");    
    spawn_blocker();
    println!("Run 4");
    run_tok();
    loop{}
}

#[tokio::main]
async fn run_tok() {
    interval_test();
}

async fn interval_test() {
    let mut interval = interval_at(Instant::now(), Duration::from_secs(INTERVAL_IN_SECONDS));
    tokio::spawn(async move {
        delay_for(Duration::from_secs(5)).await;             
        loop {            
            interval.tick().await;            
            println!("Tick ..");
        }
    });    
}

async fn spawn_blocker() {
    println!("Pre Spawn");
    task::spawn_blocking(move || { 
        thread::sleep(time::Duration::from_millis(100));
        println!("Spawn ..");
    }).await;    
    println!("Post Spawn");
}

#[tokio::main]
async fn basic_run() {
    run_100_ms().await;
    run_10_ms().await;
    // Won't run since we are ant 
    run_20_ms();
}

async fn run_100_ms() {
    thread::sleep(time::Duration::from_millis(100));

    println!("Async run 100ms")
}

async fn run_10_ms() {
    thread::sleep(time::Duration::from_millis(10));
    println!("Async run 10")
}

async fn run_20_ms() {
    thread::sleep(time::Duration::from_millis(20));
    println!("Async run 20ms")
}

#[tokio::main]
async fn spawn_run() {
    thread::sleep(time::Duration::from_millis(100));
    println!("Spawn Run");

    // spawns a non blocking future
    tokio::spawn(async move {
        // loop {
            // interval.tick().await;
            // task must implement !send
            thread::sleep(time::Duration::from_millis(100));
            println!("Spawn Run2");
        // }
    });
}

// used for IO operations taht CANT be performed Asynchrnonosuly
async fn spawn_run_block()  ->  io::Result<()> {
    thread::sleep(time::Duration::from_millis(100));
    println!("spawn run block");
    // spawns a blocking function
    let res = task::spawn_blocking(move || {
        // do some compute-heavy work or call synchronous code
        thread::sleep(time::Duration::from_millis(100));
        println!("done spawn blocking computing");
    }).await?;

    Ok(())
}

// there is an upper limit for the blocking calls
