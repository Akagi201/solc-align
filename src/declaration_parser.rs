#![allow(unused)]

use crate::utils::{get_full_type, parse_array_size};
#[derive(Clone, Debug, PartialEq)]
pub struct Declaration {
    pub storage_type: StorageType,
    pub line: String,
    pub mod_size: u32, // size mod by 32
    pub weight: u32,
}

/// StorageTypes: ref: https://docs.soliditylang.org/en/latest/types.html
#[derive(Clone, Debug, PartialEq)]
pub enum StorageType {
    Bool,
    Integer(u32),
    Address,
    Bytes(u32),
    String,
    Array(Box<StorageType>, u32),
    Struct,
    Mapping,
    Unknown, // for example, custom struct
}

impl StorageType {
    /// size mod 32
    pub fn mod_size(&self) -> u32 {
        match self {
            StorageType::Bool => 1,
            StorageType::Integer(n) => (n - 1) % 32 + 1,
            StorageType::Address => 20,
            StorageType::Bytes(n) => (n - 1) % 32 + 1,
            StorageType::String => 32,
            StorageType::Array(t, n) => (t.mod_size() * n - 1) % 32 + 1,
            StorageType::Struct => 32,
            StorageType::Mapping => 32,
            StorageType::Unknown => 32,
        }
    }

    pub fn from_str(type_name: &str, type_size: u32, array_size: u32) -> Self {
        if array_size != 0 {
            return StorageType::Array(
                Box::new(StorageType::from_str(type_name, type_size, 0)),
                array_size,
            );
        }
        match type_name {
            "bool" => StorageType::Bool,
            "int" | "uint" => StorageType::Integer(type_size),
            "address" => StorageType::Address,
            "bytes" => StorageType::Bytes(type_size),
            "string" => StorageType::String,
            "mapping" => StorageType::Mapping,
            _ => StorageType::Unknown,
        }
    }

    /// order weight order by ascending order
    pub fn weight(&self) -> u32 {
        match self {
            StorageType::Unknown => 1,
            StorageType::Struct => 2,
            StorageType::Mapping => 3,
            StorageType::Integer(32) => 4,
            StorageType::Bytes(32) => 5,
            StorageType::String => 6,
            StorageType::Array(_t, _n) => 7,
            StorageType::Bytes(_n) => 8,
            StorageType::Address => 9,
            StorageType::Integer(_n) => 10,
            StorageType::Bool => 11,
        }
    }
}

impl Declaration {
    pub fn from_str(s: &str) -> Declaration {
        let input = get_full_type(s);
        let (type_name, type_size, array_size) = parse_array_size(input);
        let storage_type = StorageType::from_str(&type_name, type_size, array_size);
        Declaration {
            storage_type: storage_type.clone(),
            line: s.to_string(),
            mod_size: storage_type.mod_size(),
            weight: storage_type.weight(),
        }
    }
}
