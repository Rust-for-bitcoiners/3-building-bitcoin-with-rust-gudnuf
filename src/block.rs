#![allow(unused)]

use crate::mresult::MResult;
use hex::{self, encode, ToHex};
use sha2::{Digest, Sha256};
use std::collections::LinkedList as List;

pub fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub struct BlockChain {
    blocks: List<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: List::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push_back(block);
    }

    pub fn get_block(&self, height: usize) -> Option<&Block> {
        self.blocks.iter().nth(height)
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        self.blocks.iter().find(|block| block.hash == hash)
    }
}

pub struct Block {
    hash: String,
    id: u128,
    transactions: List<Transaction>,
}

impl Block {
    pub fn new(hash: String, id: u128) -> Self {
        Block {
            hash,
            id,
            transactions: List::new(),
        }
    }

    // fn hash_transactions(txes: List<Transaction>) -> String {}
}

// ================START TRANSACTION ANATOMY=============

// version, fixed. 4-bytes: 01000000 -> 1
// count of inputs, compactSize field: 01
// txid, fixed. 32-bytes.
// output number, 8-bytes : 3b000000, (59)
// scriptSig. variable length :
//     length of the scriptSig, compactSize: 6a
//     data of scriptSig. variable: 473044022062f4f61766e1edf89b9b94d0629d67bfefb62e6ffec63f17d0f10ce5e329dbf4022039d566558c1f98742345e70b194bbcb8b8fff5b7fa2f0507c9f31a5a2ea24749012103fc3e6b07b7bec800cb0861c172344687d146888aaf0f811eaca8b1ef6684c9fc
//     nSequence, fixed. 4-bytes: 00000080
// count of outputs, compactSize field: 02
//       amount 8-bytes, little endian: f07e0e0000000000
//       scriptPubKey. variable length:
//       length of field: 17
//       data: a914783b39e49c6b6544a51bf0cbd4f47eef116be81987
// nLocktime, 4-bytes. fixed: 00000000

// ================ END TRANSACTION ANATOMY=============

fn encode_varint(n: usize) -> Vec<u8> {
    if n < 0xfd {
        vec![n as u8]
    } else if n <= 0xffff {
        let mut v = vec![0xfd];
        v.extend((n as u16).to_le_bytes());
        v
    } else if n <= 0xffffffff {
        let mut v = vec![0xfe];
        v.extend((n as u32).to_le_bytes());
        v
    } else {
        let mut v = vec![0xff];
        v.extend((n as u64).to_le_bytes());
        v
    }
}

fn decode_varint(input: &[u8]) -> MResult<(usize, usize), &'static str> {
    if input.is_empty() {
        return MResult::err("Input is empty");
    }

    match input[0] {
        0..=0xfc => MResult::ok((input[0] as usize, 1)),
        0xfd => {
            if input.len() < 3 {
                return MResult::err("Insufficient bytes for 2-byte varint");
            }
            let value = u16::from_le_bytes([input[1], input[2]]) as usize;
            MResult::ok((value, 3))
        }
        0xfe => {
            if input.len() < 5 {
                return MResult::err("Insufficient bytes for 4-byte varint");
            }
            let value = u32::from_le_bytes([input[1], input[2], input[3], input[4]]) as usize;
            MResult::ok((value, 5))
        }
        0xff => {
            if input.len() < 9 {
                return MResult::err("Insufficient bytes for 8-byte varint");
            }
            let value = u64::from_le_bytes([
                input[1], input[2], input[3], input[4], input[5], input[6], input[7], input[8],
            ]) as usize;
            MResult::ok((value, 9))
        }
    }
}
pub struct Transaction {
    version: u32,
    inputs: List<TxIn>,
    outputs: List<TxOut>,
    txid: String,
    locktime: u32,
}

impl Transaction {
    pub fn serialize(&self) -> String {
        let mut buffer: Vec<u8> = Vec::new();

        let version: u32 = 0x01000000;
        buffer.extend(version.to_le_bytes());

        let input_count = encode_varint(self.inputs.len());
        buffer.extend(input_count);

        self.inputs.iter().for_each(|txin| {
            buffer.extend(txin.serialize());
            buffer.extend(0xffffffff_u32.to_le_bytes()); // nSequence
        });

        let output_count = encode_varint(self.outputs.len());
        buffer.extend(output_count);

        self.outputs.iter().for_each(|txout| {
            buffer.extend(txout.serialize());
        });

        let locktime: u32 = self.locktime;
        buffer.extend(locktime.to_le_bytes());

        hex::encode(buffer)
    }
}

