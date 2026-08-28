# 19 · Examples

These examples are **normative** and are the first entries of the conformance corpus (§20.4).

## 19.1 The bigger of two

```water
fn max(a: i32, b: i32) -> i32 {
    if (a > b) { a } else { b }
}
```

Tail expression, no `return`, no `;`. `fn max<T>(a: T, b: T) -> T` does **not** compile — `T` has no `>` (§4.13).

## 19.2 The largest in an array, with a method

```water
use std::io;

struct Stats { max: i32, count: usize }

impl Stats {
    fn of(xs: []const i32) -> Option<Stats> {
        if (xs.len == 0) { return None; }
        let mut best = xs[0];
        for (x in xs) {
            if (x > best) { best = x; }
        }
        Some(Stats { max: best, count: xs.len })
    }

    fn describe(self) -> str {
        // `self : *const Stats`; read-only, chainable
        if (self.count == 0) { "empty" } else { "nonempty" }
    }
}

fn main() {
    let nums = [3, 17, -4, 9, 12];

    match Stats::of(nums[..]) {
        Some(s) => io::print("max = {}, {} ({})\n", s.max, s.count, s.describe()),
        None    => io::print("empty\n"),
    }
}
```

`Stats::of` is an associated function (no `self`); `describe` is a method (bare `self` = `*const Self`). `s.describe()` auto-takes `&s`. `{}` prints structurally; `s.max` is `17`.

## 19.3 Errors, chaining, and `?`

```water
use std::io;

enum ParseError {
    Empty,
    BadDigit(pos: usize, byte: u8),
    Overflow,
}

fn parse_i32(s: str) -> Result<i32, ParseError> {
    if (s.len == 0) { return Err(ParseError::Empty); }

    let mut i: usize = 0;
    let mut neg = false;
    if (s[0] == '-') { neg = true; i = 1; }
    if (i == s.len) { return Err(ParseError::Empty); }

    let mut acc: i32 = 0;
    while (i < s.len) {
        let c = s[i];
        if (c < '0' || c > '9') { return Err(ParseError::BadDigit(pos: i, byte: c)); }
        let d = (c - '0') as i32;                 // u8 as i32 is lossless (§4.2)
        if (acc > (2147483647 - d) / 10) { return Err(ParseError::Overflow); }
        acc = acc * 10 + d;                        // `*` and `+` trap; we checked
        i += 1;
    }
    if (neg) { acc = 0 - acc; }
    Ok(acc)
}

fn sum_all(args: []const str) -> Result<i32, ParseError> {
    let mut total = 0;
    for (a in args) {
        total += parse_i32(a)?;                    // one operator
    }
    Ok(total)
}

fn main(args: []str) {
    match sum_all(args[1..]) {
        Ok(n)                              => io::print("total = {}\n", n),
        Err(ParseError::Empty)             => io::eprint("error: empty argument\n"),
        Err(ParseError::BadDigit(pos, b))  => io::eprint("error: byte {:x} at {} is not a digit\n", b, pos),
        Err(ParseError::Overflow)          => io::eprint("error: number too large\n"),
    }
}
```

## 19.4 Memory, `defer`, and `Result<(), E>`

```water
use std::{io, mem};

fn read_all(path: str) -> Result<[]u8, io::Error> {
    let f = io::open(path)?;
    defer io::close(f);                     // runs on every path below

    let size = io::size_of_file(f)?;
    let buf = mem::alloc_n::<u8>(size);     // traps on OOM

    // A match used for effect is an expression in statement position, so it
    // takes a trailing `;` (grammar: `expr ";"`). Its value is `()` because
    // every arm here is `()`. `n` is bound above it.
    let mut n: usize = 0;
    match io::read(f, buf) {
        Ok(count) => { n = count; },
        Err(e)    => { mem::free_n(buf); return Err(e); },
    };
    Ok(buf[0..n])
}

fn save(path: str, data: []const u8) -> Result<(), io::Error> {
    let f = io::create(path)?;
    defer io::close(f);
    io::write(f, data)?;
    Ok(())                                  // "succeeded, nothing to return"
}

fn main() {
    match read_all("input.txt") {
        Ok(data) => {
            defer mem::free_n(data);        // caller owns it; one line says so
            io::print("{} bytes\n", data.len);
        },
        Err(e) => io::eprint("failed: {}\n", e),
    }
}
```

`Result<(), io::Error>` in `save` is the type C cannot spell: *can fail, returns nothing useful.* `()` earns its place here.

## 19.5 Firmware: a UART, no std

```water
// kind = "freestanding". No std, no allocator, no trap handler but ours.
use core::mem;

struct UartRegs {
    dr: u32,           // 0x00 data
    rsr: u32,          // 0x04
    _pad: [4]u32,      // 0x08..0x18   (prefix array type, §3.2)
    fr: u32,           // 0x18 flag
}

const UART0: usize = 0x1000_0000;

impl UartRegs {
    fn at(addr: usize) -> *UartRegs { mem::ptr_from_addr::<UartRegs>(addr) }

    fn putc(self: *Self, c: u8) {
        while ((mem::read_volatile(&self.fr) & 0x20) != 0) { }   // TX full
        mem::write_volatile(&self.dr, c as u32);
    }
}

// The link fails without this, by design (§11.2).
fn water_trap(info: *const TrapInfo) -> () {
    let u = UartRegs::at(UART0);
    for (b in info.message) { u.putc(b); }
    while (true) { }
}
```

`UartRegs` is not `packed` — MMIO registers are naturally aligned, `_pad: [4]u32` states the gap, and every struct is C-layout anyway; `&self.fr` is fine because the struct is aligned. `u.putc(b)` chains through the pointer receiver.
