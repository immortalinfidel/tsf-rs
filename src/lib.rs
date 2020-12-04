#![feature(external_doc)]
use sma_rs::SMA;
use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Indicator;

#[doc(include = "../README.md")]
pub struct TSF {
    x_mean: f64,
    x_var_sq: f64,
    sma: SMA,
    history: FixedQueue<f64>,
    period: u32,

}


impl TSF {
    pub fn new(period: u32) -> TSF {
        let mean = ((period as f64 * (period as f64 + 1.0)) / 2.0) / period as f64;
        Self {
            x_mean: mean,
            sma: SMA::new(period),
            history: FixedQueue::new(period),
            period,
            x_var_sq: {
                let mut sum = 0.0;
                for i in 1..(period + 1) {
                    sum += (i as f64 - mean).powf(2.0);
                }
                sum
            },
        }
    }
}


impl Indicator<f64, Option<f64>> for TSF {
    fn next(&mut self, input: f64) -> Option<f64> {
        let sma = self.sma.next(input);
        self.history.add(input);
        match sma {
            None => None,
            Some(sm) => {
                let mut beta = 0.0;
                for i in 1..(self.period + 1) {
                    println!("{} xmean {}", i, self.x_mean);
                    beta += ((i as f64 - self.x_mean) * (self.history.at((i - 1) as i32).unwrap() - sm)) / self.x_var_sq;
                }
                let alpha = sm - beta * self.x_mean;
                let tsf = alpha + beta * (self.period as f64 + 1.0);
                Some(tsf)
            }
        }
    }

    fn reset(&mut self) {
        self.sma.reset();
    }
}


#[cfg(test)]
mod tests {
    use crate::TSF;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut tsf = TSF::new(5);
        assert_eq!(tsf.next(81.59), None);
        assert_eq!(tsf.next(81.06), None);
        assert_eq!(tsf.next(82.87), None);
        assert_eq!(tsf.next(83.00), None);
        assert_eq!(tsf.next(83.61), Some(84.22));
        assert_eq!(tsf.next(83.15), Some(84.21399999999998));
        assert_eq!(tsf.next(82.84), Some(83.12099999999998));
        assert_eq!(tsf.next(83.99), Some(83.68099999999998));
        assert_eq!(tsf.next(84.55), Some(84.44399999999997));
        assert_eq!(tsf.next(84.36), Some(85.017));
        assert_eq!(tsf.next(85.53), Some(85.97899999999998));
        assert_eq!(tsf.next(86.54), Some(86.81799999999997));
        assert_eq!(tsf.next(86.89), Some(87.63199999999998));
        assert_eq!(tsf.next(87.77), Some(88.67199999999997));
        assert_eq!(tsf.next(87.29), Some(88.22899999999996));
    }
}

