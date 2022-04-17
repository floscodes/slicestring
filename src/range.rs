pub trait Range {
    fn get_indices(&self, s:&str) -> (usize, usize);
}


//Range-trait for usize
impl Range for std::ops::Range<usize> {

    fn get_indices(&self, s: &str) -> (usize, usize) {
        #[allow(unused)]
        let mut x: usize = 0;
        let mut y: usize = 0;

        if (self.end as i32) < 0 {
            y = s.len() - (self.end as i32 * -1) as usize;
        } else {
            y = self.end;
        }

        if (self.start as i32) > 0 {
            x = self.start;
        }

        (x, y)
    }
}

impl Range for std::ops::RangeTo<usize> {

    fn get_indices(&self, _s:&str) -> (usize, usize) {
            (0, self.end) 
    }
}

impl Range for std::ops::RangeFrom<usize> {
    fn get_indices(&self, s:&str) -> (usize, usize) {
        if (self.start as i32) < 0 {
            (0, s.len())
        } else {
        (self.start, s.len())
        }
    }
}


//get-indices-function for crate-use
pub (in crate) fn get_indices(s: &str, r: impl Range) -> (usize, usize) {
    r.get_indices(s)
}