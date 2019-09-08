#[macro_use]
extern crate criterion;
extern crate empty_spaces;

mod tests;

criterion_group!(benches, tests::general);
criterion_main!(benches);
