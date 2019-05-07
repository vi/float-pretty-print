# float-pretty-print

Round and format f64 number for showing it to humans, with configurable minimum and maximum width.
It automatically switches between exponential and usual forms as it sees fit.

It works by trying usual `format!`, possibly multiple times and inspecting the resulting string.

Only two formatting parameters are supported:

* width is minimum width
* precision is maximum width

`###` is printed if it can't output the number with given constraint.

Example:

```rust
  use float_pretty_print::PrettyPrintFloat;
  assert_eq!(format!("{}", PrettyPrintFloat(3.45)), "3.45");
  assert_eq!(format!("{}", PrettyPrintFloat(12.0)), "12.0");
  assert_eq!(format!("{}", PrettyPrintFloat(120000000.0)), "1.2e8");
  assert_eq!(format!("{:5.5}", PrettyPrintFloat(12345.0)), "12345");
  assert_eq!(format!("{:5.5}", PrettyPrintFloat(12.345)), "12.35");
  assert_eq!(format!("{:5.5}", PrettyPrintFloat(0.12345)), "0.123");
  assert_eq!(format!("{:5.5}", PrettyPrintFloat(1234500000.0)), "1.2e9");
  assert_eq!(format!("{:5.5}", PrettyPrintFloat(12345.0e-19)), "1e-15");
  assert_eq!(format!("{:5.5}", PrettyPrintFloat(12345.0e-100)), "1e-96");
  assert_eq!(format!("{:5.5}", PrettyPrintFloat(12345.0e-130)), "    0");
  assert_eq!(format!("{:5.5}", PrettyPrintFloat(12345.0e+130)), "1e134");
  assert_eq!(format!("{:4.4}", PrettyPrintFloat(12345.0e+130)), "####");
  assert_eq!(format!("{:6.6}", PrettyPrintFloat(12345.0e-130)), "1e-126");
```

Supports even Rust 1.23
