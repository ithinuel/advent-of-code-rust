# Reverse engineering

```text
If w1 != 14 {
 z = w1
}
If w2 != (z%26 + 13) {
  z = 26*z + w2 + 12
}
if w3 != (z%26 + 15) {
  z = 26*z + w3 + 14
}
if w4 != (z%26 + 13) {
  z = 26*z + w4
}
x = z % 26 // x may be w4 or (w3+14) or (w2+12) or (w1)
z = z / 26
if w5 != (x - 2) {
  z = 26*z + w5 + 3
}
if w6 != (z%26 + 10) {
  z = 26*z + w6 + 15
}
if w7 != (z%26 + 13) {
  z = 26*z + w7 + 11
}
x = z % 26 // x may be (w7+11) or (w6+15) or (w5+3) or (w4) or (w3+14) or (w2+12) or (w1)
z = z / 26
if w8 != (x - 15) {
  z = 26*z + w8 + 12
}
if w9 != (z%26 + 11) {
  z = 26*z + w9 + 1
}

x = z % 26
// x may be (w9+1) or (w8+12) or (w7+11) or (w6+15) or (w5+3) or (w4) or (w3+14) or (w2+12) or (w1)
z = z / 26
if w10 != (x - 9) {
  z = 26*z + w10 + 12
}

x = z % 26
// x may be (w10+12) or (w9+1) or (w8+12) or (w7+11) or (w6+15) or (w5+3) or (w4) or (w3+14) or (w2+12)
z = z / 26
if w11 != (x - 9) {
  z = 26*z + w11 + 3
}

x = z % 26
// x may be (w11+3) or (w9+1) or (w8+12) or (w7+11) or (w6+15) or (w5+3) or (w4) or (w3+14) or (w2+12)
z = z / 26
if w12 != (x - 7) {
  z = 26*z + w12 + 10
}

x = z % 26
// x may be (w12 + 10) or (w9+1) or (w8+12) or (w7+11) or (w6+15) or (w5+3) or (w4) or (w3+14)%26 or
// (w2+12)
z = z / 26
if w13 != (x - 4) {
  z  = 26*z + w13 + 14
}

x = z % 26
// x may be (w13 + 14) or (w9+1) or (w8+12) or (w7+11) or (w6+15) or (w5+3) or (w4) or (w3+14)%26 or
// (w2+12)
z = z / 26
if w14 != (x - 6) {
  z = 26*z + w14 + 12
}
```

`z = 26 * z + input(n) + offset` is equivalent to stacking the input + an offset
`x = z % 26` is equivalent to popping that stacked value.
We need to find inputs so that the stack is empty by the end of the program

Reducing the program with input range (1..=9) we conclude the following constraints:

```text
W5  = w4 - 2
W8  = w7 - 4
W10 = w9 - 8
W11 = w6 + 6
W12 = w3 + 7
W13 = w2 + 8
W14 = w1 - 6
```

Therefore my solution for part 1 is:

```text
1 2 3 4 5 6 7 8 9 a b c d e
9 1 2 9 7 3 9 5 9 1 9 9 9 3
```

And part 2 is:

```text
1 2 3 4 5 6 7 8 9 a b c d e
7 1 1 3 1 1 5 1 9 1 7 8 9 1
```
