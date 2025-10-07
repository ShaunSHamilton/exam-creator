use mongodb::bson::oid::ObjectId;
use prisma_types::*;
use proptest::prelude::*;

// Helper to generate ObjectId
pub fn object_id_strategy() -> impl Strategy<Value = ObjectId> {
    any::<[u8; 12]>().prop_map(ObjectId::from_bytes)
}

// Helper to generate reasonable strings
pub fn text_strategy() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9 ]{5,50}"
}

pub fn short_text_strategy() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9 ]{3,20}"
}

pub fn long_text_strategy() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9 .,!?\\n]{20,500}"
}

pub fn email_strategy() -> impl Strategy<Value = String> {
    "[a-z]{3,10}@[a-z]{3,10}\\.(com|org|net)"
}

pub fn url_strategy() -> impl Strategy<Value = String> {
    "https://[a-z]{3,10}\\.(com|org|net)/[a-z]{3,10}"
}

// ExamEnvironmentAnswer strategy
pub fn answer_strategy() -> impl Strategy<Value = ExamEnvironmentAnswer> {
    (object_id_strategy(), text_strategy(), prop::bool::ANY).prop_map(|(id, text, is_correct)| {
        ExamEnvironmentAnswer {
            id,
            text,
            is_correct,
        }
    })
}

// ExamEnvironmentAudio strategy
pub fn audio_strategy() -> impl Strategy<Value = ExamEnvironmentAudio> {
    (prop::option::of(text_strategy()), url_strategy()).prop_map(|(captions, url)| {
        ExamEnvironmentAudio { captions, url }
    })
}

// ExamEnvironmentMultipleChoiceQuestion strategy
pub fn multiple_choice_question_strategy(
) -> impl Strategy<Value = ExamEnvironmentMultipleChoiceQuestion> {
    (
        object_id_strategy(),
        long_text_strategy(),
        prop::collection::vec(short_text_strategy(), 0..5),
        prop::option::of(audio_strategy()),
        prop::collection::vec(answer_strategy(), 2..6),
        prop::bool::ANY,
    )
        .prop_map(|(id, text, tags, audio, answers, deprecated)| {
            ExamEnvironmentMultipleChoiceQuestion {
                id,
                text,
                tags,
                audio,
                answers,
                deprecated,
            }
        })
}

// ExamEnvironmentQuestionSet strategy
pub fn question_set_strategy() -> impl Strategy<Value = ExamEnvironmentQuestionSet> {
    (
        object_id_strategy(),
        prop::sample::select(vec![
            ExamEnvironmentQuestionType::MultipleChoice,
            ExamEnvironmentQuestionType::Dialogue,
        ]),
        prop::option::of(text_strategy()),
        prop::collection::vec(multiple_choice_question_strategy(), 1..10),
    )
        .prop_map(|(id, _type, context, questions)| ExamEnvironmentQuestionSet {
            id,
            _type,
            context,
            questions,
        })
}

// ExamEnvironmentTagConfig strategy
pub fn tag_config_strategy() -> impl Strategy<Value = ExamEnvironmentTagConfig> {
    (
        prop::collection::vec(short_text_strategy(), 1..3),
        1i64..10,
    )
        .prop_map(|(group, number_of_questions)| ExamEnvironmentTagConfig {
            group,
            number_of_questions,
        })
}

// ExamEnvironmentQuestionSetConfig strategy
pub fn question_set_config_strategy() -> impl Strategy<Value = ExamEnvironmentQuestionSetConfig> {
    (
        prop::sample::select(vec![
            ExamEnvironmentQuestionType::MultipleChoice,
            ExamEnvironmentQuestionType::Dialogue,
        ]),
        1i64..5,
        1i64..20,
        1i64..4,
        1i64..4,
    )
        .prop_map(
            |(
                _type,
                number_of_set,
                number_of_questions,
                number_of_correct_answers,
                number_of_incorrect_answers,
            )| {
                ExamEnvironmentQuestionSetConfig {
                    _type,
                    number_of_set,
                    number_of_questions,
                    number_of_correct_answers,
                    number_of_incorrect_answers,
                }
            },
        )
}