pub struct TxIn {
    prev_txid: String,  // 32 bytes
    out: u64,           // 8 bytes
    script_sig: String, // to spend the output - variable length
}

impl TxIn {
    pub fn new(prev_txid: String, out: u64, script_sig: String) -> Self {
        TxIn {
            prev_txid,
            out,
            script_sig,
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        let prev_txid_bytes =
            hex::decode(&self.prev_txid).expect("Inalid hex string for prev_txid");
        buffer.extend(prev_txid_bytes.iter().rev()); // txid encoded in little endian

        buffer.extend(self.out.to_le_bytes());

        let script_sig_len = self.script_sig.len();
        buffer.extend(encode_varint(script_sig_len));

        let script_sig_bytes =
            hex::decode(&self.script_sig).expect("Invalid hex string for script_sig");
        buffer.extend(script_sig_bytes);

        buffer
    }
}

#[derive(Clone, Debug)]
pub struct TxOut {
    public_address: String,
    satoshis: u64,
    // 1 btc = 10^8 satoshis, in total 10^8 * 21 * 10^6 = 2.1 * 10^15
    // maximum value of u64 is greater than 10^19
    // so u64 is enough to store all valid satoshis
}

impl TxOut {
    pub fn new(public_address: String, satoshis: u64) -> Self {
        TxOut {
            public_address,
            satoshis,
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        let satoshis = self.satoshis.to_le_bytes();
        buffer.extend(satoshis);

        let script_pubkey_len = self.public_address.len() / 2;
        buffer.extend(encode_varint(script_pubkey_len));

        let script_pubkey_bytes =
            hex::decode(&self.public_address).expect("Invalid hex string for script_pubkey");
        buffer.extend(script_pubkey_bytes);

        buffer
    }

    pub fn deserialize(input: &[u8]) -> MResult<TxOut, &'static str> {
        if input.len() < 8 {
            return MResult::err("Insufficient data for TxOut");
        }

        // First 8 bytes are satoshis
        let satoshis = u64::from_le_bytes(input[0..8].try_into().unwrap());

        // Next is the varint for script_pubkey length
        let (script_pubkey_len, varint_size) = match decode_varint(&input[8..]) {
            MResult::Ok((value, size)) => (value, size),
            MResult::Err(e) => {
                println!("Error: {}", e);
                panic!("Error decoding varint");
            }
        };

        let script_start = 8 + varint_size;
        let script_end = script_start + script_pubkey_len;

        if input.len() < script_end {
            return MResult::err("Insufficient data for script_pubkey");
        }

        // The rest is the script_pubkey (public address)
        let script_pubkey = hex::encode(&input[script_start..script_end]);

        MResult::ok(TxOut {
            satoshis,
            public_address: script_pubkey,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;
    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for TxOut {
        fn arbitrary(g: &mut Gen) -> Self {
            let satoshis = u64::arbitrary(g);
            let address_length = u8::arbitrary(g) as usize % 64 + 1;
            let public_address = (0..address_length)
                .map(|_| format!("{:02x}", u8::arbitrary(g)))
                .collect::<String>();

            TxOut::new(public_address, satoshis)
        }
    }

    quickcheck! {
        fn test_sha256(input: String) -> bool {
            let hash = sha256(&input);
            let expected = format!("{:x}", Sha256::digest(input.as_bytes()));
            hash == expected
        }

        fn test_sha256_length(input: String) -> bool {
            let hash = sha256(&input);
            hash.len() == 64
        }

        fn test_encode_decode_varint(n: usize) -> bool {
            let encoded = encode_varint(n);
            let (decoded, _) = decode_varint(&encoded).unwrap();
            n == decoded
        }

        fn test_txout_serialize_deserialize(txout: TxOut) -> bool {
            let serialized = txout.serialize();
            let deserialized = TxOut::deserialize(&serialized).unwrap();
            txout.satoshis == deserialized.satoshis && txout.public_address == deserialized.public_address
        }
    }
}

// Try to include bitcoin related functionalities like serialization, computing addresses etc.,
// You can add your own methods for different types and associated unit tests
