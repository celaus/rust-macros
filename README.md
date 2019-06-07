# A Collection Of Simple Macro Examples

This is part of a blog post over at [https://blog.x5ff.xyz](https://blog.x5ff.xyz). Run it like that (the warnings are part of the fun):

~~~sh
$ cargo test

   Compiling rust-macros v0.1.0 (rust-macros)
warning: unused macro definition
  --> src/lib.rs:7:1
   |
7  | / macro_rules! calc  {
8  | |   (two) => { 1 + 1 };
9  | |   (three) => { 1 + 2 }
10 | | }
   | |_^
   |
   = note: #[warn(unused_macros)] on by default

warning: unused macro definition
  --> src/lib.rs:12:1
   |
12 | / macro_rules! repeat_n_times {
13 | |     ($n:expr, $text:expr) => {(0..$n).map(|_| format!("{}", $text)).collect::<Vec<String>>()}
14 | | }
   | |_^

warning: unused macro definition
  --> src/lib.rs:21:1
   |
21 | / macro_rules! handler {
22 | |     ($i: ident, $body: block) => {
23 | |         fn $i () -> Response $body
24 | |     }
25 | | }
   | |_^

warning: unused import: `std::collections::BTreeSet`
  --> src/lib.rs:27:5
   |
27 | use std::collections::BTreeSet;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_imports)] on by default

warning: unused macro definition
  --> src/lib.rs:29:1
   |
29 | / macro_rules! set {
30 | |  ( $( $item:expr ),* ) => {
31 | |         {
32 | |             let mut s = BTreeSet::new();
...  |
38 | |     };
39 | | }
   | |_^

    Finished dev [unoptimized + debuginfo] target(s) in 1.12s
     Running target/debug/deps/rust_macros-f64009a11dc6e1e2

running 6 tests
test tests::test_calc ... ok
test tests::test_req_handler ... ok
test tests::test_repeat ... ok
test tests::test_set ... ok
test tests::test_two ... ok
test tests::test_kv ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests rust-macros

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
~~~

# License

MIT