use mongodb::{Client, Collection, bson::doc, options::ClientOptions};
use prisma_types::*;
use proptest::{prelude::*, strategy::ValueTree};
use std::error::Error;
use tracing::info;

mod strategies;

use mongodb::bson::oid::ObjectId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    dotenvy::dotenv().ok();

    info!("Seeding all collections with 3 items each");

    let env = std::env::var("MONGODB_ENV").unwrap_or_else(|_| "staging".to_string());
    info!("Using {} database environment", env);

    seed_all(&env).await?;

    info!("✅ Seeding complete!");
    Ok(())
}

async fn seed_all(env: &str) -> Result<(), Box<dyn Error>> {
    const COUNT: usize = 3;

    // Seed independent collections first
    seed_exams(COUNT, env).await?;
    seed_users(COUNT, env).await?;

    // Seed collections with dependencies
    seed_generated_exams(COUNT, env).await?;
    seed_attempts(COUNT, env).await?;
    seed_challenges(COUNT, env).await?;

    Ok(())
}

async fn get_client(env: &str) -> Result<Client, Box<dyn Error>> {
    let uri_key = match env {
        "production" => "MONGODB_URI_PRODUCTION",
        _ => "MONGODB_URI_STAGING",
    };

    let uri =
        std::env::var(uri_key).map_err(|_| format!("{} environment variable not set", uri_key))?;

    let client_options = ClientOptions::parse(&uri).await?;
    Ok(Client::with_options(client_options)?)
}

/// Fetch existing exam IDs from the database
async fn get_existing_exam_ids(client: &Client) -> Result<Vec<ObjectId>, Box<dyn Error>> {
    let db = client.database("freecodecamp");
    let collection: Collection<ExamCreatorExam> = db.collection("ExamCreatorExam");

    let mut cursor = collection.find(doc! {}).await?;
    let mut ids = Vec::new();

    use futures_util::StreamExt;
    while let Some(result) = cursor.next().await {
        let exam = result?;
        ids.push(exam.id);
    }

    Ok(ids)
}

/// Fetch existing user IDs from the database
async fn get_existing_user_ids(client: &Client) -> Result<Vec<ObjectId>, Box<dyn Error>> {
    let db = client.database("freecodecamp");
    let collection: Collection<ExamCreatorUser> = db.collection("ExamCreatorUser");

    let mut cursor = collection.find(doc! {}).await?;
    let mut ids = Vec::new();

    use futures_util::StreamExt;
    while let Some(result) = cursor.next().await {
        let user = result?;
        ids.push(user.id);
    }

    Ok(ids)
}

/// Fetch existing generated exam IDs from the database
async fn get_existing_generated_exam_ids(client: &Client) -> Result<Vec<ObjectId>, Box<dyn Error>> {
    let db = client.database("freecodecamp");
    let collection: Collection<ExamEnvironmentGeneratedExam> =
        db.collection("ExamEnvironmentGeneratedExam");

    let mut cursor = collection.find(doc! {}).await?;
    let mut ids = Vec::new();

    use futures_util::StreamExt;
    while let Some(result) = cursor.next().await {
        let exam = result?;
        ids.push(exam.id);
    }

    Ok(ids)
}

async fn seed_exams(count: usize, env: &str) -> Result<(), Box<dyn Error>> {
    info!("📝 Seeding {} exams...", count);

    let client = get_client(env).await?;
    let db = client.database("freecodecamp");
    let collection: Collection<ExamCreatorExam> = db.collection("ExamCreatorExam");

    let mut runner = proptest::test_runner::TestRunner::default();

    for i in 0..count {
        let exam = strategies::exam_strategy()
            .new_tree(&mut runner)
            .map_err(|e| format!("Failed to generate exam tree: {}", e))?
            .current();

        collection.insert_one(exam).await?;
        info!("  ✓ Inserted exam {}/{}", i + 1, count);
    }

    Ok(())
}

