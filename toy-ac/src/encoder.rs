use super::range::Range;
use super::symbol_model::SymbolModel;
use std::io::Write;
use bitbit::BitWriter;

#[derive(Debug)]
pub struct Encoder {
    range: Range,
    pending: u32,
    finished: bool
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            range: Range::new(32),
            pending: 0,
            finished: false
        }
    }

    pub fn encode<T: Eq, W: Write>(&mut self, s: &T, m: &dyn SymbolModel<T>, output: &mut BitWriter<W>) {
        if self.finished {
            panic!("Encoder already finished");
        }
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

    pub fn high(&self) -> u64 {
        self.range.high()
    }

    pub fn low(&self) -> u64 {
        self.range.low()
    }

    pub fn finish<W: Write>(&mut self,  output: &mut BitWriter<W>) -> Result<(), Box<dyn std::error::Error>> {
        // Write out any value between range low and high (0x80000000 for example)
        // plus any pending bits as 0. The correct understanding of this is 
        // writing out a 1, plus any pending bits as 0, followed by 31 more zeroes.

        output.write_bit(true)?;
        for _ in 0..self.pending+31 {
            output.write_bit(false)?;
        }

        self.finished = true;
        Ok(())
    }
}

