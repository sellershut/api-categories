use anyhow::Result;
use api_database::Client;
use std::str::FromStr;

use api_core::{api::MutateCategories, Category, Id};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use fake::{faker::lorem::en::Words, Fake};

async fn create_client(with_ns: Option<&str>) -> Result<Client> {
    let db_host = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL");
    let db_host = db_host.replace("http://", "");

    let username = std::env::var("TEST_DATABASE_USERNAME").expect("TEST_DATABASE_USERNAME");
    let password = std::env::var("TEST_DATABASE_PASSWORD").expect("TEST_DATABASE_PASSWORD");
    let db_namespace = std::env::var("TEST_DATABASE_NAMESPACE").expect("TEST_DATABASE_NAMESPACE");
    let db_name = std::env::var("TEST_DATABASE_NAME").expect("TEST_DATABASE_NAME");

    let client = Client::try_new(
        &db_host,
        &username,
        &password,
        with_ns.unwrap_or(&db_namespace),
        &db_name,
        None,
    )
    .await?;

    Ok(client)
}

fn bench(c: &mut Criterion) {
    dotenvy::dotenv().ok();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(create_client(Some("benchmarks"))).unwrap();

    let size = 100;

    let words: Vec<String> = Words(1..5).fake();
    let words = words.join(" ");

    let sub_categories: Vec<String> = Words(0..4).fake();
    let sub_categories: Vec<_> = sub_categories
        .iter()
        .map(|word| Id::from_str(&format!("category:{word}")).unwrap())
        .collect();

    let category = Category {
        id: api_core::Id::default(),
        name: words,
        sub_categories: Some(sub_categories),
        image_url: None,
        is_root: false,
    };

    c.bench_with_input(BenchmarkId::new("category insert", size), &size, |b, &s| {
        b.to_async(&rt)
            .iter(|| black_box(client.create_category(&category)));
    });

    // should probably clean everything after inserting
}

criterion_group!(benches, bench);
criterion_main!(benches);
