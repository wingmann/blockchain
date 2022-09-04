use crate::block_header::BlockHeader;
use crate::transaction::Transaction;

#[derive(Debug, Serialize)]
pub(crate) struct Block {
    pub(crate) header: BlockHeader,
    pub(crate) count: u32,
    pub(crate) transactions: Vec<Transaction>,
}
