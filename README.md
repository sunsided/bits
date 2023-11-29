# bits

Converts input into bits (with explanations, where applicable).

## Example

When run with e.g. `bits -0.3`, the output is

```
f16:  1011010011001101
      SEEEEEMMMMMMMMMM
      S: Sign (1 bit)
      E: Exponent (5 bits)
      M: Fraction / Mantissa (10 bits)
bf16: 1011111010011010
      SEEEEEEEEMMMMMMM
      S: Sign (1 bit)
      E: Exponent (8 bits)
      M: Fraction / Mantissa (7 bits)
f32:  10111110100110011001100110011010
      SEEEEEEEEMMMMMMMMMMMMMMMMMMMMMMM
      S: Sign (1 bit)
      E: Exponent (8 bits)
      M: Fraction / Mantissa (23 bits)
f64:  1011111111010011001100110011001100110011001100110011001100110011
      SEEEEEEEEEEEMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
      S: Sign (1 bit)
      E: Exponent (11 bits)
      M: Fraction / Mantissa (52 bits)
```

If only some types are required, the `--type=...` option can be used, e.g. `bin --type=f16,f32 -0.3` which gives

```
f16:  1011010011001101
      SEEEEEMMMMMMMMMM
      S: Sign (1 bit)
      E: Exponent (5 bits)
      M: Fraction / Mantissa (10 bits)
f32:  10111110100110011001100110011010
      SEEEEEEEEMMMMMMMMMMMMMMMMMMMMMMM
      S: Sign (1 bit)
      E: Exponent (8 bits)
      M: Fraction / Mantissa (23 bits)
```

### Full, short and very short format

By default, `--display=full` is implied, showing all information. If only type and bit information are wished,
`--display=short` can be used:

```
f16:  1011010011001101
f32:  10111110100110011001100110011010
```

If only the bits are wished for, the `--display=very-short` format exists, removing even the type information:

```
1011010011001101
10111110100110011001100110011010
```

The very short format makes most sense in combination with specific types; in the example above, `--type=f16,f32`
was used.
