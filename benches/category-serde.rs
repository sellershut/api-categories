use std::str::FromStr;

use api_core::Category;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fake::{faker::lorem::en::Words, Fake};

fn bench(c: &mut Criterion) {
    let count = 24;
    let mut categories = Vec::with_capacity(count);

    for _ in 0..count {
        let words: Vec<String> = Words(1..5).fake();
        let words = words.join(" ");

        let id = surrealdb::sql::Thing::from_str("category:abc").unwrap();

        let category = Category {
            id: api_core::Id::Thing(id),
            name: words,
            sub_categories: Words(0..4).fake(),
            image_url: None,
            is_root: false,
        };

        categories.push(category);
    }

    c.bench_function(&format!("serialise {count}"), |b| {
        b.iter(|| black_box(serde_json::to_string(&categories)))
    });

    let cat_str = serde_json::to_string(&categories).unwrap();

    c.bench_function(&format!("deserialise {count}"), |b| {
        b.iter(|| black_box(serde_json::from_str::<Vec<Category>>(&cat_str)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
