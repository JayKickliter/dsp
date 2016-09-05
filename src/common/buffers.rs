use num::Num;

/// A non-resizable (after creation) buffer.
pub struct ShiftRegister<T: Num + Copy> {
    qs: Box<[T]>,
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
        ShiftRegister { qs: qs.into_boxed_slice() }
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
            for i in 0..(self.len()-xs.len()) {
                self.qs[i] = self.qs[i+xs.len()];
            }
            let pivot = self.len()-xs.len();
            let mut xidx = 0;
            for i in pivot..self.len() {
                self.qs[i] = xs[xidx];
                xidx += 1;
            }
        }
    }

    /// Returns the length of the shift registers.
    ///
    /// # Examples
    /// ```
    /// # use dsp::common::buffers::ShiftRegister;
    /// let sr = ShiftRegister::<i32>::new(10);
    /// assert_eq!(10, sr.len())
    /// ```
    pub fn len(&self) -> usize {
        self.qs.len()
    }

}

impl<T: Num + Copy> AsRef<[T]> for ShiftRegister<T> {
    fn as_ref(&self) -> &[T] {
        self.qs.as_ref()
    }
}

#[cfg(test)]
mod test {
    use ::common::buffers::ShiftRegister;

    #[test]
    fn test_shift_register_multipart() {
        let mut sr = ShiftRegister::<i32>::new(5);
        sr.push([0,1,2,3,4].as_ref());
        sr.push([5].as_ref());
        assert_eq!(sr.as_ref(),[1,2,3,4,5].as_ref())
    }
}
