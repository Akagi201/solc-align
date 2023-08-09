#![allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub struct Declaration {
    pub storage_type: StorageType,
    pub line: String,
    pub mod_size: u16, // size mod by 256
    pub weight: u16,
}

/// StorageTypes: ref: https://docs.soliditylang.org/en/latest/types.html
#[derive(Clone, Debug, PartialEq)]
pub enum StorageType {
    Bool,
    Integer(u16),
    Address,
    FixedBytes(u16),
    Bytes,
    String,
    Array(Box<StorageType>),
    Struct,
    Mapping,
    Unknown, // for example, custom struct
}

impl StorageType {
    /// size mod 32
    pub fn mod_size(&self) -> u16 {
        match self {
            StorageType::Bool => 1,
            StorageType::Integer(n) => (n / 8 - 1) % 32 + 1,
            StorageType::Address => 20,
            StorageType::FixedBytes(n) => (n - 1) % 32 + 1,
            StorageType::Bytes => 32,
            StorageType::String => 32,
            StorageType::Array(t) => (0 + t.mod_size() - 1) % 32 + 1,
            StorageType::Struct => 32,
            StorageType::Mapping => 32,
            StorageType::Unknown => 32,
        }
    }

    /// order weight order by ascending order
    pub fn weight(&self) -> u16 {
        match self {
            StorageType::Unknown => 1,
            StorageType::Struct => 2,
            StorageType::Mapping => 3,
            StorageType::Address => 4,
            StorageType::Integer(256) => 5,
            StorageType::Bytes => 6,
            StorageType::String => 7,
            StorageType::Array(_t) => 8,
            StorageType::FixedBytes(_n) => 9,
            StorageType::Integer(_n) => 10,
            StorageType::Bool => 11,
        }
    }
}

impl Declaration {
    pub fn from_str(s: &str) -> Declaration {
        let s = s.trim();
        if s.starts_with("bool") {
            let t = StorageType::Bool;
            Declaration {
                storage_type: t.clone(),
                line: s.to_string(),
                mod_size: t.mod_size(),
                weight: 0,
            }
        } else if s.starts_with("int") {
            let w = s.split_whitespace().collect::<Vec<&str>>()[0];
            let n = w.strip_prefix("int").unwrap();
            let t: StorageType;
            if n.trim().len() == 0 {
                t = StorageType::Integer(256);
            } else {
                let n = n.parse::<u16>().unwrap();
                t = StorageType::Integer(n);
            }
            Declaration {
                storage_type: t.clone(),
                line: s.to_string(),
                mod_size: t.mod_size(),
                weight: 0,
            }
        } else if s.starts_with("uint") {
            let w = s.split_whitespace().collect::<Vec<&str>>()[0];
            let n = w.strip_prefix("uint").unwrap();
            let t: StorageType;
            if n.trim().len() == 0 {
                t = StorageType::Integer(256);
            } else {
                let n = n.parse::<u16>().unwrap();
                t = StorageType::Integer(n);
            }
            Declaration {
                storage_type: t.clone(),
                line: s.to_string(),
                mod_size: t.mod_size(),
                weight: 0,
            }
        } else if s.starts_with("address") {
            let t = StorageType::Address;
            Declaration {
                storage_type: t.clone(),
                line: s.to_string(),
                mod_size: t.mod_size(),
                weight: 0,
            }
        } else if s.starts_with("bytes") {
            let w = s.split_whitespace().collect::<Vec<&str>>()[0];
            let n = w.strip_prefix("bytes").unwrap();
            let t: StorageType;
            if n.trim().len() == 0 {
                t = StorageType::Bytes;
            } else {
                let n = n.parse::<u16>().unwrap();
                t = StorageType::FixedBytes(n);
            }
            Declaration {
                storage_type: t.clone(),
                line: s.to_string(),
                mod_size: t.mod_size(),
                weight: 0,
            }
        } else if s.starts_with("string") {
            let t = StorageType::String;
            Declaration {
                storage_type: t.clone(),
                line: s.to_string(),
                mod_size: t.mod_size(),
                weight: 0,
            }
        } else if s.starts_with("struct") {
            let t = StorageType::Struct;
            Declaration {
                storage_type: t.clone(),
                line: s.to_string(),
                mod_size: t.mod_size(),
                weight: 0,
            }
        } else if s.starts_with("mapping") {
            let t = StorageType::Mapping;
            Declaration {
                storage_type: t.clone(),
                line: s.to_string(),
                mod_size: t.mod_size(),
                weight: 0,
            }
        } else {
            let t = StorageType::Unknown;
            Declaration {
                storage_type: t.clone(),
                line: s.to_string(),
                mod_size: t.mod_size(),
                weight: 0,
            }
        }
    }
}
