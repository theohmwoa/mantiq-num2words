mod en;
mod fr;
mod ar;
mod currency;

pub use currency::{Currency, CurrencyInfo};

pub trait SpellOut {
    fn cardinal(&self, n: i64) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    Fr,
    Ar,
}

impl Lang {
    fn spell_out(&self) -> &dyn SpellOut {
        match self {
            Lang::En => &en::English,
            Lang::Fr => &fr::French,
            Lang::Ar => &ar::Arabic,
        }
    }
}

pub struct Num2Words {
    lang: Lang,
}

impl Num2Words {
    pub fn new(lang: Lang) -> Self {
        Self { lang }
    }

    pub fn cardinal(&self, n: i64) -> String {
        self.lang.spell_out().cardinal(n)
    }

    /// Spell out a monetary amount.
    ///
    /// Example: `spell_currency(8000.80, Currency::AED)` with `Lang::En`
    /// -> "Eight Thousand Dirhams and Eighty Fils"
    pub fn spell_currency(&self, amount: f64, currency: Currency) -> String {
        let info = currency.info(self.lang);
        let is_negative = amount < 0.0;
        let amount = amount.abs();

        // Split into major and minor units, rounding to avoid float issues
        let cents_total = (amount * 100.0).round() as u64;
        let major = (cents_total / 100) as i64;
        let minor = cents_total % 100;

        let spell = self.lang.spell_out();
        let major_words = spell.cardinal(major);
        let minor_words = spell.cardinal(minor as i64);

        let major_unit = if major == 1 { info.major_singular } else { info.major_plural };
        let minor_unit = if minor == 1 { info.minor_singular } else { info.minor_plural };

        let and_word = match self.lang {
            Lang::En => "and",
            Lang::Fr => "et",
            Lang::Ar => "\u{0648}",
        };

        let result = if minor == 0 {
            format!("{} {}", title_case(&major_words), major_unit)
        } else if major == 0 {
            format!("{} {}", title_case(&minor_words), minor_unit)
        } else {
            format!(
                "{} {} {} {} {}",
                title_case(&major_words),
                major_unit,
                and_word,
                title_case(&minor_words),
                minor_unit
            )
        };

        if is_negative {
            let minus_word = match self.lang {
                Lang::En => "Minus",
                Lang::Fr => "Moins",
                Lang::Ar => "\u{0633}\u{0627}\u{0644}\u{0628}",
            };
            format!("{minus_word} {result}")
        } else {
            result
        }
    }
}

