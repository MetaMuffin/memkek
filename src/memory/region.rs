
use std::io::SeekFrom;
use std::fs::File;
use std::io::Seek;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MemRegion {
    pub addr_from: usize,
    pub addr_to: usize,
    pub offset: usize,
    pub perm_read: bool,
    pub perm_write: bool,
    pub perm_execute: bool,
    pub inode: u64,
    pub path: String,
    pub index: usize,
}

impl MemRegion {
    pub fn find_pattern(&self, file: &mut File, pat: &[u8]) -> Vec<usize> {
        let dest = (self.addr_to - self.addr_from) as usize;
        
        file.seek(SeekFrom::Start(self.addr_from as u64)).unwrap();
        let mut buf = [0 as u8; 4096];
        let mut bytes_left = file.read(&mut buf).unwrap();
        
        let mut occ: Vec<usize> = vec![]; // Final search results
        let mut occ_partial: Vec<usize> = vec![]; // Absolute index of matches that are correct till now
        let mut read_offset = 0;

        for i in 0..dest {
            if bytes_left <= 0 { // If there are no bytes left to read in the buffer, read more
                bytes_left = file.read(&mut buf).unwrap();
                if bytes_left == 0 {
                    println!("no bytes left");
                    break;
                }
                read_offset += bytes_left; // Also add the bytes to an offset to later corrent absolute indecies to be buffer-relative
            }
            bytes_left -= 1;
            // Move all matches that are done to the final matches vec
            occ_partial.retain(|ocp| {
                if ocp + pat.len() <= i {
                    occ.push(*ocp);
                    false
                } else { true }
            });
            // Only retain partial matches that still match the current new byte.
            occ_partial.retain(|ocp| {
                // println!("{} - {}", i, ocp);
                buf[i - read_offset] == pat[i - ocp]
            });
            // If current byte matches the start of the pattern, add it to the partial matches
            if buf[i - read_offset] == pat[0] {
                occ_partial.push(i);
            }
        }
        return occ
    }

    pub fn addr_absolute(&self, addr: usize) -> usize {
        if addr + self.addr_from > self.addr_to { panic!("Trying to access memory out of bounds from this region"); }
        addr + self.addr_from
    }

}


