use axum::{
    Json,
    extract::{Query, State},
};
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use serde::Deserialize;
use tracing::instrument;

use crate::{
    database::{database_environment, prisma},
    errors::Error,
    state::ServerState,
};

#[derive(Deserialize)]
pub struct GetModerationsQuery {
    pub status: Option<prisma::ExamEnvironmentExamModerationStatus>,
}

#[instrument(skip_all, err(Debug))]
pub async fn get_moderations(
    exam_creator_user: prisma::ExamCreatorUser,
    State(server_state): State<ServerState>,
    Query(params): Query<GetModerationsQuery>,
) -> Result<Json<Vec<prisma::ExamEnvironmentExamModeration>>, Error> {
    let database = database_environment(&server_state, &exam_creator_user);
    let status = params.status;
    let filter = if let Some(status) = status {
        doc! { "status": bson::serialize_to_bson(&status)? }
    } else {
        doc! {}
    };

    let exam_moderations = database
        .exam_environment_exam_moderation
        .find(filter)
        .await?
        .try_collect()
        .await?;

    Ok(Json(exam_moderations))
}
