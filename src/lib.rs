#![feature(test)]
extern crate test;

mod base;
mod persistent;
pub use base::Database;
pub use persistent::DatabasePersistent;

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_local(b: &mut Bencher) {
        let mut db = Database::new();

        b.iter(|| {
            for i in 1..100_000 {
                black_box(db.store(i, i));
            }
        });
    }

    #[bench]
    fn bench_serde(b: &mut Bencher) {
        let mut db = DatabasePersistent::new();

        b.iter(|| {
            for i in 1..100_000 {
                black_box(db.store(i, i).unwrap());
            }
        });
    }
}
