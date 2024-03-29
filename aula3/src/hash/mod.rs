use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Hash {
    modulus: u64,
    key: u64,
    nome_arq: String,
}

impl Hash {
    pub fn new(nome_arq: &str) -> Self {
        let modulus = 0xFFFFFF;
        let mut key: u64 = 0;
        let mut mult: u64 = 1;
        for ch in nome_arq.chars() {
            key += (ch as u64).wrapping_mul(mult);
            mult = mult.wrapping_mul(16) % modulus;
        }
        key %= modulus;
        println!("{:x}", key);
        Self {
            modulus,
            key,
            nome_arq: nome_arq.to_string(),
        }
    }

    pub fn executar(&self) -> io::Result<()> {
        let file = File::open(&self.nome_arq)?;
        let reader = BufReader::new(file);
        let mut hash: u64 = 0;
        let mut mult: u64 = 1;
        for line in reader.lines() {
            let line = line?;
            for ch in line.chars() {
                hash += (ch as u64).wrapping_mul(mult);
                mult = mult.wrapping_mul(16) % self.modulus;
            }
        }
        hash = (hash ^ self.key) % self.modulus;
        println!("Hash: {:x}", hash);
        Ok(())
    }
}
