use myairtable_tests::*;

#[tokio::main]
async fn main() {
    println!("Hello from playground!");

    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    let airtable = Airtable::new(&api_key, &base_id);

    let record_id = "recUCWnd7r6zTKS0z".to_string();
    let mut record = airtable.primary_orm.get_one(&record_id).await.unwrap();
    println!("Fetched record: {:?} - {:?}", record.id, record.primary_key);

    record.single_line_text = Some("hello".to_string());
    record.save().await.unwrap();
    println!(
        "Updated record: {:?} - {:?}",
        record.id, record.single_line_text
    );
}
