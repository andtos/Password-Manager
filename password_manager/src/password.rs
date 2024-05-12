use core::time;
use std::{thread::sleep, time::{SystemTime, UNIX_EPOCH}};

use rand::{rngs::StdRng, Rng, SeedableRng};


pub fn generate_secure_password() -> String{
    let mut password = "".to_string();
    for _ in 0..20{
        let current_time = SystemTime::now();
        let duration_since_epoch = current_time.duration_since(UNIX_EPOCH).unwrap();
        let seed = duration_since_epoch.as_micros();
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed as u64);
        let random_number: u8 = rng.gen_range(33..=126);
        match std::char::from_u32(random_number as u32) {
            Some(c) => password.push(c),
            None => panic!("Error generating password"), 
        };
        sleep(time::Duration::from_millis(10))
    };
    password
}