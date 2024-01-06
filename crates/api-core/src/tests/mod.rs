use crate::Category;

fn create_category() -> Result<Category, serde_json::Error> {
    let data = r#"
        {
            "id": "category:something",
            "name": "Something",
            "sub_categories": [],
            "image_url": null,
            "is_root": false
        }"#;

    serde_json::from_str::<Category>(data)
}

#[test]
pub fn serialise() {
    let category = create_category();
    dbg!(&category);

    assert!(category.is_ok());
}

#[test]
pub fn deserialise_list() {
    let category = create_category();
    let category_2 = create_category();

    assert!(&category.is_ok());
    assert!(&category_2.is_ok());

    let category = category.unwrap();
    let category_2 = category_2.unwrap();

    let categories = vec![category, category_2];

    let str_val = serde_json::to_string(&categories);

    dbg!(&str_val);

    assert!(str_val.is_ok());
}
