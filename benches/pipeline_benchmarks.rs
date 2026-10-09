use criterion::{Criterion, criterion_group, criterion_main};
use open_social_data_core::pipeline::{RawRecord, RecordBatchBuilder};
use std::hint::black_box;

fn bench_record_batch_builder(c: &mut Criterion) {
    c.bench_function("record_batch_builder_push_1000", |b| {
        b.iter(|| {
            let mut builder = RecordBatchBuilder::new();
            for i in 0..1000 {
                let mut record = RawRecord::new();
                for j in 0..50 {
                    record = record.with(format!("col_{}", j), format!("val_{}_{}", i, j));
                }
                builder.push(black_box(record));
            }
        })
    });
}

criterion_group!(benches, bench_record_batch_builder);
criterion_main!(benches);
