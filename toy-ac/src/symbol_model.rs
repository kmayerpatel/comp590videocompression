pub trait SymbolModel<T: std::cmp::Eq> {
    fn contains(&self, s: &T) -> bool;
    fn total(&self) -> u32;
    fn interval(&self, s: &T) -> (u32, u32);
    fn lookup(&self, v: u32) -> (&T, u32, u32);
}

pub struct VectorCountSymbolModel<T: std::cmp::Eq> {
    symbols: Vec<T>,
    counts: Vec<u32>,
    total: u32,
    norm_count: u32
}

impl<T: std::cmp::Eq> VectorCountSymbolModel<T> {
    pub fn new(symbols: Vec<T>) -> Self {
        let counts: Vec<u32> = vec![1; symbols.len()];
        let length = symbols.len() as u32;
        Self {
            symbols: symbols,
            counts: counts,
            total: length,
            norm_count: 0
        }
    }

    pub fn find_index(&self, s: &T) -> usize {
        let mut idx = 0;
        while idx < self.symbols.len() {
            if self.symbols[idx] == *s {
                return idx;
            }
            idx += 1;
        }
        panic!("Symbol not found");
    }

    pub fn set_count(&mut self, s: &T, c: u32) {
        let idx = self.find_index(s);
        self.total -= self.counts[idx];
        self.counts[idx] = c;
        self.total += self.counts[idx];
        self.normalize();
    }

    pub fn incr_count(&mut self, s: &T) {
        let idx = self.find_index(s);
        self.total += 1;
        self.counts[idx] += 1;
        self.normalize();
    }

    fn normalize(&mut self) {
        // Need to prevent intervals from getting too small. 
        // This should be made configurable, but for now just hard coding
        // so that no interval can get smaller than 1/N shown below.

        while self.total >= 1000000 {    
            self.norm_count += 1;

            let mut new_total = 0;
            for i in 0..self.symbols.len() {
                self.counts[i] = if self.counts[i] < 3 {1} else {self.counts[i]/2};
                new_total += self.counts[i];
            }
            self.total = new_total;
        }
    }

}

impl<T: std::cmp::Eq> SymbolModel<T> for VectorCountSymbolModel<T> {
    fn contains(&self, s: &T) -> bool {
        return self.symbols.contains(s);
    }

    fn interval(&self, s: &T) -> (u32, u32) {
        let mut sum = 0;
        let mut idx = 0;
        while idx < self.symbols.len() {
            if self.symbols[idx] == *s {
                return (sum, sum + self.counts[idx]);
            }
            sum += self.counts[idx];
            idx += 1;
        }
        panic!("Symbol not in model.");
    }

    fn lookup(&self, v: u32) -> (&T, u32, u32) {
        if v >= self.total {
            panic!("Lookup value out of range");
        }

        let mut sum = 0;
        for i in 0..self.symbols.len() {
            let next = sum + self.counts[i];
            if v < next {
                return (&self.symbols[i], sum, next);
            }
            sum = next;
        }
        panic!("Should never happen");
    }
    
    fn total(&self) -> u32 {
        return self.total;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_test() {
        let sm = VectorCountSymbolModel::new(vec!['a', 'b', 'c', 'd', 'e']);
        assert_eq!(sm.total, 5);
        assert_eq!(sm.symbols[0], 'a');
        assert_eq!(sm.symbols[1], 'b');
        assert_eq!(sm.symbols[2], 'c');
        assert_eq!(sm.symbols[3], 'd');
        assert_eq!(sm.symbols[4], 'e');
        assert_eq!(sm.counts[0], 1);
        assert_eq!(sm.counts[1], 1);
        assert_eq!(sm.counts[2], 1);
        assert_eq!(sm.counts[3], 1);
        assert_eq!(sm.counts[4], 1);
    }

    #[test]
    fn contains_test() {
        let sm = VectorCountSymbolModel::new(vec!['a', 'b', 'c', 'd', 'e']);
        assert_eq!(sm.contains(&'a'), true);
        assert_eq!(sm.contains(&'b'), true);
        assert_eq!(sm.contains(&'c'), true);
        assert_eq!(sm.contains(&'d'), true);
        assert_eq!(sm.contains(&'e'), true);
        assert_eq!(sm.contains(&'f'), false);
        assert_eq!(sm.contains(&'g'), false);
        assert_eq!(sm.contains(&'h'), false);
    }

    #[test]
    fn interval_test() {
        let mut sm = VectorCountSymbolModel::new(vec!['a', 'b', 'c', 'd', 'e']);
        sm.set_count(&'a', 5);
        sm.set_count(&'b', 10);
        sm.set_count(&'c', 8);
        sm.set_count(&'d', 2);
        sm.set_count(&'e', 25);

        let a_interval = sm.interval(&'a');
        let b_interval = sm.interval(&'b');
        let c_interval = sm.interval(&'c');
        let d_interval = sm.interval(&'d');
        let e_interval = sm.interval(&'e');

        assert_eq!(sm.total(), 50);

        assert_eq!(a_interval.0, 0);
        assert_eq!(b_interval.0, 5);
        assert_eq!(c_interval.0, 15);
        assert_eq!(d_interval.0, 23);
        assert_eq!(e_interval.0, 25);

        assert_eq!(a_interval.1, 5);
        assert_eq!(b_interval.1, 15);
        assert_eq!(c_interval.1, 23);
        assert_eq!(d_interval.1, 25);
        assert_eq!(e_interval.1, 50);

    }
}
