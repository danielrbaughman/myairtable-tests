use myairtable_tests::*;

#[test]
fn creates_client() {
    let _client = AirtableClient::new("test_key", "appTEST123");
}
