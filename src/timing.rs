
use std::collections::HashMap;
use std::time::{Duration,Instant};
use std::sync::Mutex;

lazy_static! {
    static ref DURATIONS: Mutex<HashMap<String, Duration>> = Mutex::new(HashMap::new());
    static ref ONGOING: Mutex<HashMap<String, Instant>> = Mutex::new(HashMap::new());
}

pub fn start(name: &str) {
    /*
    stop(name);
    let mut ongoing_locked = ONGOING.lock().unwrap();
    ongoing_locked.insert(String::from(name), Instant::now());*/
}

pub fn stop(name: &str) {
    /*
    let name_string = String::from(name);
    let finish_time = Instant::now();

    let mut durations_locked = DURATIONS.lock().unwrap();
    let mut ongoing_locked = ONGOING.lock().unwrap();

    match ongoing_locked.remove(&name_string) {
        Some(start_time) => {
            
            //println!("start_time: {:?}, finish_time: {:?}", &start_time, finish_time);
            let ongoing_total = finish_time.duration_since(start_time);

            let existing_duration: Duration = durations_locked.get(&name_string).map(|d| *d).unwrap_or(Duration::from_secs(0));
            durations_locked.insert(name_string, existing_duration + ongoing_total);
        },
        _ => ()
    };*/
}

pub fn finish(name: &str) {
    /*
    stop(name);

    let locked = DURATIONS.lock().unwrap();
    println!("'{}' took {}s", name, locked.get(&String::from(name)).unwrap().as_millis() as f32 / 1000.0);*/
}