# Currency Converter

Helps user to convert USD, JPY, EUR against each other

## What the Software Can Do

Converts:

* USD -- JPY
* USD -- EUR
* EUR -- JPY

All converts vice versa
> **Restrictions**
>
> 1. Does *NOT* support *negative number*
> 2. Maximum input for each currency:
>       - USD: 9999.99 
>       - EUR: 9999.99
>       - JPY: 999,999
> 3. The Output of fractions are to the hundreth `e.g. $15.75, $1.23`
> 4. *Both* input and output's fraction after hundreth will be cut off

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
