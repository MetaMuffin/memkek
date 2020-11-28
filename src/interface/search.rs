use crate::interface::commands::CommandErr;
use crate::interface::search::SearchDataTypes::*;
use crate::CIState;


impl CIState {
    pub fn command_search(&mut self, args: &[&str]) -> Result<(), CommandErr> {
        let a = args[0];
        let mut ptype: Option<SearchDataTypes> = None;
        if a.starts_with("3i") {
            ptype = Some(I3(match a[2..].parse() {
                Ok(v) => v,
                Err(_) => return Err(CommandErr::CantParseNumber(String::from(a))),
            }))
        } else if a.starts_with("6i") {
            ptype = Some(I6(match a[2..].parse() {
                Ok(v) => v,
                Err(_) => return Err(CommandErr::CantParseNumber(String::from(a))),
            }))
        } else if a.starts_with("3u") {
            ptype = Some(U3(match a[2..].parse() {
                Ok(v) => v,
                Err(_) => return Err(CommandErr::CantParseNumber(String::from(a))),
            }))
        } else if a.starts_with("6u") {
            ptype = Some(U6(match a[2..].parse() {
                Ok(v) => v,
                Err(_) => return Err(CommandErr::CantParseNumber(String::from(a))),
            }))
        } else if a.starts_with("\"") {
            ptype = Some(Str(a[1..].to_string()))
        } else if a.starts_with("0x") {
            let rbytes = hex::decode(a[2..].to_string());
            match rbytes {
                Ok(bytes) => ptype = Some(V(bytes)),
                Err(_) => return Err(CommandErr::CantParseNumber(a[2..].to_string())),
            }
        }
        if let None = ptype {
            return Err(CommandErr::InvalidSyntax());
        }
        let pat = data_type_to_pat(ptype.unwrap());
        
        for reg in self.get_regions() {
            println!("REGION: {}", reg);
            let res = reg.find_pattern(&mut self.mem.memfile,&pat);
            for rel_addr in res {
                let abs_addr = reg.addr_absolute(rel_addr);
                println!("{:#x}-{:#x}",abs_addr,abs_addr + pat.len());
            }
        }

        Ok(())
    }
}

pub fn data_type_to_pat(s: SearchDataTypes) -> Vec<u8> {
    return match s {
        V(x) => x,
        I3(x) => Vec::from(x.to_be_bytes()),
        I6(x) => Vec::from(x.to_be_bytes()),
        U3(x) => Vec::from(x.to_be_bytes()),
        U6(x) => Vec::from(x.to_be_bytes()),
        Str(s) => Vec::from(s.as_bytes()),
    };
}

pub enum SearchDataTypes {
    Str(String),
    I3(i32),
    I6(i64),
    U3(u32),
    U6(u64),
    V(Vec<u8>),
}
