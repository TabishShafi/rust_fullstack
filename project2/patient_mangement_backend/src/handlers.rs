use actix_web::{web, HttpResponse};
use mongodb::bson::doc;
use futures_util::stream::TryStreamExt;
use crate::{models::Patient, db::get_patient_collection};

pub async fn create_patient(new_patient: web::Json<Patient>) -> HttpResponse {
    let collection = get_patient_collection().await;
    let patient = Patient {
        id: None,
        ..new_patient.into_inner()
    };

    let insert_result = collection.insert_one(patient).await;

    match insert_result {
        Ok(inserted) => HttpResponse::Created().json(inserted.inserted_id),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_patients() -> HttpResponse {
    let collection = get_patient_collection().await;
    let cursor = collection.find(doc! {}).await;

    match cursor {
        Ok(patients) => {
            let results: Vec<_> = patients.try_collect().await.unwrap_or_else(|_| vec![]);
            HttpResponse::Ok().json(results)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
