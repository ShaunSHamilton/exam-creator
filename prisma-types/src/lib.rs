use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

prisma_rust_schema::import_types!(
    schema_paths = [
        "https://raw.githubusercontent.com/freeCodeCamp/freeCodeCamp/main/api/prisma/schema.prisma",
        "https://raw.githubusercontent.com/freeCodeCamp/freeCodeCamp/main/api/prisma/exam-environment.prisma",
        "https://raw.githubusercontent.com/freeCodeCamp/freeCodeCamp/main/api/prisma/exam-creator.prisma",
    ],
    derive = [Clone, Debug, Serialize, Deserialize, PartialEq],
    include = [
        "ExamEnvironmentExam",
        "ExamCreatorExam",
        "ExamCreatorUser",
        "ExamCreatorUserSettings",
        "ExamCreatorDatabaseEnvironment",
        "ExamCreatorSession",
        "ExamEnvironmentQuestionSet",
        "ExamEnvironmentMultipleChoiceQuestion",
        "ExamEnvironmentAudio",
        "ExamEnvironmentQuestionType",
        "ExamEnvironmentConfig",
        "ExamEnvironmentQuestionSetConfig",
        "ExamEnvironmentTagConfig",
        "ExamEnvironmentAnswer",
        "ExamEnvironmentExamAttempt",
        "ExamEnvironmentQuestionSetAttempt",
        "ExamEnvironmentMultipleChoiceQuestionAttempt",
        "ExamEnvironmentGeneratedExam",
        "ExamEnvironmentGeneratedQuestionSet",
        "ExamEnvironmentGeneratedMultipleChoiceQuestion",
        "ExamEnvironmentExamModeration",
        "ExamEnvironmentExamModerationStatus",
        "ExamEnvironmentChallenge",
    ],
    patch = [
        struct ExamEnvironmentConfig {
            #[serde(rename = "totalTimeInMS")]
            pub total_time_in_m_s: f64,
            #[serde(rename = "retakeTimeInMS")]
            pub retake_time_in_m_s: f64
        },
        struct ExamEnvironmentExamAttempt {
            #[serde(rename = "startTimeInMS")]
            pub start_time_in_m_s: f64
        },
        struct ExamEnvironmentMultipleChoiceQuestionAttempt {
            #[serde(rename = "submissionTimeInMS")]
            pub submission_time_in_m_s: f64
        },
    ]
);

impl Default for ExamCreatorExam {
    fn default() -> Self {
        ExamCreatorExam {
            id: ObjectId::new(),
            question_sets: vec![],
            config: Default::default(),
            prerequisites: vec![],
            deprecated: false,
            version: 1,
        }
    }
}

impl Default for ExamEnvironmentConfig {
    fn default() -> Self {
        ExamEnvironmentConfig {
            name: String::new(),
            note: String::new(),
            tags: vec![],
            total_time_in_m_s: 2.0 * 60.0 * 60.0 * 1000.0,
            total_time_in_s: Some(2 * 60 * 60),
            question_sets: vec![],
            retake_time_in_m_s: 24.0 * 60.0 * 60.0 * 1000.0,
            retake_time_in_s: Some(24 * 60 * 60),
            passing_percent: 80.0,
        }
    }
}

impl Default for ExamCreatorUserSettings {
    fn default() -> Self {
        Self {
            database_environment: ExamCreatorDatabaseEnvironment::Production,
        }
    }
}

impl ExamCreatorExam {
    /// Helper method to construct from BSON document with projections
    pub fn from_bson_document(
        value: bson::Document,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let id = value
            .get_object_id("_id")
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let question_sets = bson::deserialize_from_bson(
            value
                .get("questionSets")
                .unwrap_or(&bson::Bson::Array(vec![]))
                .clone(),
        )
        .unwrap_or_default();
        let config = bson::deserialize_from_document(
            value
                .get_document("config")
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
                .clone(),
        )
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let prerequisites = bson::deserialize_from_bson(
            value
                .get("prerequisites")
                .unwrap_or(&bson::Bson::Array(vec![]))
                .clone(),
        )
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let deprecated = value
            .get_bool("deprecated")
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let version = match value.get("version") {
            Some(bson::Bson::Int32(v)) => *v as i64,
            Some(bson::Bson::Int64(v)) => *v,
            _ => 1,
        };

        Ok(ExamCreatorExam {
            id,
            question_sets,
            config,
            prerequisites,
            deprecated,
            version,
        })
    }
}
