# Currency Converter

**Learning Objectives**

* Familiarizing myeslf with `HashMap<K,V>`
* Getting to know how to use `std::evn::arg`

## Usage Image

Pass two abbreviation of currencies as arguments after cargo run to tell the program from what to what should the input be converted to.
```
cargo run -- 100 USD JPY (or '-- 100 usd jpy')
```
Then outputs:
```
$100 is ¥15,000!
```
---
### The Reason for Software's Existence

Help users to convert USD, JPY, EUR against each other.

### What the Software Can Do

Convert:
* USD -- JPY
* USD -- EUR
* EUR -- JPY

each converts vice versa