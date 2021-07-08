use core::convert::AsRef;

use crate::{xdr::compound_types::LimitedString, AsHash, Error, Memo};

impl Memo {
    pub fn from_text_memo<T: AsRef<[u8]>>(text: T) -> Result<Self, Error> {
        let text = text.as_ref();
        let string = LimitedString::new(text.to_vec())?;
        Ok(Self::MemoText(string))
    }

    pub fn from_id_memo(id: u64) -> Self {
        Self::MemoId(id)
    }

    pub fn from_hash_memo<T: AsHash>(hash: T) -> Result<Self, Error> {
        let hash = hash.as_hash()?;
        Ok(Self::MemoHash(hash))
    }

    pub fn from_return_hash_memo<T: AsHash>(return_hash: T) -> Result<Self, Error> {
        let return_hash = return_hash.as_hash()?;
        Ok(Self::MemoReturn(return_hash))
    }
}