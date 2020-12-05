# Advent of Code 2020

**Challenge: 1 language a day**

## Language ideas

  * [x] Lua (1)
  * [x] C (2)
  * [x] Rust compile-time (3)
  * [ ] Elixir (4)
  * [ ] Python
  * [ ] JS / TS
  * [ ] OCaml
  * [ ] Haskell
  * [ ] Kotlin
  * [ ] Bash
  * [ ] C#
  * [ ] Go
  * [ ] PHP
  * [ ] Swift
  * [ ] Dart
  * [ ] Ada
  * [ ] Groovy
  * [ ] Scratch
  * [ ] BF or other unreadable esolang
  * [ ] Rust `edition="cursed"`
  * [ ] Rust (24)

## Day 1 - Lua

```bash
lua5.3 main.lua
```

## Day 2 - C
_(was supposed to be ASM but I've never used ASM, and I gave up after spending way too much time on it)_

```bash
make run-one # Part 1
make run-two # Part 2
```

## Day 3 - Rust Compile-time-only

Okay, now this is fun. The concept is to implement today's algorithm using Rust's type-checker. Basically, the whole algorithm is implemented using declarative macros, types and traits. This essentially means that:

  1. The code looks really weird, because variables become `struct`s and `type`s, functions become `impl`s... Everything is mapped to its compile-time equivent.
  2. Nothing needs to be executed. Indeed, all the computing is done by Rust's compiler. Because the recursive function calls that sum 1s in order to create the final number are marked as `#[inline]`, Rust even optimises this sum and the integer is embeded as-is into the final binary file. This means that you don't even have to run the program, the result is inside its machine code.

All of this was mathematically guaranteed to be possible because [Rust's type system is](https://sdleffler.github.io/RustTypeSystemTuringComplete/) [Turing-complete](https://en.wikipedia.org/wiki/Turing_completeness). I took a little bit of inspiration by memory from [this reddit post](https://www.reddit.com/r/rust/comments/j9nnpv/proving_that_1_1_2_in_rust/).

Compiling the crate is **really** resource intensive, it takes about 45s on a good PC (AMD Ryzen 7 2700X Eight-Core @ 16x 3.7GHz). I had to resize `rustc`'s allowed stack size to a few GiB, as well as push the type-checker's recursion limit of the very limit before `rustc` SIGSEGVed from a stack overflow. I cannot guarantee that you will manage to compile the crate yourself, hence the inclusion of a compiled binary in the VCS in its usual location.

To compile:
```bash
# Optionally increase stack size (I have 16GiB so I can allocate 10GiB without problems)
set RUSTFLAGS "-C link-args=-Wl,-zstack-size=10000000"

# Compile the object archive
cargo build --release

# Compile the test object archive (hardcoded constant = 2 to test the compilation)
cargo build --features test-compile --release
```

_Compiling with `--release` is required for the inlining to work properly. I assume that it's linked to the fact that Rust integer sums are dynamically checked for overflows while in debug mode, and not in release. I could have wrapped my numbers in a `Wrapped<>` to solve this but I also had the choice not to. So I didn't._

After compiling, you don't have to launch the binary (actually you can't, its a `.a` file on purpose). You will just need to analyse it with `objdump` to find the compiled constant:

```bash
objdump -d target/release/libday_3.a  | grep -A2 get_trees
```

This spits out:

```
Disassembly of section .text.get_trees:

0000000000000000 <get_trees>:
   0:   b8 24 01 00 00          mov    $[[[REDACTED]]],%eax
   5:   c3                      retq
```

where `[[[REDACTED]]]` is the hex representation of the result. You could statically link it to another crate, or even a C program if you want, and call the function `get_trees()` to obtain the constant (`u64`).

Last thing: the input file is a little bit processed by `build.rs`. Although I'd consider too much preprocessing as cheating, I'm just adding spaces between characters and wrapping each line between square brackets so it's easier to parse with Rust's procedural macros. This is because detecting line breaks is impossible and I don't want to hardcode the expected line length, and because the dot (`.`) tokens inside the input file – if not separated by spaces – will merge into a range operator/expression (`..`) which makes the processing a bit harder and doesn't really add value to the challenge.

## Day 4 - Elixir <kbd>First time</kbd>

```bash
# (If you don't want to lose 1h installing Elixir with 3 different broken
# methods:
nix-shell -p elixir
# )

# Else/Then:
elixir main.exs
```
