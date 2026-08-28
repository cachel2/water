# 14 · Tests

```water
fn max(a: i32, b: i32) -> i32 { if (a > b) { a } else { b } }

test "max picks the larger" {
    assert_eq(max(3, 7), 7);
    assert_eq(max(7, 3), 7);
    assert_eq(max(-1, -9), -1);
}
```

`test "name" { }` is a language item: parsed, checked, compiled only for `water test`, stripped from every other build; usable with its module's private items. Doc examples in `///` comments compile and run as tests. `water test` runs each test **twice — native and under the interpreter** — and reports both; deterministic order, isolated processes, no shared globals; a failing assertion prints the values. A test fails on a failed assertion, a trap, or — under the interpreter — a leak. The double run is not a verification programme; **the interpreter run is your sanitizer** (§16), free because you were going to write the test anyway.