async fn seed_users(count: usize, env: &str) -> Result<(), Box<dyn Error>> {
    info!("👤 Seeding {} users...", count);

    let client = get_client(env).await?;
    let db = client.database("freecodecamp");
    let collection: Collection<ExamCreatorUser> = db.collection("ExamCreatorUser");

    let mut runner = proptest::test_runner::TestRunner::default();

    for i in 0..count {
        let user = strategies::user_strategy()
            .new_tree(&mut runner)
            .map_err(|e| format!("Failed to generate user tree: {}", e))?
            .current();

        collection.insert_one(user).await?;
        info!("  ✓ Inserted user {}/{}", i + 1, count);
    }

    Ok(())
}

async fn seed_attempts(count: usize, env: &str) -> Result<(), Box<dyn Error>> {
    info!("📋 Seeding {} exam attempts...", count);

    let client = get_client(env).await?;
    let db = client.database("freecodecamp");
    let collection: Collection<ExamEnvironmentExamAttempt> =
        db.collection("ExamEnvironmentExamAttempt");

    let exam_ids = get_existing_exam_ids(&client).await?;
    let user_ids = get_existing_user_ids(&client).await?;
    let generated_exam_ids = get_existing_generated_exam_ids(&client).await?;

    if exam_ids.is_empty() || user_ids.is_empty() || generated_exam_ids.is_empty() {
        return Err("Missing required data. This should not happen in seed_all flow.".into());
    }

    let mut runner = proptest::test_runner::TestRunner::default();

    for i in 0..count {
        let attempt =
            strategies::exam_attempt_with_ids_strategy(&exam_ids, &user_ids, &generated_exam_ids)
                .new_tree(&mut runner)
                .map_err(|e| format!("Failed to generate attempt tree: {}", e))?
                .current();

        collection.insert_one(attempt).await?;
        info!("  ✓ Inserted exam attempt {}/{}", i + 1, count);
    }

    Ok(())
}

async fn seed_generated_exams(count: usize, env: &str) -> Result<(), Box<dyn Error>> {
    info!("🎲 Seeding {} generated exams...", count);

    let client = get_client(env).await?;
    let db = client.database("freecodecamp");
    let collection: Collection<ExamEnvironmentGeneratedExam> =
        db.collection("ExamEnvironmentGeneratedExam");

    let exam_ids = get_existing_exam_ids(&client).await?;

    if exam_ids.is_empty() {
        return Err("No exams found. This should not happen in seed_all flow.".into());
    }

    let mut runner = proptest::test_runner::TestRunner::default();

    for i in 0..count {
        let generated_exam = strategies::generated_exam_with_exam_id_strategy(&exam_ids)
            .new_tree(&mut runner)
            .map_err(|e| format!("Failed to generate generated exam tree: {}", e))?
            .current();

        collection.insert_one(generated_exam).await?;
        info!("  ✓ Inserted generated exam {}/{}", i + 1, count);
    }

    Ok(())
}

async fn seed_challenges(count: usize, env: &str) -> Result<(), Box<dyn Error>> {
    info!("🎯 Seeding {} challenges...", count);

    let client = get_client(env).await?;
    let db = client.database("freecodecamp");
    let collection: Collection<ExamEnvironmentChallenge> =
        db.collection("ExamEnvironmentChallenge");

    let exam_ids = get_existing_exam_ids(&client).await?;

    if exam_ids.is_empty() {
        return Err("No exams found. This should not happen in seed_all flow.".into());
    }

    let mut runner = proptest::test_runner::TestRunner::default();

    for i in 0..count {
        let challenge = strategies::challenge_with_exam_id_strategy(&exam_ids)
            .new_tree(&mut runner)
            .map_err(|e| format!("Failed to generate challenge tree: {}", e))?
            .current();

        collection.insert_one(challenge).await?;
        info!("  ✓ Inserted challenge {}/{}", i + 1, count);
    }

    Ok(())
}
