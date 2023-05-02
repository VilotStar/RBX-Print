pub struct ASLR {
    base: usize,
    rec_base: usize
}

impl ASLR {
    pub fn new(base: usize, rec_base: usize) -> Self {
        Self {
            base,
            rec_base
        }
    }

    pub fn aslr(&self, addr: usize) -> usize {
        (addr - self.rec_base) + self.base
    }
}
