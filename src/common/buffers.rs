use num::Num;

/// A non-resizable (after creation) buffer.
pub struct ShiftBuf<T: Num + Copy> {
    qs: Box<[T]>,
}


impl<T: Num + Copy> ShiftBuf<T> {
    /// Create a new shift register.
    ///
    /// # Examples
    /// ```
    /// # use dsp::common::buffers::ShiftBuf;
    /// let buf: ShiftBuf<f32> = ShiftBuf::new(10);
    /// ```
    pub fn new(n: usize) -> Self {
        let mut qs = Vec::new();
        for _ in 0..n {
            qs.push(T::zero());
        }
        ShiftBuf { qs: qs.into_boxed_slice() }
    }

    /// Shift new simples into the register.
    ///
    /// ```
    /// # use dsp::common::buffers::ShiftBuf;
    /// let buf: ShiftBuf<f32> = ShiftBuf::new(10);
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
    /// # use dsp::common::buffers::ShiftBuf;
    /// let buf = ShiftBuf::<i32>::new(10);
    /// assert_eq!(10, buf.len())
    /// ```
    pub fn len(&self) -> usize {
        self.qs.len()
    }

}

impl<T: Num + Copy> AsRef<[T]> for ShiftBuf<T> {
    fn as_ref(&self) -> &[T] {
        self.qs.as_ref()
    }
}

#[cfg(test)]
mod test {
    use ::common::buffers::ShiftBuf;

    #[test]
    fn test_shift_register_multipart() {
        let mut buf = ShiftBuf::<i32>::new(5);
        buf.push([0,1,2,3,4].as_ref());
        buf.push([5].as_ref());
        assert_eq!(buf.as_ref(),[1,2,3,4,5].as_ref())
    }
}
