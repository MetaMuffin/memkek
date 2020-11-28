use crate::interface::helper::MemRegionContraints;
use crate::memory::region::MemRegion;
use crate::CIState;

#[derive(Debug)]
pub enum CommandErr {
    CommandNotFound(),
    FlagUnknown(char),
    InvalidMapIndex(String),
    CantParseNumber(String),
    InvalidSyntax(),
}

impl CIState {
    pub fn list_maps(&mut self) -> Result<(), CommandErr> {
        self.mem.maps.update_mapping(self.pid);
        for m in self.mem.maps.entries.iter() {
            println!("{}", m)
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
                for c in a[1..].chars() {
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
                if a.starts_with("-") {

                    if a.contains("h") {
                        let mut f = false;
                        for m in &self.mem.maps.entries {
                            if m.path == "[heap]" {
                                f = true;
                                match &mut ct.indecies {
                                    Some(v) => v.push(m.index),
                                    None => ct.indecies = Some(vec![m.index]),
                                }
                            }
                        }
                        if !f {
                            println!("* Ignoring heap flag because no heap could be detected.")
                        }
                    }
                    if a.contains("s") {
                        let mut f = false;
                        for m in &self.mem.maps.entries {
                            if m.path == "[stack]" {
                                f = true;
                                match &mut ct.indecies {
                                    Some(v) => v.push(m.index),
                                    None => ct.indecies = Some(vec![m.index]),
                                }
                            }
                        }
                        if !f {
                            println!("* Ignoring heap flag because no heap could be detected.")
                        }
                    }
                } else {
                    match a.parse::<usize>() {
                        Ok(n) => match &mut ct.indecies {
                            Some(v) => v.push(n),
                            None => ct.indecies = Some(vec![n]),
                        },
                        Err(_) => return Err(CommandErr::InvalidMapIndex(String::from(*a))),
                    }
                }
            }
        }
        self.region_constraints = Some(ct);
        Ok(())
    }

    pub fn get_regions(&self) -> Vec<MemRegion> {
        match &self.region_constraints {
            None => {
                let mut ms = Vec::new();
                for m in &self.mem.maps.entries {
                    ms.push(m.clone());
                }
                return ms;
            },
            Some(c) => {
                let a = c.active_maps(&self.mem.maps.entries);
                let mut b = vec![];
                for e in a {b.push(e.clone())}
                return b;
            },
        };
    }

    pub fn show_used_regions(&self) -> Result<(), CommandErr> {
        for m in self.get_regions() {
            println!("{}", m);
        }
        Ok(())
    }

    

}
