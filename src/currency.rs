use crate::Lang;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    AED, // UAE Dirham
    USD, // US Dollar
    EUR, // Euro
    GBP, // British Pound
    XOF, // West African CFA Franc
}

pub struct CurrencyInfo {
    pub major_singular: &'static str,
    pub major_plural: &'static str,
    pub minor_singular: &'static str,
    pub minor_plural: &'static str,
}

impl Currency {
    pub fn info(&self, lang: Lang) -> CurrencyInfo {
        match lang {
            Lang::En => self.info_en(),
            Lang::Fr => self.info_fr(),
            Lang::Ar => self.info_ar(),
        }
    }

    fn info_en(&self) -> CurrencyInfo {
        match self {
            Currency::AED => CurrencyInfo {
                major_singular: "Dirham",
                major_plural: "Dirhams",
                minor_singular: "Fil",
                minor_plural: "Fils",
            },
            Currency::USD => CurrencyInfo {
                major_singular: "Dollar",
                major_plural: "Dollars",
                minor_singular: "Cent",
                minor_plural: "Cents",
            },
            Currency::EUR => CurrencyInfo {
                major_singular: "Euro",
                major_plural: "Euros",
                minor_singular: "Cent",
                minor_plural: "Cents",
            },
            Currency::GBP => CurrencyInfo {
                major_singular: "Pound",
                major_plural: "Pounds",
                minor_singular: "Penny",
                minor_plural: "Pence",
            },
            Currency::XOF => CurrencyInfo {
                major_singular: "Franc",
                major_plural: "Francs",
                minor_singular: "Centime",
                minor_plural: "Centimes",
            },
        }
    }

    fn info_fr(&self) -> CurrencyInfo {
        match self {
            Currency::AED => CurrencyInfo {
                major_singular: "Dirham",
                major_plural: "Dirhams",
                minor_singular: "Fil",
                minor_plural: "Fils",
            },
            Currency::USD => CurrencyInfo {
                major_singular: "Dollar",
                major_plural: "Dollars",
                minor_singular: "Cent",
                minor_plural: "Cents",
            },
            Currency::EUR => CurrencyInfo {
                major_singular: "Euro",
                major_plural: "Euros",
                minor_singular: "Centime",
                minor_plural: "Centimes",
            },
            Currency::GBP => CurrencyInfo {
                major_singular: "Livre",
                major_plural: "Livres",
                minor_singular: "Penny",
                minor_plural: "Pence",
            },
            Currency::XOF => CurrencyInfo {
                major_singular: "Franc",
                major_plural: "Francs",
                minor_singular: "Centime",
                minor_plural: "Centimes",
            },
        }
    }

    fn info_ar(&self) -> CurrencyInfo {
        match self {
            Currency::AED => CurrencyInfo {
                major_singular: "\u{062f}\u{0631}\u{0647}\u{0645}",     // درهم
                major_plural: "\u{062f}\u{0631}\u{0627}\u{0647}\u{0645}", // دراهم
                minor_singular: "\u{0641}\u{0644}\u{0633}",             // فلس
                minor_plural: "\u{0641}\u{0644}\u{0648}\u{0633}",       // فلوس
            },
            Currency::USD => CurrencyInfo {
                major_singular: "\u{062f}\u{0648}\u{0644}\u{0627}\u{0631}", // دولار
                major_plural: "\u{062f}\u{0648}\u{0644}\u{0627}\u{0631}\u{0627}\u{062a}", // دولارات
                minor_singular: "\u{0633}\u{0646}\u{062a}",               // سنت
                minor_plural: "\u{0633}\u{0646}\u{062a}\u{0627}\u{062a}", // سنتات
            },
            Currency::EUR => CurrencyInfo {
                major_singular: "\u{064a}\u{0648}\u{0631}\u{0648}",     // يورو
                major_plural: "\u{064a}\u{0648}\u{0631}\u{0648}",       // يورو (same)
                minor_singular: "\u{0633}\u{0646}\u{062a}",             // سنت
                minor_plural: "\u{0633}\u{0646}\u{062a}\u{0627}\u{062a}", // سنتات
            },
            Currency::GBP => CurrencyInfo {
                major_singular: "\u{062c}\u{0646}\u{064a}\u{0647}",     // جنيه
                major_plural: "\u{062c}\u{0646}\u{064a}\u{0647}\u{0627}\u{062a}", // جنيهات
                minor_singular: "\u{0628}\u{0646}\u{0633}",             // بنس
                minor_plural: "\u{0628}\u{0646}\u{0633}\u{0627}\u{062a}", // بنسات
            },
            Currency::XOF => CurrencyInfo {
                major_singular: "\u{0641}\u{0631}\u{0646}\u{0643}",     // فرنك
                major_plural: "\u{0641}\u{0631}\u{0646}\u{0643}\u{0627}\u{062a}", // فرنكات
                minor_singular: "\u{0633}\u{0646}\u{062a}\u{064a}\u{0645}", // سنتيم
                minor_plural: "\u{0633}\u{0646}\u{062a}\u{064a}\u{0645}\u{0627}\u{062a}", // سنتيمات
            },
        }
    }
}
