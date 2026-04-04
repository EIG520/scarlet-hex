// Evil bithack file
use crate::hexo::state::Coord;

#[derive(Clone, Copy)]
pub struct CoordSet {
    data: [u64; 64],
    data1: [u64; 64],
    data2: [u64; 64]
}

impl CoordSet {
    fn translate(c: Coord) -> Coord {
        return (c.0 + 31, c.1 + 31);
    }

    pub fn insert(&mut self, c: Coord) {
        // println!("{c:?}");

        let nc = Self::translate(c);
        self.data[nc.0 as usize] |= 1 << nc.1;
        self.data1[nc.1 as usize] |= 1 << nc.0;
        self.data2[(nc.1 + nc.0 - 31) as usize] |= 1 << nc.1;
    }

    pub fn remove(&mut self, c: Coord) {
        let nc = Self::translate(c);
        self.data[nc.0 as usize] &= !(1 << nc.1);
        self.data1[nc.1 as usize] &= !(1 << nc.0);
        self.data2[(nc.1 + nc.0 - 31) as usize] &= !(1 << nc.1);
    }

    pub fn contains(&self, c: Coord) -> bool {
        let nc = Self::translate(c);
        return (self.data[nc.0 as usize] >> nc.1) & 1 == 1;
    }

    pub fn count_1r(row: u64, idx: u32, d: u32) -> u32 {
        return ((row << (64 - idx)) & !((1 << (64 - d + 1)) - 1)).count_ones();
    }

    pub fn count_1l(row: u64, idx: u32, d: u32) -> u32 {
        return ((row >> idx) & ((1 << (d + 1)) - 1)).count_ones();
    }

    pub fn count_1_0r(&self, c: Coord, d: u32) -> u32 {
        let nc = Self::translate(c);

        return Self::count_1r(self.data[nc.0 as usize], nc.1 as u32, d);
    }

    pub fn count_1_1r(&self, c: Coord, d: u32) -> u32 {
        let nc = Self::translate(c);

        return Self::count_1r(self.data1[nc.1 as usize], nc.0 as u32, d);
    }

    pub fn count_1_2r(&self, c: Coord, d: u32) -> u32 {
        let nc = Self::translate(c);

        return Self::count_1r(self.data2[(nc.1 + nc.0 - 31) as usize], nc.1 as u32, d);
    }

    pub fn count_1_0l(&self, c: Coord, d: u32) -> u32 {
        let nc = Self::translate(c);

        return Self::count_1l(self.data[nc.0 as usize], nc.1 as u32, d);
    }

    pub fn count_1_1l(&self, c: Coord, d: u32) -> u32 {
        let nc = Self::translate(c);

        return Self::count_1l(self.data1[nc.1 as usize], nc.0 as u32, d);
    }

    pub fn count_1_2l(&self, c: Coord, d: u32) -> u32 {
        let nc = Self::translate(c);

        return Self::count_1l(self.data2[(nc.1 + nc.0 - 31) as usize], nc.1 as u32, d);
    }

    pub fn get_0_run(row: u64, idx: u32) -> u32 {
        return Self::get_0_runr(row, idx) + Self::get_0_runl(row, idx);
    }

    pub fn get_0_runr(row: u64, idx: u32) -> u32 {
        let runr = (row << (64 - idx)).leading_zeros();

        return runr;
    }

    pub fn get_0_runl(row: u64, idx: u32) -> u32 {
        let runl = (row >> idx).trailing_zeros();

        return runl;
    }

    pub fn get_0_run_0(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_run(self.data[nc.0 as usize], nc.1 as u32);
    }

    pub fn get_0_run_1(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_run(self.data1[nc.1 as usize], nc.0 as u32);
    }

    pub fn get_0_run_2(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_run(self.data2[(nc.1 + nc.0 - 31) as usize], nc.1 as u32);
    }

    pub fn get_0_run_0r(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_runr(self.data[nc.0 as usize], nc.1 as u32);
    }

    pub fn get_0_run_1r(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_runr(self.data1[nc.1 as usize], nc.0 as u32);
    }

    pub fn get_0_run_2r(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_runr(self.data2[(nc.1 + nc.0 - 31) as usize], nc.1 as u32);
    }

    pub fn get_0_run_0l(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_runl(self.data[nc.0 as usize], nc.1 as u32);
    }

    pub fn get_0_run_1l(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_runl(self.data1[nc.1 as usize], nc.0 as u32);
    }

    pub fn get_0_run_2l(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_runl(self.data2[(nc.1 + nc.0 - 31) as usize], nc.1 as u32);
    }

    pub fn get_1_run_0(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_run(!self.data[nc.0 as usize], nc.1 as u32);
    }

    pub fn get_1_run_1(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_run(!self.data1[nc.1 as usize], nc.0 as u32);
    }

    pub fn get_1_run_2(&self, c: Coord) -> u32 {
        let nc = Self::translate(c);
        return Self::get_0_run(!self.data2[(nc.1 + nc.0 - 31) as usize], nc.1 as u32);
    }

    pub fn get_ones(&self) -> Vec<Coord> {
        let mut cvec = vec![];
        for (idx, &v) in self.data.iter().enumerate() {
            let mut t = v;

            while t > 0 {
                let pos = t.trailing_zeros();

                cvec.push((idx as i16 - 31, pos as i16 - 31));

                t = t & (t-1);
            }
        }

        // println!("{cvec:?}");

        cvec
    }

    pub fn new() -> CoordSet {
        CoordSet { data: [0; 64], data1: [0; 64], data2: [0; 64] }
    }
}
