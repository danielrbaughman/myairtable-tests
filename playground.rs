use myairtable_tests::{field_types::PrimaryView, *};

#[tokio::main]
async fn main() {
    println!("Hello from playground!");

    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    let airtable = Airtable::new(&api_key, &base_id);

    let record_id = "recUCWnd7r6zTKS0z".to_string();
    let query = AirtableQuery::new().view(PrimaryView::FilterByView);
    let (records, _offset) = airtable.primary_orm.get_many(&query).await.unwrap();
    println!(
        "Fetched record: {:?} - {:?}",
        records[0].id, records[0].primary_key
    );

    // records[0].single_line_text = Some("hello".to_string());
    // records[0].save().await.unwrap();
    // println!(
    //     "Updated record: {:?} - {:?}",
    //     records[0].id, records[0].single_line_text
    // );
}
