

use crate::memory::maps::MemMap;
use std::fs::File;

pub struct ProcMem {
    pub maps: MemMap,
    pub memfile: File,
}

impl ProcMem {
    pub fn of_pid(pid: i32) -> ProcMem {
        let maps = MemMap::of_pid(pid);
        let f = File::open(format!("/proc/{}/mem",pid)).unwrap();

        ProcMem {
            maps: maps,
            memfile: f
        }
    }

    
}