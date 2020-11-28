use crate::interface::io::CIState;
use std::env;
use std::io;
use sysinfo::{ProcessExt, System, SystemExt};
mod memory;
mod interface;

fn getpids(name: &str) -> Vec<i32> {
    let s = System::new_all();
    let mut pids: Vec<i32> = Vec::new();

    for p in s.get_process_by_name(name) {
        pids.push(p.pid());
    }

    return pids;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let pid = match args[1].as_str() {
        "bin" => getpids(args[2].as_str())[0],
        "pid" => args[2].parse().unwrap(),
        _ => panic!("Idk what to do now..."),
    };
    let mut interface = CIState::new(pid);
    interface.command_loop();
    
    /*let pat = [0x41, 0x41, 0x41, 0x41];

    for region in mem.maps.entries {
        if !region.perm_write || region.inode != 0 {
            continue;
        }
        println!(
            "Occurences of {:?} in {} from {:#x} to {:#x}, length: {:#x}",
            pat,
            region.path,
            region.addr_from,
            region.addr_to,
            region.addr_to - region.addr_from
        );
        let matches = region.find_pattern(&mut mem.memfile, &pat);
        for relm in matches {
            let m = region.addr_absolute(relm);
            let mut buf = [0 as u8; 16];
            mem.memfile.seek(SeekFrom::Start(m as u64))?;
            mem.memfile.read(&mut buf)?;
            println!("{:#x} - {:#x}: {}", m, m + 16, pretty_hex::pretty_hex(&buf));
        }
    }*/

    Ok(())
}
