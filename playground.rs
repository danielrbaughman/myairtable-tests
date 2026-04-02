use myairtable_tests::*;

#[tokio::main]
async fn main() {
    println!("Hello from playground!");

    dotenvy::from_filename(".env").ok();
    let api_key = std::env::var("AIRTABLE_API_KEY").expect("AIRTABLE_API_KEY must be set");
    let base_id = std::env::var("AIRTABLE_BASE_ID").expect("AIRTABLE_BASE_ID must be set");
    let airtable = Airtable::new(&api_key, &base_id);

    // let record_id = "recUCWnd7r6zTKS0z".to_string();
    let query = AirtableQuery::new().formula(formula::AND(&[&PrimaryModel::F
        .single_line_text
        .contains("single", true, false)]));
    let (records, _offset) = airtable.primary_orm.get_many(&query).await.unwrap();
    airtable.url();
    records[0].record_url("");
    airtable.primary_orm.url();
    println!(
        "Fetched record: {:?} - {:?}",
        records[0].id, records[0].primary_key
    );
    PrimaryModel::O.single_select;

    // records[0].single_line_text = Some("hello".to_string());
    // records[0].save().await.unwrap();
    // println!(
    //     "Updated record: {:?} - {:?}",
    //     records[0].id, records[0].single_line_text
    // );
}
