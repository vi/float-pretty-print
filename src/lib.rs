use std::fmt::{Display, Formatter, Result};

const DEBUG : bool = false;

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

        if c == Medium {
            let probe = format!("{:.0}", x);
            let length_of_integer_part = probe.len();

            if DEBUG { eprintln!(
                "Length of integer part is {}, which width_max is {}",
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
                    if probe != "0" {
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
                // b fits max_width, but may be opportunities to chip off zeroes
                let b = format!("{:.p$}", x, p=(width_max-1-length_of_integer_part));
                if DEBUG { eprintln!("Intermediate result: {}", &b); }
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
                return write!(fmt, "{}", &b[0..end]);
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
                } else if minimum == width_max || minimum == width_max-1 {
                    if DEBUG { eprintln!("Fits just right"); }
                    return write!(fmt, "{}", probe);
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
