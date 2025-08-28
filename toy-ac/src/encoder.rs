use super::range::Range;
use super::symbol_model::SymbolModel;
use std::io::Write;
use bitbit::BitWriter;

pub struct Encoder {
    range: Range,
    pending: u32
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            range: Range::new(32),
            pending: 0,
        }
    }

    pub fn encode<T: Eq, W: Write>(&mut self, s: &T, m: &dyn SymbolModel<T>, output: &mut BitWriter<W>) {
        if !m.contains(s) {
            panic!("Value is not in model");
        }
        let (int_start, int_end) = m.interval(s);
        let range_width = self.range.width();
        let new_low = (range_width as f64 * int_start) as u64 + self.range.low();
        let new_high = (range_width as f64 * int_end) as u64 + self.range.low() - 1;
        self.range.reduce(new_high, new_low);
        if self.range.hob_match() {
            let is_one = self.range.shift_hob();
            output.write_bit(is_one).unwrap();
            for _ in 0..self.pending {
                output.write_bit(!is_one).unwrap();
            }
            self.pending = 0;
            while self.range.hob_match() {
                output.write_bit(self.range.shift_hob()).unwrap();
            }
        }
        assert!(!self.range.hob_match());
        while self.range.in_middle() {
            self.range.shift_sob();
            self.pending += 1;
        }
    }
}

