use axum::{
    Json,
    extract::{Path, State},
};
use http::StatusCode;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::{
    database::{ExamCreatorUser, prisma},
    errors::Error,
    state::ServerState,
};

#[instrument(skip_all, err(Debug))]
pub async fn discard_exam_state_by_id(
    _: ExamCreatorUser,
    State(state): State<ServerState>,
    Path(exam_id): Path<ObjectId>,
) -> Result<Json<prisma::EnvExam>, Error> {
    let original_exam = state
        .database
        .temp_env_exam
        .find_one(doc! {
            "_id": &exam_id
        })
        .await?
        .ok_or(Error::Server(
            StatusCode::BAD_REQUEST,
            format!("No exam {exam_id} found"),
        ))?;

    let client_sync = &mut state.client_sync.lock().unwrap();
    if let Some(exam) = client_sync.exams.iter_mut().find(|e| e.id == exam_id) {
        *exam = original_exam.clone();
    } else {
        info!("No exam in client sync state: {}", exam_id)
    }

    Ok(Json(original_exam))
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct MaintenanceMode {
    pub enabled: bool,
}

#[instrument(skip_all, err(Debug))]
pub async fn post_maintenance(
    _: ExamCreatorUser,
    State(state): State<ServerState>,
    Json(maintenance_mode): Json<MaintenanceMode>,
) -> Result<Json<MaintenanceMode>, Error> {
    let mut mode = state.maintenance_mode.write().unwrap();
    *mode = maintenance_mode.clone();
    Ok(Json(maintenance_mode))
}
#[instrument(skip_all, err(Debug))]
pub async fn get_maintenance(
    _: ExamCreatorUser,
    State(state): State<ServerState>,
) -> Result<Json<MaintenanceMode>, Error> {
    let maintenance_mode = state
        .maintenance_mode
        .read()
        .expect("maintenance mode to be readable");
    Ok(Json(*maintenance_mode))
}