fn title_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = true;
    for c in s.chars() {
        if capitalize_next && c.is_alphabetic() {
            for u in c.to_uppercase() {
                result.push(u);
            }
            capitalize_next = false;
        } else {
            result.push(c);
            if c == ' ' || c == '-' {
                capitalize_next = true;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_cardinals() {
        let n = Num2Words::new(Lang::En);
        assert_eq!(n.cardinal(0), "zero");
        assert_eq!(n.cardinal(1), "one");
        assert_eq!(n.cardinal(15), "fifteen");
        assert_eq!(n.cardinal(42), "forty-two");
        assert_eq!(n.cardinal(100), "one hundred");
        assert_eq!(n.cardinal(115), "one hundred and fifteen");
        assert_eq!(n.cardinal(1000), "one thousand");
        assert_eq!(n.cardinal(8000), "eight thousand");
        assert_eq!(n.cardinal(1_000_000), "one million");
        assert_eq!(n.cardinal(-42), "minus forty-two");
    }

    #[test]
    fn english_currency_aed() {
        let n = Num2Words::new(Lang::En);
        assert_eq!(
            n.spell_currency(8000.80, Currency::AED),
            "Eight Thousand Dirhams and Eighty Fils"
        );
    }

    #[test]
    fn english_currency_usd() {
        let n = Num2Words::new(Lang::En);
        assert_eq!(
            n.spell_currency(1234.56, Currency::USD),
            "One Thousand Two Hundred And Thirty-Four Dollars and Fifty-Six Cents"
        );
    }

    #[test]
    fn english_currency_no_minor() {
        let n = Num2Words::new(Lang::En);
        assert_eq!(
            n.spell_currency(500.00, Currency::EUR),
            "Five Hundred Euros"
        );
    }

    #[test]
    fn french_cardinals() {
        let n = Num2Words::new(Lang::Fr);
        assert_eq!(n.cardinal(0), "z\u{00e9}ro");
        assert_eq!(n.cardinal(71), "soixante-et-onze");
        assert_eq!(n.cardinal(80), "quatre-vingts");
        assert_eq!(n.cardinal(91), "quatre-vingt-onze");
        assert_eq!(n.cardinal(200), "deux cents");
        assert_eq!(n.cardinal(201), "deux cent un");
    }

    #[test]
    fn french_currency() {
        let n = Num2Words::new(Lang::Fr);
        assert_eq!(
            n.spell_currency(1500.50, Currency::EUR),
            "Mille Cinq Cents Euros et Cinquante Centimes"
        );
    }

    #[test]
    fn arabic_cardinals() {
        let n = Num2Words::new(Lang::Ar);
        assert_eq!(n.cardinal(0), "\u{0635}\u{0641}\u{0631}");
        assert_eq!(n.cardinal(1), "\u{0648}\u{0627}\u{062d}\u{062f}");
        assert_eq!(n.cardinal(5), "\u{062e}\u{0645}\u{0633}\u{0629}");
    }

    #[test]
    fn arabic_currency_aed() {
        let n = Num2Words::new(Lang::Ar);
        let result = n.spell_currency(8000.80, Currency::AED);
        // Should contain "دراهم" (dirhams in Arabic)
        assert!(result.contains("\u{062f}\u{0631}\u{0627}\u{0647}\u{0645}"));
    }

    // ==========================================
    // English edge cases
    // ==========================================

    #[test]
    fn english_teens() {
        // 10-19 are irregular, not composed from tens+ones
        let n = Num2Words::new(Lang::En);
        assert_eq!(n.cardinal(10), "ten");
        assert_eq!(n.cardinal(11), "eleven");
        assert_eq!(n.cardinal(12), "twelve");
        assert_eq!(n.cardinal(13), "thirteen");
        assert_eq!(n.cardinal(19), "nineteen");
    }

    #[test]
    fn english_round_tens() {
        let n = Num2Words::new(Lang::En);
        assert_eq!(n.cardinal(20), "twenty");
        assert_eq!(n.cardinal(30), "thirty");
        assert_eq!(n.cardinal(90), "ninety");
    }

    #[test]
    fn english_hundred_boundaries() {
        // "and" insertion: 101 = "one hundred and one", not "one hundred one"
        let n = Num2Words::new(Lang::En);
        assert_eq!(n.cardinal(100), "one hundred");
        assert_eq!(n.cardinal(101), "one hundred and one");
        assert_eq!(n.cardinal(110), "one hundred and ten");
        assert_eq!(n.cardinal(111), "one hundred and eleven");
        assert_eq!(n.cardinal(999), "nine hundred and ninety-nine");
    }

    #[test]
    fn english_thousand_and_insertion() {
        // "and" before last group when it's < 100
        let n = Num2Words::new(Lang::En);
        assert_eq!(n.cardinal(1001), "one thousand and one");
        assert_eq!(n.cardinal(1010), "one thousand and ten");
        assert_eq!(n.cardinal(1099), "one thousand and ninety-nine");
        // No "and" when last group >= 100
        assert_eq!(n.cardinal(1100), "one thousand one hundred");
        assert_eq!(n.cardinal(1234), "one thousand two hundred and thirty-four");
    }

    #[test]
    fn english_large_numbers() {
        let n = Num2Words::new(Lang::En);
        assert_eq!(n.cardinal(1_000_000), "one million");
        assert_eq!(n.cardinal(1_000_001), "one million and one");
        assert_eq!(n.cardinal(1_001_000), "one million one thousand");
        assert_eq!(
            n.cardinal(1_234_567),
            "one million two hundred and thirty-four thousand five hundred and sixty-seven"
        );
        assert_eq!(n.cardinal(1_000_000_000), "one billion");
        assert_eq!(n.cardinal(1_000_000_000_000), "one trillion");
    }

    #[test]
    fn english_negative() {
        let n = Num2Words::new(Lang::En);
        assert_eq!(n.cardinal(-1), "minus one");
        assert_eq!(n.cardinal(-1000), "minus one thousand");
    }

    #[test]
    fn english_currency_singular() {
        // 1 Dollar (not Dollars), 1 Cent (not Cents)
        let n = Num2Words::new(Lang::En);
        assert_eq!(
            n.spell_currency(1.01, Currency::USD),
            "One Dollar and One Cent"
        );
    }

    #[test]
    fn english_currency_only_minor() {
        let n = Num2Words::new(Lang::En);
        assert_eq!(
            n.spell_currency(0.99, Currency::USD),
            "Ninety-Nine Cents"
        );
    }

    #[test]
    fn english_currency_negative() {
        let n = Num2Words::new(Lang::En);
        assert_eq!(
            n.spell_currency(-50.00, Currency::USD),
            "Minus Fifty Dollars"
        );
    }

    #[test]
    fn english_currency_float_rounding() {
        // 19.99 can be tricky with floating point (19.990000000000002)
        let n = Num2Words::new(Lang::En);
        assert_eq!(
            n.spell_currency(19.99, Currency::USD),
            "Nineteen Dollars and Ninety-Nine Cents"
        );
        // 0.10 is a classic float trap
        assert_eq!(
            n.spell_currency(0.10, Currency::USD),
            "Ten Cents"
        );
    }

    // ==========================================
    // French edge cases
    // ==========================================

    #[test]
    fn french_seventies() {
        // 70-79 = soixante + (10..19) — the famous French quirk
        let n = Num2Words::new(Lang::Fr);
        assert_eq!(n.cardinal(70), "soixante-dix");
        assert_eq!(n.cardinal(71), "soixante-et-onze");
        assert_eq!(n.cardinal(72), "soixante-douze");
        assert_eq!(n.cardinal(75), "soixante-quinze");
        assert_eq!(n.cardinal(79), "soixante-dix-neuf");
    }

    #[test]
    fn french_eighties() {
        // 80 = quatre-vingts (with S), 81-89 = quatre-vingt-X (no S)
        let n = Num2Words::new(Lang::Fr);
        assert_eq!(n.cardinal(80), "quatre-vingts");
        assert_eq!(n.cardinal(81), "quatre-vingt-un");
        assert_eq!(n.cardinal(85), "quatre-vingt-cinq");
        assert_eq!(n.cardinal(89), "quatre-vingt-neuf");
    }

    #[test]
    fn french_nineties() {
        // 90-99 = quatre-vingt + (10..19)
        let n = Num2Words::new(Lang::Fr);
        assert_eq!(n.cardinal(90), "quatre-vingt-dix");
        assert_eq!(n.cardinal(91), "quatre-vingt-onze");
        assert_eq!(n.cardinal(95), "quatre-vingt-quinze");
        assert_eq!(n.cardinal(99), "quatre-vingt-dix-neuf");
    }

    #[test]
    fn french_et_un_variants() {
        // 21, 31, 41, 51, 61 use "et un", but 81 and 91 do NOT
        let n = Num2Words::new(Lang::Fr);
        assert_eq!(n.cardinal(21), "vingt-et-un");
        assert_eq!(n.cardinal(31), "trente-et-un");
        assert_eq!(n.cardinal(41), "quarante-et-un");
        assert_eq!(n.cardinal(51), "cinquante-et-un");
        assert_eq!(n.cardinal(61), "soixante-et-un");
        // 81 and 91: no "et"
        assert_eq!(n.cardinal(81), "quatre-vingt-un");
        assert_eq!(n.cardinal(91), "quatre-vingt-onze");
    }

    #[test]
    fn french_hundreds_plural_rule() {
        // "cents" (plural) only when exact multiple: 200 = "deux cents", 201 = "deux cent un"
        let n = Num2Words::new(Lang::Fr);
        assert_eq!(n.cardinal(100), "cent");
        assert_eq!(n.cardinal(200), "deux cents");
        assert_eq!(n.cardinal(201), "deux cent un");
        assert_eq!(n.cardinal(300), "trois cents");
        assert_eq!(n.cardinal(301), "trois cent un");
        assert_eq!(n.cardinal(999), "neuf cent quatre-vingt-dix-neuf");
    }

    #[test]
    fn french_mille_no_un() {
        // 1000 = "mille" (never "un mille"), but 2000 = "deux mille"
        let n = Num2Words::new(Lang::Fr);
        assert_eq!(n.cardinal(1000), "mille");
        assert_eq!(n.cardinal(2000), "deux mille");
        assert_eq!(n.cardinal(1001), "mille un");
        assert_eq!(n.cardinal(2001), "deux mille un");
    }

    #[test]
    fn french_millions_plural() {
        // "million" takes S when plural: "deux millions"
        let n = Num2Words::new(Lang::Fr);
        assert_eq!(n.cardinal(1_000_000), "un million");
        assert_eq!(n.cardinal(2_000_000), "deux millions");
        assert_eq!(n.cardinal(2_000_001), "deux millions un");
    }

    #[test]
    fn french_large_composed() {
        let n = Num2Words::new(Lang::Fr);
        assert_eq!(
            n.cardinal(1_999_999),
            "un million neuf cent quatre-vingt-dix-neuf mille neuf cent quatre-vingt-dix-neuf"
        );
    }

    // ==========================================
    // Arabic edge cases
    // ==========================================

    #[test]
    fn arabic_ones_before_tens() {
        // Arabic: ones come BEFORE tens with و (and) — e.g. 25 = "خمسة وعشرون"
        let n = Num2Words::new(Lang::Ar);
        let result = n.cardinal(25);
        // Should have ones (خمسة) before tens (عشرون)
        let five_pos = result.find("\u{062e}\u{0645}\u{0633}\u{0629}").unwrap();
        let twenty_pos = result.find("\u{0639}\u{0634}\u{0631}\u{0648}\u{0646}").unwrap();
        assert!(five_pos < twenty_pos, "Arabic: ones should come before tens");
    }

    #[test]
    fn arabic_dual_form() {
        // 2 = اثنان, 2000 = ألفان (dual), 200 = مائتان (dual)
        let n = Num2Words::new(Lang::Ar);
        assert_eq!(n.cardinal(2), "\u{0627}\u{062b}\u{0646}\u{0627}\u{0646}"); // اثنان
        assert_eq!(n.cardinal(200), "\u{0645}\u{0627}\u{0626}\u{062a}\u{0627}\u{0646}"); // مائتان
        assert_eq!(n.cardinal(2000), "\u{0623}\u{0644}\u{0641}\u{0627}\u{0646}"); // ألفان
    }

    #[test]
    fn arabic_thousands_3_to_10() {
        // 3000-10000 use plural آلاف
        let n = Num2Words::new(Lang::Ar);
        let result_3k = n.cardinal(3000);
        assert!(result_3k.contains("\u{0622}\u{0644}\u{0627}\u{0641}"), "3000 should use آلاف");
        let result_10k = n.cardinal(10000);
        assert!(result_10k.contains("\u{0622}\u{0644}\u{0627}\u{0641}"), "10000 should use آلاف");
    }

    #[test]
    fn arabic_thousands_above_10() {
        // 11000+ uses singular ألف
        let n = Num2Words::new(Lang::Ar);
        let result = n.cardinal(11000);
        assert!(result.contains("\u{0623}\u{0644}\u{0641}"), "11000 should use singular ألف");
    }

    #[test]
    fn arabic_hundreds() {
        let n = Num2Words::new(Lang::Ar);
        assert_eq!(n.cardinal(100), "\u{0645}\u{0627}\u{0626}\u{0629}"); // مائة
        assert_eq!(n.cardinal(200), "\u{0645}\u{0627}\u{0626}\u{062a}\u{0627}\u{0646}"); // مائتان
        // 300 = ثلاثة مائة
        let result_300 = n.cardinal(300);
        assert!(result_300.contains("\u{062b}\u{0644}\u{0627}\u{062b}\u{0629}")); // ثلاثة
        assert!(result_300.contains("\u{0645}\u{0627}\u{0626}\u{0629}")); // مائة
    }

    #[test]
    fn arabic_composed_number() {
        // 1234 — tests hundreds + tens + ones + thousands together
        let n = Num2Words::new(Lang::Ar);
        let result = n.cardinal(1234);
        // Should contain ألف (thousand) and مائتان or مائة and some digits
        assert!(result.contains("\u{0623}\u{0644}\u{0641}"), "1234 should contain ألف");
    }

    #[test]
    fn arabic_negative() {
        let n = Num2Words::new(Lang::Ar);
        let result = n.cardinal(-5);
        assert!(result.contains("\u{0633}\u{0627}\u{0644}\u{0628}")); // سالب
        assert!(result.contains("\u{062e}\u{0645}\u{0633}\u{0629}")); // خمسة
    }

    #[test]
    fn arabic_currency_singular() {
        // 1 dirham = درهم (singular), not دراهم
        let n = Num2Words::new(Lang::Ar);
        let result = n.spell_currency(1.00, Currency::AED);
        assert!(result.contains("\u{062f}\u{0631}\u{0647}\u{0645}")); // درهم
        assert!(!result.contains("\u{062f}\u{0631}\u{0627}\u{0647}\u{0645}")); // NOT دراهم
    }

    // ==========================================
    // Cross-language edge cases
    // ==========================================

    #[test]
    fn all_languages_zero() {
        assert_eq!(Num2Words::new(Lang::En).cardinal(0), "zero");
        assert_eq!(Num2Words::new(Lang::Fr).cardinal(0), "z\u{00e9}ro");
        assert_eq!(Num2Words::new(Lang::Ar).cardinal(0), "\u{0635}\u{0641}\u{0631}");
    }

    #[test]
    fn all_languages_max_reasonable() {
        // Make sure large numbers don't panic
        let big = 999_999_999_999;
        for lang in [Lang::En, Lang::Fr, Lang::Ar] {
            let n = Num2Words::new(lang);
            let result = n.cardinal(big);
            assert!(!result.is_empty(), "Should produce output for {big} in {lang:?}");
        }
    }

    #[test]
    fn currency_zero_amount() {
        // Edge case: 0.00 should still produce something sensible
        let n = Num2Words::new(Lang::En);
        let result = n.spell_currency(0.00, Currency::USD);
        assert_eq!(result, "Zero Dollars");
    }
}
