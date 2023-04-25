# Dev
clone the [./src/other/this_is_template.rs](./src/other/this_is_template.rs)
as the starter template for your function


# Test

```sh
cargo test <fn_name> -- --color always --nocapture
# This is ok too
cargo test -- --color always --nocapture <fn_name>
```
--nocapture will show the println!() output

Eg:
```
$ cargo test -- --color always --nocapture id00xx_

test leetcode::id00xx_add_two_numbers::tests::id00xx_bench ... ok
test leetcode::id00xx_add_two_numbers::tests::id00xx_stress ... ok
test leetcode::id00xx_add_two_numbers::tests::id00xx_manual ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

$ cargo test -- --color always --nocapture id00xx_manual
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s
```
