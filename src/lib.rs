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
}
