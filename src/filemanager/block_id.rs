#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockId {
    filename: String,
    blknum: u64,
}

impl BlockId {
    pub fn new(filename: String, blknum: u64) -> Self {
        BlockId { filename, blknum }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn number(&self) -> u64 {
        self.blknum
    }
}
