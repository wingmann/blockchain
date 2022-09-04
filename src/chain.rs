use crate::block::Block;
use crate::block_header::BlockHeader;
use crate::transaction::Transaction;

use indicatif::ProgressBar;
use sha2::{Digest, Sha256};
use std::{fmt::Write, thread, time::SystemTime};
use time::Duration;

pub(crate) struct Chain {
    chain: Vec<Block>,
    current_transaction: Vec<Transaction>,
    difficulty: u32,
    miner_address: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_address: String, difficulty: u32) -> Self {
        let mut chain = Self {
            chain: vec![],
            current_transaction: vec![],
            difficulty,
            miner_address,
            reward: 50.0,
        };
        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.current_transaction.push(Transaction {
            sender,
            receiver,
            amount,
        });
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = deificulty;
        true
    }

    pub fn update_reward(&mut self) -> bool {
        self.reward = self.reward / 2f32;
        println!("New reward: {}", self.reward);
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = BlockHeader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            previous_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_transaction = Transaction {
            sender: "Root".to_string(),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![],
        };

        block.transactions.push(reward_transaction);
        block.transactions.append(&mut self.current_transaction);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());

        Chain::proof_of_work(&mut block.header);

        println!("last {:#?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(current_transaction: Vec<Transaction>) -> String {
        let mut merkle = Vec::<String>::new();
        for transaction in &current_transaction {
            let hash = Chain::hash(transaction);
            merkle.push(hash);
        }
        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }
        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }

        merkle.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut BlockHeader) {
        println!();
        let difficulty = header.difficulty as u64;
        let pb = ProgressBar::new(1024);
        let delta = 8 / difficulty;
        let handle = std::thread::spawn(move || {
            for _ in 0..(1024 / delta) {
                pb.inc(delta);
                thread::sleep(std::time::Duration::from_millis(difficulty * 10));
            }
            pb.finish_with_message("done")
        });
        let mut m = "".to_string();
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            header.nonce += 1;
            if let val = slice.parse::<u32>() {
                if val == 0 {
                    m = hash;
                    break;
                }
            }
        }
        handle.join().unwrap();
        print!("\nBlock hash: {}\n\n", m);
    }

    pub fn hash(item: &impl serde::Serialize) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.input(input.as_bytes());
        let res = hasher.result();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = "".to_string();
        for b in vec_res {
            write!(&mut s, "{b:x}").unwrap();
        }
        s
    }
}
