//Range-trait for i32
pub trait Range {
    fn get_indices(&self, s:&str) -> (usize, usize);
}

impl Range for std::ops::Range<i32> {

    fn get_indices(&self, s: &str) -> (usize, usize) {
        #[allow(unused)]
        let mut x: usize = 0;
        let mut y: usize = 0;

        if self.end < 0 {
            y = s.len() - (self.end * -1) as usize;
        } else {
            y = self.end as usize;
        }

        if self.start > 0 {
            x = self.start as usize;
        }

        (x, y)
    }
}

impl Range for std::ops::RangeTo<i32> {

    fn get_indices(&self, s:&str) -> (usize, usize) {
        if self.end < 0 {
            (0, s.len() - (self.end * -1) as usize)
        } else {
            (0, self.end as usize)
        }
    }
}

impl Range for std::ops::RangeFrom<i32> {
    fn get_indices(&self, s:&str) -> (usize, usize) {
        if self.start < 0 {
            (0, s.len())
        } else {
        (self.start as usize, s.len())
        }
    }
}

impl Range for std::ops::RangeFull {
    fn get_indices(&self, s:&str) -> (usize, usize) {
        (0, s.len())
    }
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