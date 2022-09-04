#[derive(Debug, Serialize)]
pub(crate) struct BlockHeader {
    pub(crate) timestamp: i64,
    pub(crate) nonce: u32,
    pub(crate) previous_hash: String,
    pub(crate) merkle: String,
    pub(crate) difficulty: u32,
}