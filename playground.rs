use myairtable_tests::*;

#[tokio::main]
async fn main() {
    println!("Hello from playground!");

    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    let airtable = Airtable::new(&api_key, &base_id);

    let record_id = "recNk6Lmrr5y3Fx81".to_string();
    let record = airtable.primary.get_one(&record_id, true).await.unwrap();
    let name = record.fields.get(PrimaryFields::PRIMARY_KEY_ID).unwrap();
    println!("Fetched record: {} - {}", record.id, name);
}
