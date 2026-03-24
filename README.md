# mantiq_num2words

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

```
running 8 tests
test tests::arabic_cardinals ... ok
test tests::arabic_currency_aed ... ok
test tests::english_cardinals ... ok
test tests::english_currency_aed ... ok
test tests::english_currency_no_minor ... ok
test tests::english_currency_usd ... ok
test tests::french_cardinals ... ok
test tests::french_currency ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## License

MIT
