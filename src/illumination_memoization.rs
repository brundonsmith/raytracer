
use std::collections::HashMap;
use std::sync::{Mutex};

use crate::vec3::Vec3;
use crate::illumination::Illumination;

lazy_static! {
    static ref MEMOIZED_ILLUMINATIONS: Mutex<HashMap<usize, Vec<(Vec3,Illumination)>>> = Mutex::new(HashMap::new());
}

pub fn memoize_illumination(obj_index: usize, position: Vec3, illumination: Illumination) {
    /*
    let mut locked = MEMOIZED_ILLUMINATIONS.lock().unwrap();
    let existing = locked.get_mut(&obj_index);

    match existing {
        Some(vec) => vec.push(( position, illumination )),
        None => {
            locked.insert(obj_index, vec![ ( position, illumination ) ]);
        }
    };*/
}

pub fn find_memoized_illumination(obj_index: usize, position: &Vec3) -> Option<Illumination> {
    return None;
    /*
    let locked = MEMOIZED_ILLUMINATIONS.lock().unwrap();
    locked.get(&obj_index)
        .map(|vec| vec.iter()
            .find(|&pair| (position - &pair.0).len() < 1.0)
            .map(|&pair| pair.1.clone()))
        .unwrap_or(None)*/
}

pub fn print_memoization() {
    let locked = MEMOIZED_ILLUMINATIONS.lock().unwrap().clone();
    for (index, lums) in locked {
        println!("{}: {:?}", index, lums.len());
    }
}