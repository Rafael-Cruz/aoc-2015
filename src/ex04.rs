use md5;

const N_LIMIT: u64 = 1844674407370955200;
const KEY_PREFIX: &str = "iwrupvqb";

pub fn run() {
    let mut n: u64 = 1;
    loop {
        if n >= N_LIMIT {
            println!("limite atingido");
            break;
        }

        let hash = generate_hash(n);
        
        if verify_hash(&hash) {
            println!("n = {}; hash: {}", n, hash);
            break;
        }
        
        n = n + 1;
    }
}

fn generate_hash(n: u64) -> String {
    let mut key: String = String::from(KEY_PREFIX);
    key.push_str(n.to_string().as_str());

    let hash = md5::compute(key);
    let number: u128 = u128::from_be_bytes(*hash);
    let formatted_number = format!("{:x}", number);

    return formatted_number
}

fn verify_hash(hash: &str) -> bool {
    &hash[..5] == "00000"
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_generates_hash() {
        assert_eq!(generate_hash(1), "6a76d07a5b54f2061b3bb4afb7433949");
        assert_eq!(generate_hash(2), "6cff6c82636bb0caead7381126553037");
    }

    #[test]
    fn test_verify_hash() {
        assert_eq!(verify_hash("6a76d07a5b54f2061b3bb4afb7433949"), false);
        assert_eq!(verify_hash("0000087a5b54f2061b3bb4afb7433949"), true);
        assert_eq!(verify_hash("0000787a5b54f2061b3bb4afb7433949"), false);
    }
}