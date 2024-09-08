# Restore IP Addresses using Rust Structs and Implementations

Given a string `s` containing only digits, return all possible valid IP addresses that can be obtained from `s`. You can return them in **any** order.

## Task

Your task is to implement this solution using Rust structs and implementations. Here's a suggested structure:

1. Create an `IpAddress` struct to represent a valid IP address.
2. Implement methods for the `IpAddress` struct to:
   - Check if a given string can be a valid IP address.
   - Convert a string into an `IpAddress` instance.
   - Format the `IpAddress` as a string.
3. Create an `IpRestorer` struct to handle the restoration process.
4. Implement methods for the `IpRestorer` struct to:
   - Take the input string and generate all possible IP addresses.
   - Validate each generated IP address using the `IpAddress` struct.

## Struct Definitions

Start with these struct definitions:

```rust
struct IpAddress {
    octets: [u8; 4],
}

struct IpRestorer {
    input: String,
}
```

Implement methods for these structs to solve the problem.

## Expected Function Signature

```rust
impl IpRestorer {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        // Your implementation here
    }
}
```

## Examples

**Example 1:**

```text
Input: s = "25525511135"
Output: ["255.255.11.135","255.255.111.35"]
```

**Example 2:**

```text
Input: s = "0000"
Output: ["0.0.0.0"]
```

**Example 3:**

```text
Input: s = "1111"
Output: ["1.1.1.1"]
```

**Example 4:**

```text
Input: s = "010010"
Output: ["0.10.0.10","0.100.1.0"]
```

**Example 5:**

```text
Input: s = "101023"
Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
```

## Constraints

- `0 <= s.length`
- `s` consists of digits only.
- IPv4 format only.

## Notes

- Use all digits in the input string `s` to obtain each valid IP.
- Practice creating methods for your structs using the `impl` keyword.
- Consider implementing the `Display` trait for your `IpAddress` struct for easy formatting.
- Use Rust's ownership and borrowing rules effectively in your implementations.
- A **valid IP address** consists of exactly four integers, each integer is between 0 and 255, separated by single dots and cannot have leading zeros. For example, "0.1.2.201" and "192.168.1.1" are **valid** IP addresses and "0.011.255.245", "192.168.1.312" and "192.168@1.1" are **invalid** IP addresses.
