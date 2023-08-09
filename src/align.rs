use std::collections::HashSet;

use crate::declaration_parser::Declaration;

pub fn align_32bytes(decls: &[Declaration]) -> Vec<Declaration> {
    let mut result = Vec::new();
    let mut included = HashSet::new();
    for (i, v) in decls.iter().enumerate() {
        result.push(decls[i].clone());
        if v.storage_type.mod_size() == 32 {
            continue;
        } else {
            let j = i + 1;
            let mut size = v.storage_type.mod_size();
            for j in j..decls.len() {
                if included.contains(&j) {
                    continue;
                }
                if size + decls[j].storage_type.mod_size() <= 32 {
                    size += decls[j].storage_type.mod_size();
                    included.insert(j);
                    result.push(decls[j].clone());
                } else {
                    continue;
                }
            }
        }
    }
    result
}
