use crate::hexo::state::Coord;

#[derive(Clone, Copy)]
pub struct CoordSet {
    data: [u64; 64]
}

impl CoordSet {
    fn translate(c: Coord) -> Coord {
        return (c.0 + 31, c.1 + 31);
    }

    pub fn insert(&mut self, c: Coord) {
        let nc = Self::translate(c);
        self.data[nc.0 as usize] |= 1 << nc.1
    }

    pub fn remove(&mut self, c: Coord) {
        let nc = Self::translate(c);
        self.data[nc.0 as usize] &= !(1 << nc.1);
    }

    pub fn contains(&self, c: Coord) -> bool {
        let nc = Self::translate(c);
        return self.data[nc.0 as usize] & (1 << nc.1) > 0;
    }

    pub fn new() -> CoordSet {
        CoordSet { data: [0; 64] }
    }
}