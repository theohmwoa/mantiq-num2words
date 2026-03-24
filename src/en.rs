use crate::SpellOut;

pub struct English;

const ONES: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
    "eighteen", "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: [&str; 7] = [
    "", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion",
];

impl SpellOut for English {
    fn cardinal(&self, n: i64) -> String {
        if n < 0 {
            return format!("minus {}", self.cardinal(-n));
        }
        if n == 0 {
            return "zero".to_string();
        }
        spell_positive(n as u64).trim().to_string()
    }
}

fn spell_positive(n: u64) -> String {
    if n == 0 {
        return String::new();
    }

    let mut parts: Vec<String> = Vec::new();
    let mut remaining = n;
    let mut scale_idx = 0;

    while remaining > 0 {
        let group = (remaining % 1000) as u32;
        remaining /= 1000;

        if group > 0 {
            let group_words = spell_group(group);
            if scale_idx > 0 {
                parts.push(format!("{} {}", group_words, SCALES[scale_idx]));
            } else {
                parts.push(group_words);
            }
        }

        scale_idx += 1;
    }

    parts.reverse();

    // Insert "and" before the last part if appropriate (British/formal English style)
    if parts.len() >= 2 {
        let last = parts.last().unwrap();
        // Add "and" if the last group is less than 100 (just tens/ones)
        if n % 1000 < 100 && n % 1000 > 0 {
            let last_with_and = format!("and {last}");
            *parts.last_mut().unwrap() = last_with_and;
        }
    }

    parts.join(" ")
}

fn spell_group(n: u32) -> String {
    debug_assert!(n > 0 && n < 1000);

    let hundreds = n / 100;
    let rest = n % 100;

    match (hundreds, rest) {
        (0, r) => spell_under_100(r),
        (h, 0) => format!("{} hundred", ONES[h as usize]),
        (h, r) => format!("{} hundred and {}", ONES[h as usize], spell_under_100(r)),
    }
}

fn spell_under_100(n: u32) -> String {
    debug_assert!(n < 100);

    if n < 20 {
        ONES[n as usize].to_string()
    } else {
        let tens = n / 10;
        let ones = n % 10;
        if ones == 0 {
            TENS[tens as usize].to_string()
        } else {
            format!("{}-{}", TENS[tens as usize], ONES[ones as usize])
        }
    }
}
