use bytes::Bytes;

#[derive(Debug)]
pub enum Command {
    Get {
      key: String,
    },
    Set {
        key: String,
        val: Bytes,
    }
}