## Letter Combinations of a Phone Number

Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in **any order**.

A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

<p align="left">
  <img src="/assets/Telephone-keypad2.png" alt= "telephone keypad">"
</p>

Note: *whoever made this fucking repo didn't put the assests in the same folder as the projects so if I opend the workspace from folder i can't see the image, but if I open the workspace to include the scope of the assets rust analyzer doesnt work unless I add ***100*** unique fucking paths to my .json settings.*

```rust
t9 mapping:
(1, ""),
(2, "abc"),
(3, "def"),
(4, "ghi"),
(5, "jkl"),
(6, "mno"),
(7, "pqrs"),
(8, "tuv"),
(9, "wxyz"),
(0, ""),
```

### Examples

**Example 1:**

```rust
Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
```

**Example 2:**

```rust
Input: digits = ""
Output: []
```

**Example 3:**

```rust 
Input: digits = "2"
Output: ["a","b","c"]
```

### Constraints

- `0 <= digits.length <= 4`
- `digits[i] is a digit in the range ['2', '9'].`

---

### Notes

- N/A
