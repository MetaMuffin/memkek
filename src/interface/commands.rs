use crate::interface::helper::MemRegionContraints;
use crate::CIState;

#[derive(Debug)]
pub enum CommandErr {
    CommandNotFound(),
    FlagUnknown(char),
    InvalidMapIndex(String),
}

impl CIState {
    pub fn list_maps(&mut self) -> Result<(), CommandErr> {
        self.mem.maps.update_mapping(self.pid);
        for (i, m) in self.mem.maps.entries.iter().enumerate() {
            let mut perms = String::new();
            if m.perm_read {
                perms += "r";
            } else {
                perms += "-"
            }
            if m.perm_write {
                perms += "w";
            } else {
                perms += "-"
            }
            if m.perm_execute {
                perms += "x";
            } else {
                perms += "-"
            }
            println!(
                "[{}] {} {:#x}-{:#x} {} {}",
                i, perms, m.addr_from, m.addr_to, m.inode, m.path
            );
        }
        Ok(())
    }

    pub fn use_regions(&mut self, args: &[&str]) -> Result<(), CommandErr> {
        let mut ct = MemRegionContraints {
            indecies: None,
            perm_read: None,
            perm_write: None,
            perm_execute: None,
            inode: None,
        };
        for a in args {
            if a.starts_with("@") {
                for c in a.chars() {
                    match c {
                        'r' => ct.perm_read = Some(true),
                        'R' => ct.perm_read = Some(false),
                        'w' => ct.perm_write = Some(true),
                        'W' => ct.perm_write = Some(false),
                        'x' => ct.perm_execute = Some(true),
                        'X' => ct.perm_execute = Some(false),
                        'i' => ct.inode = Some(true),
                        'I' => ct.inode = Some(false),
                        _ => return Err(CommandErr::FlagUnknown(c)),
                    }
                }
            } else {
                match a.parse::<usize>() {
                    Ok(n) => match ct.indecies {
                        Some(v) => v.push(n),
                        None => ct.indecies = vec![n],
                    },
                    Err(e) => return Err(CommandErr::InvalidMapIndex(String::from(*a))),
                }
            }
        }
        self.region_constraints = Some(ct);
        Ok(())
    }
}
