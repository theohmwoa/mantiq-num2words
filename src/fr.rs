use crate::SpellOut;

pub struct French;

const ONES: [&str; 20] = [
    "z\u{00e9}ro", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf",
    "dix", "onze", "douze", "treize", "quatorze", "quinze", "seize", "dix-sept",
    "dix-huit", "dix-neuf",
];

const TENS: [&str; 10] = [
    "", "", "vingt", "trente", "quarante", "cinquante", "soixante",
    "soixante", // 70s use 60+10..19
    "quatre-vingt", // 80
    "quatre-vingt", // 90s use 80+10..19
];

const SCALES: [&str; 7] = [
    "", "mille", "million", "milliard", "billion", "billiard", "trillion",
];

impl SpellOut for French {
    fn cardinal(&self, n: i64) -> String {
        if n < 0 {
            return format!("moins {}", self.cardinal(-n));
        }
        if n == 0 {
            return "z\u{00e9}ro".to_string();
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
            if scale_idx == 0 {
                parts.push(group_words);
            } else if scale_idx == 1 {
                // "mille" — no "un mille", just "mille" for 1000
                if group == 1 {
                    parts.push("mille".to_string());
                } else {
                    parts.push(format!("{} mille", group_words));
                }
            } else {
                // million, milliard, etc. — these take "s" for plural
                let scale = SCALES[scale_idx];
                if group == 1 {
                    parts.push(format!("un {scale}"));
                } else {
                    parts.push(format!("{} {scale}s", group_words));
                }
            }
        }

        scale_idx += 1;
    }

    parts.reverse();
    parts.join(" ")
}

fn spell_group(n: u32) -> String {
    debug_assert!(n > 0 && n < 1000);

    let hundreds = n / 100;
    let rest = n % 100;

    match (hundreds, rest) {
        (0, r) => spell_under_100(r),
        (1, 0) => "cent".to_string(),
        (h, 0) => format!("{} cents", ONES[h as usize]),
        (1, r) => format!("cent {}", spell_under_100(r)),
        (h, r) => format!("{} cent {}", ONES[h as usize], spell_under_100(r)),
    }
}

fn spell_under_100(n: u32) -> String {
    debug_assert!(n < 100);

    match n {
        0..=19 => ONES[n as usize].to_string(),
        // 21, 31, 41, 51, 61 — use "et un" (except 81, 91)
        21 => "vingt-et-un".to_string(),
        31 => "trente-et-un".to_string(),
        41 => "quarante-et-un".to_string(),
        51 => "cinquante-et-un".to_string(),
        61 => "soixante-et-un".to_string(),
        71 => "soixante-et-onze".to_string(),
        // 70-79: soixante + (10..19)
        70..80 => {
            let sub = n - 60;
            format!("soixante-{}", ONES[sub as usize])
        }
        // 80: quatre-vingts (with s when alone)
        80 => "quatre-vingts".to_string(),
        // 81-89: quatre-vingt-X (no s)
        81..90 => {
            let ones = n - 80;
            format!("quatre-vingt-{}", ONES[ones as usize])
        }
        // 90-99: quatre-vingt-dix, quatre-vingt-onze, etc.
        90..100 => {
            let sub = n - 80;
            format!("quatre-vingt-{}", ONES[sub as usize])
        }
        // Regular tens
        _ => {
            let tens = n / 10;
            let ones = n % 10;
            if ones == 0 {
                TENS[tens as usize].to_string()
            } else {
                format!("{}-{}", TENS[tens as usize], ONES[ones as usize])
            }
        }
    }
}
