# Advent of Code 2023

These are my solutions for Advent of Code 2023 in Rust.

## Day 1 - Trebuchet

The first part was easy, just iterate over all the lines and find the first and last digit in the line. For example: `pas24threecmds45foursd`, the 2 digits are `2` and `5`.

For part two, the digits could also be in word form (as in `two`, `three`, etc.). For example:  `pas24threecmds45foursd`, the 2 digits are `2` and `4`.

Once you find the two digits, you should return the number they form: `25` or `24` respectively.

Then calculate the sum of these numbers over the whole file.
