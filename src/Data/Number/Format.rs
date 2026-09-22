// JavaScript number formatting: `toFixed`, `toPrecision`, `toExponential` and
// `toString`. The library clamps the digit counts (precision 1..21, fixed and
// exponential 0..20), so the ranges below are bounded in practice.

// ECMAScript `Number::toString`: shortest round-trip digits, decimal notation
// for 10^-6 <= |n| < 10^21, exponential outside.
fn js_number_to_string(n: f64) -> String {
    if n.is_nan() {
        return "NaN".to_owned();
    }
    if n == 0.0 {
        return "0".to_owned();
    }
    if n < 0.0 {
        return format!("-{}", js_number_to_string(-n));
    }
    if n.is_infinite() {
        return "Infinity".to_owned();
    }
    let formatted = format!("{:e}", n);
    let (mantissa, exponent) = formatted.split_once('e').expect("exponent notation");
    let exponent: i32 = exponent.parse().expect("decimal exponent");
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let k = digits.len() as i32;
    let point = exponent + 1;
    if k <= point && point <= 21 {
        format!("{}{}", digits, "0".repeat((point - k) as usize))
    } else if 0 < point && point <= 21 {
        let (head, tail) = digits.split_at(point as usize);
        format!("{}.{}", head, tail)
    } else if -6 < point && point <= 0 {
        format!("0.{}{}", "0".repeat((-point) as usize), digits)
    } else {
        let exponent = point - 1;
        let sign = if exponent >= 0 { "+" } else { "-" };
        let (head, tail) = digits.split_at(1);
        if tail.is_empty() {
            format!("{}e{}{}", head, sign, exponent.abs())
        } else {
            format!("{}.{}e{}{}", head, tail, sign, exponent.abs())
        }
    }
}

fn special(n: f64) -> Option<String> {
    if n.is_nan() {
        Some("NaN".to_owned())
    } else if n.is_infinite() {
        Some(if n > 0.0 { "Infinity".to_owned() } else { "-Infinity".to_owned() })
    } else {
        None
    }
}

pub fn Data_Number_Format_toString(number: f64) -> String {
    js_number_to_string(number)
}

pub fn Data_Number_Format_toFixedNative(digits: i64, number: f64) -> String {
    if let Some(text) = special(number) {
        return text;
    }
    let places = digits.max(0) as usize;
    if number.abs() >= 1e21 {
        return js_number_to_string(number);
    }
    let negative = number < 0.0;
    let rounded = (number.abs() * 10f64.powi(places as i32)).round();
    let text = format!("{:.0}", rounded);
    let text = if places == 0 {
        text
    } else if text.len() <= places {
        format!("0.{}{}", "0".repeat(places - text.len()), text)
    } else {
        let split = text.len() - places;
        format!("{}.{}", &text[..split], &text[split..])
    };
    if negative { format!("-{}", text) } else { text }
}

pub fn Data_Number_Format_toPrecisionNative(digits: i64, number: f64) -> String {
    if let Some(text) = special(number) {
        return text;
    }
    let significant = digits.max(1) as usize;
    let negative = number < 0.0;
    let formatted = format!("{:e}", number.abs());
    let (mantissa, exponent) = formatted.split_once('e').expect("exponent notation");
    let mut exponent: i32 = exponent.parse().expect("decimal exponent");
    let mut digits: Vec<u8> = mantissa.bytes().filter(|b| *b != b'.').collect();
    if digits.len() > significant {
        let round_up = digits[significant] >= b'5';
        digits.truncate(significant);
        if round_up {
            let mut index = significant;
            while index > 0 {
                if digits[index - 1] == b'9' {
                    digits[index - 1] = b'0';
                    index -= 1;
                } else {
                    digits[index - 1] += 1;
                    break;
                }
            }
            if index == 0 {
                digits.insert(0, b'1');
                digits.truncate(significant);
                exponent += 1;
            }
        }
    } else {
        while digits.len() < significant {
            digits.push(b'0');
        }
    }
    let text = if exponent < -6 || exponent >= significant as i32 {
        let head = digits[0] as char;
        let tail: String = digits[1..].iter().map(|b| *b as char).collect();
        let body = if tail.is_empty() {
            head.to_string()
        } else {
            format!("{}.{}", head, tail)
        };
        let sign = if exponent >= 0 { "+" } else { "-" };
        format!("{}e{}{}", body, sign, exponent.abs())
    } else if exponent >= 0 {
        let integer_len = (exponent + 1) as usize;
        if integer_len >= significant {
            let mut body: String = digits.iter().map(|b| *b as char).collect();
            body.push_str(&"0".repeat(integer_len - significant));
            body
        } else {
            let head: String = digits[..integer_len].iter().map(|b| *b as char).collect();
            let tail: String = digits[integer_len..].iter().map(|b| *b as char).collect();
            format!("{}.{}", head, tail)
        }
    } else {
        let body: String = digits.iter().map(|b| *b as char).collect();
        format!("0.{}{}", "0".repeat((-exponent - 1) as usize), body)
    };
    if negative { format!("-{}", text) } else { text }
}

pub fn Data_Number_Format_toExponentialNative(digits: i64, number: f64) -> String {
    if let Some(text) = special(number) {
        return text;
    }
    let places = digits.max(0) as usize;
    let negative = number < 0.0;
    let formatted = format!("{:.*e}", places, number.abs());
    let (mantissa, exponent) = formatted.split_once('e').expect("exponent notation");
    let exponent: i32 = exponent.parse().expect("decimal exponent");
    let sign = if exponent >= 0 { "+" } else { "-" };
    let text = format!("{}e{}{}", mantissa, sign, exponent.abs());
    if negative { format!("-{}", text) } else { text }
}
