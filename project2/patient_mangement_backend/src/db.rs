use mongodb::{Client, Collection};
use crate::models::Patient;
use std::env;

pub async fn get_patient_collection() -> Collection<Patient> {
    dotenv::dotenv().ok();  // Load environment variables
    let client_uri = env::var("DATABASE_URI").expect("DATABASE_URI must be set");
    let client = Client::with_uri_str(&client_uri).await.expect("Failed to connect to MongoDB");
    let database = client.database(&env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"));
    database.collection::<Patient>("patients")
}