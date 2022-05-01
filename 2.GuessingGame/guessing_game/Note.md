- Add `crate` to the `Cargo.toml` file: `rand = '0.8.3'` is actually shorthand for ^0.8.3, which means any version that is at least 0.8.3 but below 0.9.0
- The parse method will only work on characters that can logically be converted into numbers and so can easily cause errors.
                    