// ExamEnvironmentConfig strategy
pub fn config_strategy() -> impl Strategy<Value = ExamEnvironmentConfig> {
    (
        text_strategy(),
        text_strategy(),
        prop::collection::vec(tag_config_strategy(), 0..5),
        1i64..10000,
        prop::option::of(1i64..10000),
        prop::collection::vec(question_set_config_strategy(), 1..3),
        1i64..100000,
        prop::option::of(1i64..100000),
        50.0f64..100.0,
    )
        .prop_map(
            |(
                name,
                note,
                tags,
                total_time_in_m_s,
                total_time_in_s,
                question_sets,
                retake_time_in_m_s,
                retake_time_in_s,
                passing_percent,
            )| {
                ExamEnvironmentConfig {
                    name,
                    note,
                    tags,
                    total_time_in_m_s: total_time_in_m_s as f64,
                    total_time_in_s,
                    question_sets,
                    retake_time_in_m_s: retake_time_in_m_s as f64,
                    retake_time_in_s,
                    passing_percent,
                }
            },
        )
}

// ExamCreatorExam strategy
pub fn exam_strategy() -> impl Strategy<Value = ExamCreatorExam> {
    (
        object_id_strategy(),
        prop::collection::vec(question_set_strategy(), 1..3),
        config_strategy(),
        prop::collection::vec(object_id_strategy(), 0..3),
        prop::bool::ANY,
        1i64..10,
    )
        .prop_map(
            |(id, question_sets, config, prerequisites, deprecated, version)| {
                ExamCreatorExam {
                    id,
                    question_sets,
                    config,
                    prerequisites,
                    deprecated,
                    version,
                }
            },
        )
}

// ExamCreatorUser strategy
pub fn user_strategy() -> impl Strategy<Value = ExamCreatorUser> {
    (
        object_id_strategy(),
        text_strategy(),
        email_strategy(),
        prop::option::of(url_strategy()),
        prop::option::of(user_settings_strategy()),
        prop::option::of(any::<i64>()),
        1i64..10,
    )
        .prop_map(|(id, name, email, picture, settings, github_id, version)| ExamCreatorUser {
            id,
            name,
            email,
            picture,
            settings,
            github_id,
            version,
        })
}

// ExamCreatorUserSettings strategy
pub fn user_settings_strategy() -> impl Strategy<Value = ExamCreatorUserSettings> {
    prop::sample::select(vec![
        ExamCreatorDatabaseEnvironment::Production,
        ExamCreatorDatabaseEnvironment::Staging,
    ])
    .prop_map(|database_environment| ExamCreatorUserSettings {
        database_environment,
    })
}

// ExamEnvironmentMultipleChoiceQuestionAttempt strategy
pub fn multiple_choice_question_attempt_strategy(
) -> impl Strategy<Value = ExamEnvironmentMultipleChoiceQuestionAttempt> {
    (
        object_id_strategy(),
        prop::collection::vec(object_id_strategy(), 0..3),
        1i64..10000000,
        prop::option::of(any::<i64>().prop_map(|ms| {
            mongodb::bson::DateTime::from_millis(ms.abs() % 1_700_000_000_000)
        })),
    )
        .prop_map(|(id, answers, submission_time_in_m_s, submission_time)| {
            ExamEnvironmentMultipleChoiceQuestionAttempt {
                id,
                answers,
                submission_time_in_m_s: submission_time_in_m_s as f64,
                submission_time,
            }
        })
}

