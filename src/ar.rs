use crate::SpellOut;

pub struct Arabic;

const ONES: [&str; 20] = [
    "\u{0635}\u{0641}\u{0631}",           // صفر
    "\u{0648}\u{0627}\u{062d}\u{062f}",   // واحد
    "\u{0627}\u{062b}\u{0646}\u{0627}\u{0646}", // اثنان
    "\u{062b}\u{0644}\u{0627}\u{062b}\u{0629}", // ثلاثة
    "\u{0623}\u{0631}\u{0628}\u{0639}\u{0629}", // أربعة
    "\u{062e}\u{0645}\u{0633}\u{0629}",   // خمسة
    "\u{0633}\u{062a}\u{0629}",           // ستة
    "\u{0633}\u{0628}\u{0639}\u{0629}",   // سبعة
    "\u{062b}\u{0645}\u{0627}\u{0646}\u{064a}\u{0629}", // ثمانية
    "\u{062a}\u{0633}\u{0639}\u{0629}",   // تسعة
    "\u{0639}\u{0634}\u{0631}\u{0629}",   // عشرة
    "\u{0623}\u{062d}\u{062f} \u{0639}\u{0634}\u{0631}", // أحد عشر
    "\u{0627}\u{062b}\u{0646}\u{0627} \u{0639}\u{0634}\u{0631}", // اثنا عشر
    "\u{062b}\u{0644}\u{0627}\u{062b}\u{0629} \u{0639}\u{0634}\u{0631}", // ثلاثة عشر
    "\u{0623}\u{0631}\u{0628}\u{0639}\u{0629} \u{0639}\u{0634}\u{0631}", // أربعة عشر
    "\u{062e}\u{0645}\u{0633}\u{0629} \u{0639}\u{0634}\u{0631}", // خمسة عشر
    "\u{0633}\u{062a}\u{0629} \u{0639}\u{0634}\u{0631}",   // ستة عشر
    "\u{0633}\u{0628}\u{0639}\u{0629} \u{0639}\u{0634}\u{0631}", // سبعة عشر
    "\u{062b}\u{0645}\u{0627}\u{0646}\u{064a}\u{0629} \u{0639}\u{0634}\u{0631}", // ثمانية عشر
    "\u{062a}\u{0633}\u{0639}\u{0629} \u{0639}\u{0634}\u{0631}", // تسعة عشر
];

const TENS: [&str; 10] = [
    "",
    "",
    "\u{0639}\u{0634}\u{0631}\u{0648}\u{0646}",   // عشرون
    "\u{062b}\u{0644}\u{0627}\u{062b}\u{0648}\u{0646}", // ثلاثون
    "\u{0623}\u{0631}\u{0628}\u{0639}\u{0648}\u{0646}", // أربعون
    "\u{062e}\u{0645}\u{0633}\u{0648}\u{0646}",   // خمسون
    "\u{0633}\u{062a}\u{0648}\u{0646}",           // ستون
    "\u{0633}\u{0628}\u{0639}\u{0648}\u{0646}",   // سبعون
    "\u{062b}\u{0645}\u{0627}\u{0646}\u{0648}\u{0646}", // ثمانون
    "\u{062a}\u{0633}\u{0639}\u{0648}\u{0646}",   // تسعون
];

const HUNDRED: &str = "\u{0645}\u{0627}\u{0626}\u{0629}"; // مائة
const TWO_HUNDRED: &str = "\u{0645}\u{0627}\u{0626}\u{062a}\u{0627}\u{0646}"; // مائتان
const THOUSAND: &str = "\u{0623}\u{0644}\u{0641}"; // ألف
const TWO_THOUSAND: &str = "\u{0623}\u{0644}\u{0641}\u{0627}\u{0646}"; // ألفان
const THOUSANDS: &str = "\u{0622}\u{0644}\u{0627}\u{0641}"; // آلاف
const MILLION: &str = "\u{0645}\u{0644}\u{064a}\u{0648}\u{0646}"; // مليون
const MILLIONS: &str = "\u{0645}\u{0644}\u{0627}\u{064a}\u{064a}\u{0646}"; // ملايين
const BILLION: &str = "\u{0645}\u{0644}\u{064a}\u{0627}\u{0631}"; // مليار
const BILLIONS: &str = "\u{0645}\u{0644}\u{064a}\u{0627}\u{0631}\u{0627}\u{062a}"; // مليارات

const AND: &str = " \u{0648}"; // و

impl SpellOut for Arabic {
    fn cardinal(&self, n: i64) -> String {
        if n < 0 {
            return format!("\u{0633}\u{0627}\u{0644}\u{0628} {}", self.cardinal(-n));
        }
        if n == 0 {
            return ONES[0].to_string();
        }
        spell_positive(n as u64).trim().to_string()
    }
}

fn spell_positive(n: u64) -> String {
    if n == 0 {
        return String::new();
    }

    let mut parts: Vec<String> = Vec::new();

    let billions = n / 1_000_000_000;
    let millions = (n % 1_000_000_000) / 1_000_000;
    let thousands = (n % 1_000_000) / 1_000;
    let remainder = n % 1_000;

    if billions > 0 {
        parts.push(spell_scale(billions, BILLION, BILLIONS));
    }
    if millions > 0 {
        parts.push(spell_scale(millions, MILLION, MILLIONS));
    }
    if thousands > 0 {
        parts.push(spell_thousands(thousands));
    }
    if remainder > 0 {
        parts.push(spell_under_1000(remainder as u32));
    }

    parts.join(AND)
}

fn spell_scale(count: u64, singular: &str, plural: &str) -> String {
    match count {
        1 => singular.to_string(),
        2 => format!("{singular}\u{0627}\u{0646}"), // dual: add ان
        3..=10 => format!("{} {plural}", spell_under_1000(count as u32)),
        _ => format!("{} {singular}", spell_under_1000(count as u32)),
    }
}

fn spell_thousands(count: u64) -> String {
    match count {
        1 => THOUSAND.to_string(),
        2 => TWO_THOUSAND.to_string(),
        3..=10 => format!("{} {THOUSANDS}", spell_under_1000(count as u32)),
        _ => format!("{} {THOUSAND}", spell_under_1000(count as u32)),
    }
}

fn spell_under_1000(n: u32) -> String {
    debug_assert!(n > 0 && n < 1000);

    let hundreds = n / 100;
    let rest = n % 100;

    let h_part = match hundreds {
        0 => None,
        1 => Some(HUNDRED.to_string()),
        2 => Some(TWO_HUNDRED.to_string()),
        h => Some(format!("{} {HUNDRED}", ONES[h as usize])),
    };

    let r_part = if rest > 0 {
        Some(spell_under_100(rest))
    } else {
        None
    };

    match (h_part, r_part) {
        (Some(h), Some(r)) => format!("{h}{AND}{r}"),
        (Some(h), None) => h,
        (None, Some(r)) => r,
        (None, None) => unreachable!(),
    }
}

fn spell_under_100(n: u32) -> String {
    debug_assert!(n > 0 && n < 100);

    if n < 20 {
        return ONES[n as usize].to_string();
    }

    let tens = n / 10;
    let ones = n % 10;

    if ones == 0 {
        TENS[tens as usize].to_string()
    } else {
        // Arabic: ones before tens with و (and)
        format!("{}{AND}{}", ONES[ones as usize], TENS[tens as usize])
    }
}
