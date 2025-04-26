use sha256;
use std::thread;
use std::sync::mpsc::Sender;

fn check_hash(hash: &str, zeros: u32) -> bool {
    let hash_len = hash.len();
    let last_zero = hash_len - (zeros as usize);

    // кол-во нулей не меньше заданного
    let first_condition = hash[last_zero..hash_len] == "0".repeat(zeros as usize);

    // кол-во нулей не больше заданного
    let second_condition = hash[last_zero - 2..hash_len] != "0".repeat((zeros + 1) as usize);

    first_condition && second_condition
}

pub fn calculate_hashes(tx: Sender<(u64, String)>, zeros: u32, threads_count: u64) {
    for i in 1..=threads_count {
        let tx_loc = tx.clone();

        thread::spawn(move || {
            let mut i_loc: u64 = i;

            loop {
                let hash = sha256::digest(i_loc.to_string());

                if check_hash(&hash, zeros) {
                    let _ = tx_loc.send((i_loc, hash));
                }

                i_loc += &threads_count;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    #[test]
    fn contains_right_number_of_zeros() {
        let hashes = [
            "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b",
            "535fa30d7e25dd8a49f1536779734ec8286108d115da5045d77f3b4185d8f790",
            "d26eae87829adde551bf4b852f9da6b8c3c2db9b65b8b68870632a2db5f53e00",
            "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000",
            "16b024b09ebcb9d66f6a9968858d7e01081e51a14a4922edf3c8e3c2009c0000",
        ];

        hashes
            .iter()
            .enumerate()
            .for_each(|(idx, hash)| {
                assert!(super::check_hash(hash, idx as u32));
            });
    }

    #[test]
    fn sends_valid_data_to_channel() {
        let mut hashes_count = 0;
        let (tx, rx) = mpsc::channel();

        super::calculate_hashes(tx, 0, 1);

        for pair in rx {
            if hashes_count <= 5 {
                hashes_count += 1;
                assert!(sha256::digest(pair.0.to_string()) == pair.1);
            } else {
                break;
            }
        }
    }
}
