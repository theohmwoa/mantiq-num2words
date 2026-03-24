# mantiq_num2words

> **This is a proof of concept.** Do not use in official documents, invoices, or any production context where accuracy is critical. Language rules and currency names have not been fully verified by native speakers.

Multilingual number-to-words conversion with currency support, written in Rust.

## Features

- Cardinal number spelling (e.g. `8000` -> `"eight thousand"`)
- Currency amount spelling (e.g. `8000.80 AED` -> `"Eight Thousand Dirhams and Eighty Fils"`)
- Negative number support

## Supported Languages

| Language | Code | Notes |
|----------|------|-------|
| English  | `En` | Standard short-scale |
| French   | `Fr` | Handles 70/80/90 quirks (soixante-dix, quatre-vingts, quatre-vingt-dix) |
| Arabic   | `Ar` | Dual forms, ones-before-tens grammar |

## Supported Currencies

| Currency | Code  | Major Unit | Minor Unit |
|----------|-------|------------|------------|
| UAE Dirham | `AED` | Dirham(s) | Fil(s) |
| US Dollar | `USD` | Dollar(s) | Cent(s) |
| Euro | `EUR` | Euro(s) | Cent(s) / Centime(s) |
| British Pound | `GBP` | Pound(s) | Penny / Pence |
| West African CFA Franc | `XOF` | Franc(s) | Centime(s) |

## Usage

```rust
use mantiq_num2words::{Num2Words, Lang, Currency};

// English
let n = Num2Words::new(Lang::En);
n.cardinal(8000);                          // "eight thousand"
n.spell_currency(8000.80, Currency::AED);  // "Eight Thousand Dirhams and Eighty Fils"

// French
let n = Num2Words::new(Lang::Fr);
n.cardinal(91);                            // "quatre-vingt-onze"
n.spell_currency(1500.50, Currency::EUR);  // "Mille Cinq Cents Euros et Cinquante Centimes"

// Arabic
let n = Num2Words::new(Lang::Ar);
n.cardinal(5);                             // "خمسة"
n.spell_currency(8000.80, Currency::AED);  // "ثمانية آلاف دراهم و ثمانون فلوس"
```

## Adding a New Language

1. Create `src/<lang>.rs` with a struct implementing the `SpellOut` trait
2. Add the variant to the `Lang` enum in `src/lib.rs`
3. Wire it up in `Lang::spell_out()`
4. Add currency unit names in `src/currency.rs`

## Test Results

37 tests covering core functionality and language-specific edge cases:

```
running 37 tests
test tests::all_languages_max_reasonable ... ok
test tests::all_languages_zero ... ok
test tests::arabic_cardinals ... ok
test tests::arabic_composed_number ... ok
test tests::arabic_currency_aed ... ok
test tests::arabic_currency_singular ... ok
test tests::arabic_dual_form ... ok
test tests::arabic_hundreds ... ok
test tests::arabic_negative ... ok
test tests::arabic_ones_before_tens ... ok
test tests::arabic_thousands_3_to_10 ... ok
test tests::arabic_thousands_above_10 ... ok
test tests::currency_zero_amount ... ok
test tests::english_cardinals ... ok
test tests::english_currency_aed ... ok
test tests::english_currency_float_rounding ... ok
test tests::english_currency_negative ... ok
test tests::english_currency_no_minor ... ok
test tests::english_currency_only_minor ... ok
test tests::english_currency_singular ... ok
test tests::english_currency_usd ... ok
test tests::english_hundred_boundaries ... ok
test tests::english_large_numbers ... ok
test tests::english_negative ... ok
test tests::english_round_tens ... ok
test tests::english_teens ... ok
test tests::english_thousand_and_insertion ... ok
test tests::french_cardinals ... ok
test tests::french_currency ... ok
test tests::french_eighties ... ok
test tests::french_et_un_variants ... ok
test tests::french_hundreds_plural_rule ... ok
test tests::french_large_composed ... ok
test tests::french_mille_no_un ... ok
test tests::french_millions_plural ... ok
test tests::french_nineties ... ok
test tests::french_seventies ... ok

test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Edge cases covered

**English:** teens (11-19), "and" insertion (`one hundred and one`, `one thousand and one`), large numbers up to trillions, float rounding (`19.99`, `0.10`), singular currency (`1 Dollar`, `1 Cent`), negative amounts, zero amount

**French:** 70s (`soixante-dix`, `soixante-et-onze`), 80 plural S (`quatre-vingts` vs `quatre-vingt-un`), 90s (`quatre-vingt-dix`), `et un` only for 21/31/41/51/61 (not 81/91), hundreds plural rule (`deux cents` vs `deux cent un`), `mille` without `un`, millions plural S

**Arabic:** ones-before-tens order, dual forms (2 = اثنان, 200 = مائتان, 2000 = ألفان), thousands plural (آلاف for 3-10, ألف for 11+), singular vs plural currency names

## License

MIT
