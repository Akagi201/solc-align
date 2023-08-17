use std::collections::HashSet;

use crate::declaration_parser::Declaration;

pub fn align_32bytes(decls: &[Declaration]) -> Vec<Declaration> {
    let mut result = Vec::new();
    let mut included = HashSet::new();
    for (i, v) in decls.iter().enumerate() {
        if included.contains(&i) {
            continue;
        }
        result.push(decls[i].clone());
        included.insert(i);
        // println!("normal included: {:?}", decls[i].clone());
        if v.storage_type.mod_size() == 32 {
            continue;
        } else {
            let j = i + 1;
            let mut size = v.storage_type.mod_size();
            for k in j..decls.len() {
                if included.contains(&k) {
                    continue;
                }
                if size + decls[k].storage_type.mod_size() <= 32 {
                    size += decls[k].storage_type.mod_size();
                    included.insert(k);
                    result.push(decls[k].clone());
                    // println!("small included: {:?}", decls[k].clone());
                } else {
                    continue;
                }
            }
        }
    }
    result
}
