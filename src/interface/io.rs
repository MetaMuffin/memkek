


use crate::interface::commands::CommandErr;
use crate::interface::helper::MemRegionContraints;
use crate::memory::mem::ProcMem;
use std::io::Write;

pub struct CIState {
    pub pid: i32,
    pub mem: ProcMem,

    pub region_constraints: Option<MemRegionContraints>,
}



impl CIState {
    pub fn new(pid: i32) -> CIState {
        CIState {
            pid: pid,
            mem: ProcMem::of_pid(pid),
            region_constraints: None
        }
    }

    pub fn command_loop(&mut self) {
        loop {
            let mut s = String::new();
            print!(">");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut s).unwrap();
            
            if s.starts_with("q") { println!("bye kek"); break; }
            else if let Err(e) = self.dispatch_command(s) {
                println!("ERR: {:?}",e);
            }
        }
    }


    pub fn dispatch_command(&mut self, cmd: String) -> Result<(), CommandErr> {
        let args: Vec<_> = cmd.split_whitespace().collect();
        if args.len() <= 0 { return Ok(()); }
        let com = args[0];
        let cargs = &args[1..];
        match com {
            "maps" | "m" => self.list_maps(),
            "use_region" | "use" | "u" => self.use_regions(cargs),
            "show_used_regions" | "show_used" | "su" => self.show_used_regions(),
            "clear" | "cls" | "c" => { print!("\x1B[2J\x1B[1;1H"); Ok(())},
            "search" | "s" => self.command_search(cargs),
            "update_maps" | "um" => { self.mem.maps.update_mapping(self.pid); return Ok(()) },
            _ => Err(CommandErr::CommandNotFound()),
        }
    }
}

