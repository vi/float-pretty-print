extern crate float_pretty_print;

use std::io::BufRead;

fn main() {
    eprintln!("Type number to be formatted using float_pretty_print");
    eprintln!("Also type `width=<number>` or `prec=<number>` to set width or precision.");

    let si = std::io::stdin();
    let si = si.lock();
    let si = std::io::BufReader::new(si);

    let mut width : Option<usize> = None;
    let mut prec : Option<usize> = None;

    for l in si.lines() {
        let l = l.unwrap();
        if l.contains('=') {
            let before_and_after_eq_sign = l.split('=').collect::<Vec<_>>();
            assert_eq!(before_and_after_eq_sign.len(), 2);
            match before_and_after_eq_sign[0] {
                "width" => {
                    width = Some(before_and_after_eq_sign[1].parse().unwrap());
                },
                "prec" => {
                    prec = Some(before_and_after_eq_sign[1].parse().unwrap());
                },
                _ => {
                    eprintln!("Unknown property");
                }
            }
            continue;
        }
        let f : f64 = l.parse().unwrap();
        let f = float_pretty_print::PrettyPrintFloat(f);

        match (width, prec) {
            (None, None) => println!("{}", f),
            (Some(w), None) => println!("{:w$}", f, w=w),
            (None, Some(p)) => println!("{:.p$}", f, p=p),
            (Some(w), Some(p)) => println!("{:w$.p$}", f, w=w, p=p),
        }
    }
}
