use crate::memory::region::MemRegion;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[derive(Debug)]
pub struct MemMap {
    pub entries: Vec<MemRegion>,
}

impl MemMap {
    pub fn of_pid(pid: i32) -> MemMap {
        let mut map = MemMap { entries: Vec::new() };
        map.update_mapping(pid);
        return map;
    }

    pub fn update_mapping(&mut self, pid: i32) {
        self.entries = vec![];
        if let Ok(lines) = read_lines(format!("/proc/{}/maps", pid)) {
            for (i,rline) in lines.enumerate() {
                if let Ok(line) = rline {
                    self.entries.push(Self::parse_region(&line,i))
                }
            }
        }
    }

    pub fn parse_region(raw: &String, index: usize) -> MemRegion {
        let re =
            Regex::new(r"([0-9a-f]+)-([0-9a-f]+) (.{4}) ([0-9a-f]+) ..:.. (\d+) +(.*)").unwrap();
        let caps = re.captures(raw).unwrap();
        let cap_perm = caps.get(3).unwrap().as_str();
        MemRegion {
            index: index,

            addr_from: usize::from_str_radix(caps.get(1).unwrap().as_str(), 16).unwrap(),
            addr_to: usize::from_str_radix(caps.get(2).unwrap().as_str(), 16).unwrap(),
            offset: usize::from_str_radix(caps.get(4).unwrap().as_str(), 16).unwrap(),

            perm_read: cap_perm.find("r").is_some(),
            perm_write: cap_perm.find("w").is_some(),
            perm_execute: cap_perm.find("x").is_some(),
            
            inode: caps.get(5).unwrap().as_str().parse().unwrap(),
            path: String::from(caps.get(6).unwrap().as_str()),
        }
    }
}
