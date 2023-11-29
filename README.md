# bits

Converts input into bits (with explanations, where applicable).

## Example

When run with e.g. `bits 0.3`, the output is

```
f32: 10111110100110011001100110011010
     SEEEEEEEEMMMMMMMMMMMMMMMMMMMMMMM
     S: Sign (1 bit)
     E: Exponent (8 bits)
     M: Fraction / Mantissa (23 bits)
f64: 1011111111010011001100110011001100110011001100110011001100110011
     SEEEEEEEEEEEMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
     S: Sign (1 bit)
     E: Exponent (11 bits)
     M: Fraction / Mantissa (52 bits)
```
