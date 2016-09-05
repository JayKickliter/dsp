use num::Num;

/// A non-resizable (after creation) buffer.
pub struct ShiftRegister<T: Num + Copy> {
    qs: Vec<T>,
}


impl<T: Num + Copy> ShiftRegister<T> {
    /// Create a new shift register.
    ///
    /// # Examples
    /// ```
    /// # use dsp::common::buffers::ShiftRegister;
    /// let sr: ShiftRegister<f32> = ShiftRegister::new(10);
    /// ```
    pub fn new(n: usize) -> Self {
        let mut qs = Vec::new();
        for _ in 0..n {
            qs.push(T::zero());
        }
        ShiftRegister { qs: qs }
    }

    /// Shift new simples into the register.
    ///
    /// ```
    /// # use dsp::common::buffers::ShiftRegister;
    /// let sr: ShiftRegister<f32> = ShiftRegister::new(10);
    /// ```
    pub fn push(&mut self, xs: &[T]) {
        if xs.len() >= self.len() {
            let pivot = xs.len() - self.len();
            let new_qs = &xs[pivot..];
            for i in 0..self.len() {
                self.qs[i] = new_qs[i];
            }
        } else {

        }
    }

    /// Returns the length of the shift registers.
    pub fn len(&self) -> usize {
        self.qs.len()
    }
}
