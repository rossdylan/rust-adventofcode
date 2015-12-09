use crypto::md5;
use crypto::digest::Digest;


pub fn hash(data : &str) -> String {
    let mut hasher = md5::Md5::new();
    hasher.input_str(data);
    hasher.result_str()
}

pub fn find_hash(key: &str) -> usize {
    let trimmed = key.trim();
    (0..).filter(|i| {
        hash(&(trimmed.to_string() + &i.to_string())).starts_with("00000")
    }).next().unwrap()
}
