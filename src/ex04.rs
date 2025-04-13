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
            println!("n = {}; hash: {:?}", n, hash);
            break;
        }

        if n % 10000 == 0 {
            println!("{}", n);
        }
        
        n = n + 1;
    }
}

fn generate_hash(n: u64) -> md5::Digest {
    md5::compute([KEY_PREFIX, n.to_string().as_str()].concat())
}

fn verify_hash(hash: &md5::Digest) -> bool {
    hash[0] == 0 &&
    hash[1] == 0 &&
    hash[2] == 0
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_generates_hash() {
        assert_eq!(generate_hash(1), md5::Digest(0x6a76d07a5b54f2061b3bb4afb7433949_u128.to_be_bytes()) );
        assert_eq!(generate_hash(2), md5::Digest(0x6cff6c82636bb0caead7381126553037_u128.to_be_bytes()) );
    }

    #[test]
    fn test_verify_hash() {
        assert_eq!(verify_hash(&md5::Digest(0x6a76d07a5b54f2061b3bb4afb7433949_u128.to_be_bytes())), false);
        assert_eq!(verify_hash(&md5::Digest(0x0000087a5b54f2061b3bb4afb7433949_u128.to_be_bytes())), true);
        assert_eq!(verify_hash(&md5::Digest(0x0000787a5b54f2061b3bb4afb7433949_u128.to_be_bytes())), false);
    }
}