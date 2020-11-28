use crate::memory::region::MemRegion;

pub struct MemRegionContraints {
    pub perm_read: Option<bool>,
    pub perm_write: Option<bool>,
    pub perm_execute: Option<bool>,
    pub inode: Option<bool>,
    pub indecies: Option<Vec<usize>>,
}

impl MemRegionContraints {
    pub fn active_maps<'a>(&self, maps: &'a Vec<MemRegion>) -> Vec<&'a MemRegion> {
        let mut a = vec![];
        for m in maps {
            if self.check_predicate(&m) { a.push(m); }
        }
        return a;
    }

    pub fn check_predicate(&self, reg: &MemRegion) -> bool {
        if let Some(p) = self.perm_read {
            if reg.perm_read != p { return false; }
        }
        if let Some(p) = self.perm_write {
            if reg.perm_write != p { return false; }
        }
        if let Some(p) = self.perm_execute {
            if reg.perm_execute != p { return false; }
        }
        if let Some(p) = self.perm_execute {
            if (reg.inode != 0) != p { return false; }
        }
        if let Some(i) = &self.indecies {
            if !i.contains(&reg.index) { return false; }
        }
        return true;
    }
}
