// The first thing to do is to add some crate level documentation. We use //! for that.

//! `bucketize` is a crate for slotting numerical values into buckets.
//! To do this, create a `Bucketizer` and add your buckets to it,
//! then use the `.bucketize()` method to get back the bucket a value fits into.
//!
//! # Example
//! ```
//! use bucketize::Bucketizer;
//! 
//! let b = Bucketizer::new()
//!     .bucket(Some(10.0), Some(20.0), 15.0)
//!     .bucket(Some(5.0), Some(10.0), 7.5)
//!     .bucket(None, Some(4.0), 0.0);
//!
//! assert_eq!(b.bucketize(12.34), Some(15.0));
//! assert_eq!(b.bucketize(9999.99), None);
//! ```

// Now we need documentation for the Bucketizer struct.

/// A `Bucketizer` holds the list of buckets you want to slot values into, and does
/// the bucketization operation.
///
/// You can create one with `new()` and add buckets with chained `.bucket()` calls.
/// These calls add buckets which are evaluated in order. For instance, if you add
/// a bucket from 0 to 100 and then add a bucket from 2 to 50, nothing will ever
/// get put in that second bucket.
///
/// Buckets are min-inclusive and max-exclusive. If a given value matches no bucket,
/// `bucketize` returns `None`.
///
/// # Example
/// ```
/// use bucketize::Bucketizer;
/// 
/// let b = Bucketizer::new()
///     .bucket(Some(10.0), Some(20.0), 15.0)
///     .bucket(Some(5.0), Some(10.0), 7.5)
///     .bucket(None, Some(4.0), 0.0);
///
/// assert_eq!(b.bucketize(12.34), Some(15.0));
/// assert_eq!(b.bucketize(9999.99), None);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Bucketizer {
    buckets: Vec<Bucket>
}

// We won't document this, because it's an internal only type. The user never sees it.
type Bucket = (Option<f64>, Option<f64>, f64);

impl Bucketizer {
    /// Create a new `Bucketizer` with no buckets configured.
    pub fn new() -> Self {
        Self {
            buckets: Vec::new()
        }
    }
    
    /// Add a new bucket to the `Bucketizer`. Consumes and returns the `Bucketizer` so
    /// it can be chained.
    ///
    /// Buckets are evaluated in the order they are added.
    ///
    /// A value fits in a bucket if it is greater than or equal to `min` and less than
    /// `max`, if each is present.
    ///
    /// # Examples
    ///
    /// Here, we create a `Bucketizer` with a single bucket matching any value 10
    /// or greater.
    ///
    /// ```
    /// # use bucketize::Bucketizer;
    /// let b = Bucketizer::new().bucket(Some(10.0), None, 10.0);
    ///
    /// assert_eq!(b.bucketize(12.0), Some(10.0));
    /// assert_eq!(b.bucketize(-10.0), None);
    /// ```
    ///
    /// Here, we create a `Bucketizer` matching values from 0 to < 10 and from 
    /// 10 to infinity.
    /// ```
    /// # use bucketize::Bucketizer;
    /// let b = Bucketizer::new()
    ///     .bucket(Some(10.0), None, 10.0)
    ///     .bucket(Some(0.0), Some(10.0), 5.0);
    ///
    /// assert_eq!(b.bucketize(4.132), Some(5.0));
    /// assert_eq!(b.bucketize(12.0), Some(10.0));
    /// assert_eq!(b.bucketize(-10.0), None);
    /// ```
    pub fn bucket(self, min: Option<f64>, max: Option<f64>, value: f64) -> Self {
        let mut new = self;
        new.buckets.push((min, max, value));
        new
    }

    /// Get the bucketized value of `input` based on the previously configured `buckets`
    /// for this `Bucketizer`.
    pub fn bucketize(&self, input: f64) -> Option<f64> {
        for bucket in &self.buckets {
            match *bucket {
                (None, None, val) => return Some(val),
                (Some(min), None, val) => {
                    if input >= min { return Some(val) }
                },
                (None, Some(max), val) => {
                    if input < max { return Some(val) }
                },
                (Some(min), Some(max), val) => {
                    if input >= min && input < max { return Some(val) }
                }
            }
        }
    
        return None;
    }
}

// So why do we add assert_eq! in documentation? Because they're also tests!

#[cfg(test)]
mod tests {
    use super::Bucketizer;

    #[test]
    fn single_bucket_middle_values() {
        let bucketizer = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(bucketizer.bucketize(0.1), Some(0.5));
        assert_eq!(bucketizer.bucketize(0.7), Some(0.5));
    }

   #[test]
    fn single_bucket_end_values() {
        let bucketizer = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(bucketizer.bucketize(0.0), Some(0.5));
        assert_eq!(bucketizer.bucketize(1.0), None);
    }

    #[test]
    fn multiple_buckets_closed_ends() {
        let bucketizer = Bucketizer::new()
            .bucket(Some(-1.0), Some(0.0), -0.5)
            .bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(bucketizer.bucketize(0.0), Some(0.5));
        assert_eq!(bucketizer.bucketize(-0.7), Some(-0.5));
        assert_eq!(bucketizer.bucketize(999.99), None);
    }
    
    #[test]
    fn multiple_buckets_open_ended() {
        let bucketizer = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5)
            .bucket(Some(1.0), None, 1.0);
        assert_eq!(bucketizer.bucketize(0.3), Some(0.5));
        assert_eq!(bucketizer.bucketize(999.99), Some(1.0));
        assert_eq!(bucketizer.bucketize(-999.99), None);
    }
}

