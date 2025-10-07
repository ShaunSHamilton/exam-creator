use axum::{
    Json,
    extract::{Path, State},
};
use http::StatusCode;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use tracing::{info, instrument};

use crate::{
    database::{self, prisma},
    errors::Error,
    generation::{self, ExamInput},
    state::ServerState,
};

/// Generate an exam based on the exam configuration
#[instrument(skip_all, err(Debug))]
pub async fn post_generate_exam(
    auth_user: crate::extractor::AuthUser,
    State(state): State<ServerState>,
    Path(exam_id): Path<ObjectId>,
) -> Result<Json<prisma::ExamEnvironmentGeneratedExam>, Error> {
    info!("Generating exam for exam_id: {}", exam_id);

    let database = database::database_environment(&state, &auth_user);

    // Fetch the exam from the appropriate database
    let exam_creator_exam = database
        .exam_creator_exam
        .find_one(doc! { "_id": exam_id })
        .await?
        .ok_or(Error::Server(
            StatusCode::BAD_REQUEST,
            format!("exam non-existent: {exam_id}"),
        ))?;

    // Convert to ExamInput for generation
    let exam_input = ExamInput {
        id: exam_creator_exam.id,
        question_sets: exam_creator_exam.question_sets,
        config: exam_creator_exam.config,
    };

    // Generate the exam
    let generated_exam = generation::generate_exam(exam_input)?;

    // Store the generated exam in the database
    database.generated_exam.insert_one(&generated_exam).await?;

    info!("Successfully generated exam: {}", generated_exam.id);

    Ok(Json(generated_exam))
}
