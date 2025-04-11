use std::io::Read;

use md5;

const N_LIMIT: u64 = 1844674407370955200;
const KEY_PREFIX: &str = "iwrupvqb";

pub fn run() {
    let mut n: u64 = 1;
    // loop {
        // if n >= N_LIMIT {
        //     println!("limite atingido");
        //     break;
        // }

        let mut key: String = String::from(KEY_PREFIX);
        key.push_str(n.to_string().as_str());

        let hash = md5::compute(key);
    // }
}
