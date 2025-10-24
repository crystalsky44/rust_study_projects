# Currency Converter

Helps user to convert USD, JPY, EUR against each other


## What the Software Can Do

Convert:
* USD -- JPY
* USD -- EUR
* EUR -- JPY

All can convert vice versa


## Usage Image

After `cargo run`, pass the number of amount one wants to convert as first argument. Then, continue to pass two abbreviation of currencies as arguments to tell the program from what to what it should convert.
```
cargo run -- 100 USD JPY (or '-- 100 usd jpy')
```
Then outputs:
```
$100 is ¥15,000!
```
---
### Learning Objectives

* Familiarizing myeslf with `HashMap<K,V>`
* Getting to know how to use `std::evn::arg`