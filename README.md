# bitvec + wasm build bug?

`cargo build` works fine.

`cargo build --target wasm32-unknown-unknown` fails with:

```
   Compiling wasm-bug v0.1.0 (/home/ari/src/wasm-bug)
error[E0277]: the trait bound `[u64; 4]: BitViewSized` is not satisfied
  --> src/main.rs:4:38
   |
4  |     let a = BitArray::<_, Lsb0>::new([0u64; 4]);
   |             ------------------------ ^^^^^^^^^ the trait `BitViewSized` is not implemented for `[u64; 4]`
   |             |
   |             required by a bound introduced by this call
   |
   = help: the trait `BitViewSized` is implemented for `[T; N]`
note: required by a bound in `BitArray::<A, O>::new`
  --> /home/ari/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitvec-1.0.1/src/array.rs:38:5
   |
38 |     A: BitViewSized,
   |        ^^^^^^^^^^^^ required by this bound in `BitArray::<A, O>::new`
...
59 |     pub fn new(data: A) -> Self {
   |            --- required by a bound in this associated function

error[E0277]: the trait bound `[u64; 4]: BitViewSized` is not satisfied
  --> src/main.rs:4:13
   |
4  |     let a = BitArray::<_, Lsb0>::new([0u64; 4]);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `BitViewSized` is not implemented for `[u64; 4]`
   |
   = help: the trait `BitViewSized` is implemented for `[T; N]`
note: required by a bound in `BitArray`
  --> /home/ari/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitvec-1.0.1/src/array.rs:27:5
   |
25 | pub struct BitArray<A = [usize; 1], O = Lsb0>
   |            -------- required by a bound in this struct
26 | where
27 |     A: BitViewSized,
   |        ^^^^^^^^^^^^ required by this bound in `BitArray`

error[E0277]: the trait bound `[u64; 4]: BitViewSized` is not satisfied
  --> src/main.rs:5:5
   |
5  |     dbg!(a);
   |     ^^^^^^^ the trait `BitViewSized` is not implemented for `[u64; 4]`
   |
   = help: the trait `BitViewSized` is implemented for `[T; N]`
note: required by a bound in `BitArray`
  --> /home/ari/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitvec-1.0.1/src/array.rs:27:5
   |
25 | pub struct BitArray<A = [usize; 1], O = Lsb0>
   |            -------- required by a bound in this struct
26 | where
27 |     A: BitViewSized,
   |        ^^^^^^^^^^^^ required by this bound in `BitArray`
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `dbg` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `wasm-bug` (bin "wasm-bug") due to 3 previous errors
```