// ExamEnvironmentQuestionSetAttempt strategy
pub fn question_set_attempt_strategy() -> impl Strategy<Value = ExamEnvironmentQuestionSetAttempt> {
    (
        object_id_strategy(),
        prop::collection::vec(multiple_choice_question_attempt_strategy(), 1..10),
    )
        .prop_map(|(id, questions)| ExamEnvironmentQuestionSetAttempt { id, questions })
}

// ExamEnvironmentExamAttempt strategy with existing IDs
pub fn exam_attempt_with_ids_strategy(
    exam_ids: &[ObjectId],
    user_ids: &[ObjectId],
    generated_exam_ids: &[ObjectId],
) -> impl Strategy<Value = ExamEnvironmentExamAttempt> {
    let exam_ids = exam_ids.to_vec();
    let user_ids = user_ids.to_vec();
    let generated_exam_ids = generated_exam_ids.to_vec();

    (
        object_id_strategy(),
        prop::sample::select(user_ids),
        prop::sample::select(exam_ids),
        prop::sample::select(generated_exam_ids),
        prop::collection::vec(question_set_attempt_strategy(), 1..3),
        1i64..10000000,
        prop::option::of(any::<i64>().prop_map(|ms| {
            mongodb::bson::DateTime::from_millis(ms.abs() % 1_700_000_000_000)
        })),
        1i64..10,
    )
        .prop_map(
            |(
                id,
                user_id,
                exam_id,
                generated_exam_id,
                question_sets,
                start_time_in_m_s,
                start_time,
                version,
            )| {
                ExamEnvironmentExamAttempt {
                    id,
                    user_id,
                    exam_id,
                    generated_exam_id,
                    question_sets,
                    start_time_in_m_s: start_time_in_m_s as f64,
                    start_time,
                    version,
                }
            },
        )
}

// ExamEnvironmentGeneratedMultipleChoiceQuestion strategy
pub fn generated_multiple_choice_question_strategy(
) -> impl Strategy<Value = ExamEnvironmentGeneratedMultipleChoiceQuestion> {
    (
        object_id_strategy(),
        prop::collection::vec(object_id_strategy(), 2..6),
    )
        .prop_map(|(id, answers)| ExamEnvironmentGeneratedMultipleChoiceQuestion {
            id,
            answers,
        })
}

// ExamEnvironmentGeneratedQuestionSet strategy
pub fn generated_question_set_strategy(
) -> impl Strategy<Value = ExamEnvironmentGeneratedQuestionSet> {
    (
        object_id_strategy(),
        prop::collection::vec(generated_multiple_choice_question_strategy(), 1..10),
    )
        .prop_map(|(id, questions)| ExamEnvironmentGeneratedQuestionSet { id, questions })
}

// ExamEnvironmentGeneratedExam strategy with existing exam ID
pub fn generated_exam_with_exam_id_strategy(
    exam_ids: &[ObjectId],
) -> impl Strategy<Value = ExamEnvironmentGeneratedExam> {
    let exam_ids = exam_ids.to_vec();
    (
        object_id_strategy(),
        prop::sample::select(exam_ids),
        prop::collection::vec(generated_question_set_strategy(), 1..3),
        prop::bool::ANY,
        1i64..10,
    )
        .prop_map(|(id, exam_id, question_sets, deprecated, version)| {
            ExamEnvironmentGeneratedExam {
                id,
                exam_id,
                question_sets,
                deprecated,
                version,
            }
        })
}

// ExamEnvironmentChallenge strategy with existing exam ID
pub fn challenge_with_exam_id_strategy(
    exam_ids: &[ObjectId],
) -> impl Strategy<Value = ExamEnvironmentChallenge> {
    let exam_ids = exam_ids.to_vec();
    (
        object_id_strategy(),
        prop::sample::select(exam_ids),
        object_id_strategy(),
        1i64..10,
    )
        .prop_map(|(id, exam_id, challenge_id, version)| ExamEnvironmentChallenge {
            id,
            exam_id,
            challenge_id,
            version,
        })
}
