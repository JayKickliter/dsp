use itertools::Stride;
use std::ops::Index;

/// # Examples
/// ```
/// use dsp::filters::fir::pfb::PFB;
/// let taps = vec![0,1,2,3,4,5,6,7,8];
/// let pfb = PFB::new(3, &taps);
/// assert_eq!([0,3,6].as_ref(), pfb.get(0).unwrap());
/// assert_eq!([1,4,7].as_ref(), pfb.get(1).unwrap());
/// assert_eq!([2,5,8].as_ref(), pfb.get(2).unwrap());
/// ```
pub struct PFB<T> {
    pfb: Vec<Vec<T>>,
}

impl<T: Copy> PFB<T> {
    /// Create a new PFB from an array of filter taps.
    ///
    /// # Panics
    /// Panics if `n < taps.len()`.
    ///
    /// # Examples
    /// ```
    /// # #![allow(unused)]
    /// # use dsp::filters::fir::pfb::PFB;
    /// let taps = vec![0,1,2,3,4,5,6,7,8];
    /// let pfb = PFB::new(3, &taps);
    /// ```
    pub fn new(n: usize, taps: &[T]) -> PFB<T> {
        assert!(taps.len() >= n);
        let mut arms = Vec::new();
        // build up the filter bank
        for i in 0..n {
            let mut arm = Vec::new();
            // build up the filter arm
            // first set the offset from the beginning of taps
            let offset_taps = &taps[i..];
            for tap in Stride::from_slice(offset_taps, n as isize) {
                arm.push(*tap);
            }
            arms.push(arm);
        }
        PFB { pfb: arms }
    }

    /// Returns an arm of the PFB as a slice.
    pub fn get(&self, idx: usize) -> Option<&[T]> {
        if let Some(arm) = self.pfb.get(idx) {
            Some(arm.as_ref())
        } else {
            None
        }
    }
}


impl<T> Index<usize> for PFB<T> {
    type Output = [T];

    fn index<'a>(&'a self, index: usize) -> &'a [T] {
        &self.pfb[index]
    }
}
