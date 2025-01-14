use md5::{Digest, Md5};
use serde::Serializer;

pub fn serialize_md5<S: Serializer>(input: &str, serializer: S) -> Result<S::Ok, S::Error> {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    serializer.serialize_str(&format!("{:x}", result))
}
