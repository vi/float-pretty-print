#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! Round and format f64 number for showing it to humans, with configurable minimum and maximum width.
//! It automatically switches between exponential and usual forms as it sees fit.
//!
//! It works by trying usual `format!`, possibly multiple times and inspecting the resulting string.
//!
//! Only two formatting parameters are supported:
//!
//! * width is minimum width
//! * precision is maximum width
//!
//! `###` is printed if it can't output the number with given constraint.
//!
//! Example:
//!
//! ```
//!   use float_pretty_print::PrettyPrintFloat;
//!   assert_eq!(format!("{}", PrettyPrintFloat(3.45)), "3.45");
//!   assert_eq!(format!("{}", PrettyPrintFloat(12.0)), "12.0");
//!   assert_eq!(format!("{}", PrettyPrintFloat(120000000.0)), "1.2e8");
//!   assert_eq!(format!("{:5.5}", PrettyPrintFloat(12345.0)), "12345");
//!   assert_eq!(format!("{:5.5}", PrettyPrintFloat(12.345)), "12.35");
//!   assert_eq!(format!("{:5.5}", PrettyPrintFloat(0.12345)), "0.123");
//!   assert_eq!(format!("{:5.5}", PrettyPrintFloat(1234500000.0)), "1.2e9");
//!   assert_eq!(format!("{:5.5}", PrettyPrintFloat(12345.0e-19)), "1e-15");
//!   assert_eq!(format!("{:5.5}", PrettyPrintFloat(12345.0e-100)), "1e-96");
//!   assert_eq!(format!("{:5.5}", PrettyPrintFloat(12345.0e-130)), "    0");
//!   assert_eq!(format!("{:5.5}", PrettyPrintFloat(12345.0e+130)), "1e134");
//!   assert_eq!(format!("{:4.4}", PrettyPrintFloat(12345.0e+130)), "####");
//!   assert_eq!(format!("{:6.6}", PrettyPrintFloat(12345.0e-130)), "1e-126");
//! ```
//!
//! Supports even Rust 1.23

use std::fmt::{Display, Formatter, Result};

const DEBUG : bool = false;

/// `f64` wrapper to use with formatting code.
/// See the crate-level doc for details.
///
/// ```
///   use float_pretty_print::PrettyPrintFloat;
///   assert_eq!(format!("{:4.4}", PrettyPrintFloat(0.00005)), "5e-5");
/// ```
pub struct PrettyPrintFloat(pub f64);

#[derive(PartialEq, Eq, Debug)]
enum NumberClass {
    Big,
    Medium,
    Small,
    Zero,
    Special,
    Unprintable,
}

impl PrettyPrintFloat {
    fn cls(&self) -> NumberClass {
        let mut x = self.0;
        if !x.is_finite() {
            return NumberClass::Special;
        }
        if x < 0.0 {
            x = -x;
        }
        if x == 0.0 {
            return NumberClass::Zero;
        }
        if x > 99999.0 {
            return NumberClass::Big;
        }
        if x < 0.001 {
            return NumberClass::Small;
        }
        return NumberClass::Medium;
    }
}

impl Display for PrettyPrintFloat {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let mut width_min = fmt.width().unwrap_or(3);
        let mut width_max = fmt.precision().unwrap_or(12);
        let x = self.0;

        if width_min == 0 {
            width_min = 1;
        }

        if width_max == 0 {
            return Ok(())
        }

        if width_min > width_max {
            width_max = width_min;
        }

        use NumberClass::*;
        let mut c = self.cls();

        if DEBUG { eprintln!("Number {} classified as {:?}", x, c); }

        if c == Special {
            let q = format!("{}", x);
            return if q.len() <= width_max {
                write!(fmt, "{:w$}", q, w=width_min)
            } else {
                write!(fmt, "{:.p$}", "########", p=width_max)
            }
        }
        if c == Zero {
            return if width_max < 3 || width_min < 3 {
                write!(fmt, "{:w$}", "0", w=width_min)
            } else {
                write!(fmt, "{:.p$}", 0.0, p=(width_min-2))
            };
        }

