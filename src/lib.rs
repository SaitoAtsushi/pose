use std::{fmt::Display, fmt::Formatter};
pub mod parser;
pub use parser::*;
pub mod types;
pub use types::*;

struct FixedPoint(f64);

impl Display for FixedPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (flag, absolute) = if self.0 < 0.0 {
            ("-", -self.0)
        } else {
            ("", self.0)
        };
        let log10 = absolute.log10();
        const PREC: usize = 15;
        let fr = absolute.fract();
        let tr = absolute.trunc();
        let p = log10.ceil() as usize;
        write!(f, "{}{}", flag, tr as u64)?;
        let mut width = PREC - p;
        let mut fr = ((10.0_f64.powi(width.try_into().unwrap())) as f64 * fr) as u64;

        if fr != 0 {
            while fr % 10 == 0 {
                width -= 1;
                fr /= 10
            }
            write!(f, ".{:0width$}", fr)?;
        }
        Ok(())
    }
}

impl Display for PoseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            &PoseType::End => write!(f, "EOF"),
            &PoseType::String(ref str) => write!(f, "\"{}\"", str),
            &PoseType::Number(num) => {
                let log10 = num.abs().log10();
                let width = log10.ceil() as i32;
                const PREC: i32 = 15;
                if (width <= PREC && num >= 0.1) || (width <= PREC && num <= -0.1) {
                    write!(f, "{}", FixedPoint(num))
                } else if width > PREC || num < 0.1 {
                    let e = 10.0_f64.powi(width - 1);
                    write!(f, "{}e{}", FixedPoint(num / e), width - 1)?;
                    Ok(())
                } else {
                    write!(f, "{:.*e}", PREC as usize, num)
                }
            }
            &PoseType::Symbol(ref str) => write!(f, "{}", str),
            &PoseType::List(ref v) => {
                write!(f, "(")?;
                let mut iter = (&v).into_iter();
                if let Some(first) = iter.next() {
                    write!(f, "{}", first)?;
                    for &ref e in iter {
                        write!(f, "{}", e)?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
        }
    }
}
