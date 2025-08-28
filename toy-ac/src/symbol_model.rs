pub trait SymbolModel<T: std::cmp::Eq> {
    fn contains(&self, s: &T) -> bool;
    fn interval(&self, s: &T) -> (f64, f64);
    fn lookup(&self, v: f64) -> (&T, f64, f64);
}

pub struct VectorCountSymbolModel<T: std::cmp::Eq> {
    symbols: Vec<T>,
    counts: Vec<u32>,
    total: u32,
}

impl<T: std::cmp::Eq> VectorCountSymbolModel<T> {
    pub fn new(symbols: Vec<T>) -> Self {
        let counts: Vec<u32> = vec![1; symbols.len()];
        let length = symbols.len() as u32;
        Self {
            symbols: symbols,
            counts: counts,
            total: length,
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
    }

    pub fn incr_count(&mut self, s: &T) {
        let idx = self.find_index(s);
        self.total += 1;
        self.counts[idx] += 1;
    }

}

impl<T: std::cmp::Eq> SymbolModel<T> for VectorCountSymbolModel<T> {
    fn contains(&self, s: &T) -> bool {
        return self.symbols.contains(s);
    }

    fn interval(&self, s: &T) -> (f64, f64) {
        let mut sum = 0;
        let mut idx = 0;
        while idx < self.symbols.len() {
            if self.symbols[idx] == *s {
                return (
                    (sum as f64 / self.total as f64),
                    ((sum + self.counts[idx]) as f64 / self.total as f64),
                );
            }
            sum += self.counts[idx];
            idx += 1;
        }
        panic!("Symbol not in model.");
    }

    fn lookup(&self, v: f64) -> (&T, f64, f64) {
        if v < 0.0 || v >= 1.0 {
            panic!("Lookup value out of range");
        }

        let mut sum = 0;
        let mut idx = 0;
        while idx < self.symbols.len() {
            let int_start = (sum as f64 / self.total as f64);
            let int_end = ((sum + self.counts[idx]) as f64) / self.total as f64;
            if v >= int_start && v < int_end {
                return (&self.symbols[idx], int_start, int_end);
            }
            sum += self.counts[idx];
            idx += 1;
        }
        panic!("Should never happen");
    }
}

mod tests {
    use super::*;
    use assert_float_eq::assert_f64_near;

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

        assert_f64_near!(a_interval.0, 0.0);
        assert_f64_near!(b_interval.0, (5.0 / 50.0) as f64);
        assert_f64_near!(c_interval.0, (15.0 / 50.0) as f64);
        assert_f64_near!(d_interval.0, (23.0 / 50.0) as f64);
        assert_f64_near!(e_interval.0, (25.0 / 50.0) as f64);

        assert_f64_near!(a_interval.1, (5.0 / 50.0) as f64);
        assert_f64_near!(b_interval.1, (15.0 / 50.0) as f64);
        assert_f64_near!(c_interval.1, (23.0 / 50.0) as f64);
        assert_f64_near!(d_interval.1, (25.0 / 50.0) as f64);
        assert_f64_near!(e_interval.1, 1.0);
    }
}