        let probe_for_medium_mode;
        if c == Medium {
            probe_for_medium_mode = format!("{:.0}", x);
            let length_of_integer_part = probe_for_medium_mode.len();

            if DEBUG { eprintln!(
                "Probe=`{}`; length of integer part is {}, which width_max is {}",
                probe_for_medium_mode,
                length_of_integer_part,
                width_max,
            ); }

            match length_of_integer_part {
                l if l > width_max => {
                    if DEBUG { eprintln!("Too large, switching to Big"); }
                    c = Big;
                },
                l if l + 1 >= width_max => {
                    if DEBUG { eprintln!("Almost too large, checking zeroness"); }
                    if probe_for_medium_mode != "0" {
                        if DEBUG { eprintln!("Seems to be OK to print as integer"); }
                        // print as integer
                        return write!(fmt, "{:w$.0}", x, w=width_min);
                    } else {
                        if DEBUG { eprintln!("Refusing to print it"); }
                        c = Unprintable;
                    }
                },
                _ => {
                    // Enouch room to try fractional part
                    // Check if it would be all zeroes
                    if DEBUG { eprintln!("Enough room to consider fractional part"); }

                    let probe = format!(
                        "{:.p$}",
                        x,
                        p=(width_max - 1 - length_of_integer_part),
                    );

                    let mut num_zeroes = 0;
                    let mut num_digits = 0;
                    let mut significant_zeroes = false;

                    for c in probe.chars() {
                        match c {
                            '0' => {
                                num_digits += 1;
                                if ! significant_zeroes {
                                    num_zeroes += 1;
                                }
                            },
                            '.' => {
                                
                            }
                            '-' => {

                            },
                            _ => {
                                num_digits += 1;
                                significant_zeroes = true;
                            },
                        }
                    }
                    if DEBUG { eprintln!(
                        "{} zero of {} digits in the test print",
                        num_zeroes,
                        num_digits,
                    ); }

                    assert!(num_digits > 0);

                    if (num_zeroes * 100 / num_digits) > 80 {
                        if DEBUG { eprintln!("Too small to print normally, switching to Small"); }
                        // Too many zeroes, too few actual digits
                        c = Small;
                    }
                },
            }

            if c == Medium {
                if DEBUG { eprintln!("Medium mode confirmed"); }
                // b fits max_width, but there may be opportunities to chip off zeroes
                let mut b = format!("{:.p$}", x, p=(width_max-1-length_of_integer_part));
                if DEBUG { eprintln!("Intermediate result: {}", &b); }
                let first_digit_of_probe = probe_for_medium_mode.bytes().into_iter().next();
                if first_digit_of_probe == Some(b'1') && first_digit_of_probe != b.bytes().into_iter().next() {
                    if DEBUG { eprintln!("Looks like we have overestimated the integer part size");  }

                    let b2 = format!("{:.p$}", x, p=(width_max-1-length_of_integer_part+1));
                    if b2.len() <= width_max {
                        b = b2;
                    }
                }
                let mut end = b.len();
                if b.contains('.') {
                    loop {
                        if end <= width_min { break }
                        if end < 3 { break }
                        if !b[0..end].ends_with('0') { break }
                        if b[0..(end-1)].ends_with('.') { 
                            // protect one zero after '.'
                            break
                        }
                        if DEBUG { eprintln!("Chipped away some zero"); }
                        end -= 1;
                    }
                }
                let b = &b[0..end];
                for _ in b.len()..width_min {
                    write!(fmt, " ")?;
                }
                return write!(fmt, "{}", b);
            }
        }

        match c {
            Zero | Special | Medium => unreachable!(),
            Big | Small => {
                let probe = format!("{:.0e}", x);
                if DEBUG { eprintln!("First probe: {}", &probe); }
                let mut minimum = probe.len();
                if minimum > width_max {
                    if DEBUG { eprintln!("Can't fit it"); }
                    if c == Big {
                        c = Unprintable;
                    } else {
                        if DEBUG { eprintln!("Just print zero"); }
                        return write!(fmt, "{:w$}", 0.0, w=width_min);
                    }
                } else if minimum == width_max {
                    if DEBUG { eprintln!("Fits just right"); }
                    return write!(fmt, "{}", probe);
                } else if minimum == width_max-1 {
                    if DEBUG { eprintln!("Fits almost just right"); }
                    // Can't increase precision because of we need to add a `.` as well
                    return write!(fmt, " {}", probe);
                } else {
                    if DEBUG { eprintln!("There is some space to be more precise"); }
                    let probe2 = format!("{:.p$e}", x, p=(width_max - minimum - 1) );
                    if DEBUG { eprintln!("Second probe: {}", &probe2); }
                    if probe2.len() > width_max {
                        minimum += probe2.len() - width_max;
                    }
                    let mut zeroes_before_e = 0;
                    let mut zeroes_in_a_row = 0;
                    for c in probe2.chars() { match c {
                        '0' => zeroes_in_a_row += 1,
                        'e' | 'E' => {
                            zeroes_before_e = zeroes_in_a_row;
                        },
                        _ => zeroes_in_a_row = 0,
                    } }
                    if DEBUG { eprintln!("{} zeroes before E", zeroes_before_e); }
                    let zeroes_to_chip_away = zeroes_before_e.min(width_max-width_min);
                    if DEBUG { eprintln!("{} zeroes to be removed", zeroes_to_chip_away); }
                    return write!(fmt, "{:.p$e}", x, p=(width_max - minimum - 1 - zeroes_to_chip_away) );
                }
            },
            Unprintable => (),
        }
        let _ = c;

        write!(fmt, "{:.p$}", "##################################", p=width_min)
    }
}

#[cfg(test)]
mod test;
