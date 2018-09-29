use std::fmt;
use {Close, Next, Reset};
use errors::*;

/// An Relative moving average (RMA)
///
/// Exactly the same like EMA, except what it uses alpha = 1 / y
///
/// # Parameters
///
/// * _length_ - number of periods (integer greater than 0)
///
/// # Example
///
///
/// # Links
///
/// * [Exponential moving average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average)
///
#[derive(Debug,Clone)]
pub struct RollingMovingAverage {
    length: u32,
    k:  f64,
    current: f64,
    is_new: bool
}

impl RollingMovingAverage {
    pub fn new(length: u32) -> Result<Self> {
        match length {
            0 => Err(Error::from_kind(ErrorKind::InvalidParameter)),
            _ => {
                let k = 1f64 / (length as f64 + 1f64);
                let indicator = Self { length: length, k: k, current: 0f64, is_new: true };
                Ok(indicator)
            }
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }
}

impl Next<f64> for RollingMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            self.current = input;
        } else {
            self.current = self.k * input + (1.0 - self.k) * self.current;
        }
        self.current
    }
}

impl<'a, T: Close> Next<&'a T> for RollingMovingAverage {
    type Output = f64;

    fn next(&mut self, input: &'a T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for RollingMovingAverage {
    fn reset(&mut self) {
        self.current = 0.0;
        self.is_new = true;
    }
}

impl Default for RollingMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for RollingMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EMA({})", self.length)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_helper::*;

    test_indicator!(RelativeMovingAverage);

    #[test]
    fn test_new() {
        assert!(RollingMovingAverage::new(0).is_err());
        assert!(RollingMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut rma = RollingMovingAverage::new(3).unwrap();

        assert_eq!(rma.next(2.0), 2.0);
        assert_eq!(rma.next(5.0), 2.75);
        assert_eq!(rma.next(1.0), 2.3125);
        assert_eq!(rma.next(6.25), 3.296875);

        let mut rma = RollingMovingAverage::new(3).unwrap();
        let bar1 = Bar::new().close(2);
        let bar2 = Bar::new().close(5);
        assert_eq!(rma.next(&bar1), 2.0);
        assert_eq!(rma.next(&bar2), 2.75);
    }

    #[test]
    fn test_reset() {
        let mut rma = RollingMovingAverage::new(5).unwrap();

        assert_eq!(rma.next(4.0), 4.0);
        rma.next(10.0);
        rma.next(15.0);
        rma.next(20.0);
        assert_ne!(rma.next(4.0), 4.0);

        rma.reset();
        assert_eq!(rma.next(4.0), 4.0);
    }

    #[test]
    fn test_default() {
        RollingMovingAverage::default();
    }

    #[test]
    fn test_display() {
        let ema = RollingMovingAverage::new(7).unwrap();
        assert_eq!(format!("{}", ema), "EMA(7)");
    }
}
