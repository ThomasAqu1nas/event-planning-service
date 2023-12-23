use sha3::{Sha3_256, Digest};

pub fn get_sha3_256_hash(data: &String) -> String {
   let mut hasher = Sha3_256::default();
   hasher.update(data);
   format!("{:X}", hasher.finalize())
}