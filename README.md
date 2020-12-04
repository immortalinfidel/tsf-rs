# TimeSeries Forecast (TSF)
```
 use tsf_rs::TSF;
 use ta_common::traits::Indicator;
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

```