# Exam Creator Seeder

A Rust binary that uses [proptest](https://docs.rs/proptest/latest/proptest/) to generate arbitrary test data for seeding MongoDB databases with realistic exam data.

## Overview

The seeder generates 3 of each type with proper referential integrity:
- **Exams** (ExamCreatorExam) - Exam configurations with question sets
- **Users** (ExamCreatorUser) - User accounts with settings
- **Generated Exams** (ExamEnvironmentGeneratedExam) - Generated exam instances
- **Exam Attempts** (ExamEnvironmentExamAttempt) - User attempts at exams
- **Challenges** (ExamEnvironmentChallenge) - Challenge mappings to exams

## Usage

### Prerequisites

1. Set up environment variables (copy from `sample.env`):
   ```bash
   MONGODB_URI_STAGING=mongodb://...
   MONGODB_URI_PRODUCTION=mongodb://...
   ```

2. Ensure MongoDB is running and accessible

### Running the Seeder

Build and run:
```bash
cargo run --package seeder
```

This will seed 3 of each collection type to the staging database by default.

To use production database:
```bash
MONGODB_ENV=production cargo run --package seeder
```

### Environment Variables

- `MONGODB_URI_STAGING` - MongoDB connection string for staging (required)
- `MONGODB_URI_PRODUCTION` - MongoDB connection string for production (required)
- `MONGODB_ENV` - Database environment to use: `staging` (default) or `production`

### What Gets Seeded

The seeder always creates 3 of each type in this order:
1. **3 Exams** → exam-creator.Exam
2. **3 Users** → exam-creator.User
3. **3 Generated Exams** → exam-environment.GeneratedExam (references Exams)
4. **3 Exam Attempts** → exam-environment.ExamAttempt (references Exams, Users, Generated Exams)
5. **3 Challenges** → exam-environment.Challenge (references Exams)

All foreign key relationships are maintained automatically.

## How It Works

### Seeding Order

The seeder automatically handles dependencies by seeding in the correct order:

1. **Exams & Users** (independent) → Created first
2. **Generated Exams** → Uses existing exam IDs
3. **Exam Attempts** → Uses existing exam, user, and generated exam IDs
4. **Challenges** → Uses existing exam IDs

### Referential Integrity

All foreign key relationships are maintained:
- `GeneratedExam.examId` → `Exam._id`
- `ExamAttempt.examId` → `Exam._id`
- `ExamAttempt.userId` → `User._id`
- `ExamAttempt.generatedExamId` → `GeneratedExam._id`
- `Challenge.examId` → `Exam._id`

### Data Generation

Uses proptest strategies to generate realistic data:
- Follows Prisma schema constraints
- Realistic value ranges
- Includes optional fields with appropriate probability

## Example Output

```
Seeding all collections with 3 items each
Using staging database environment
📝 Seeding 3 exams...
  ✓ Inserted exam 1/3
  ✓ Inserted exam 2/3
  ✓ Inserted exam 3/3
👤 Seeding 3 users...
  ✓ Inserted user 1/3
  ✓ Inserted user 2/3
  ✓ Inserted user 3/3
🎲 Seeding 3 generated exams...
  ✓ Inserted generated exam 1/3
  ✓ Inserted generated exam 2/3
  ✓ Inserted generated exam 3/3
📋 Seeding 3 exam attempts...
  ✓ Inserted exam attempt 1/3
  ✓ Inserted exam attempt 2/3
  ✓ Inserted exam attempt 3/3
🎯 Seeding 3 challenges...
  ✓ Inserted challenge 1/3
  ✓ Inserted challenge 2/3
  ✓ Inserted challenge 3/3
✅ Seeding complete!
```

## Troubleshooting

### Connection Errors

```
Error: MONGODB_URI_STAGING environment variable not set
```

**Solution**: Copy `sample.env` to `.env` and configure MongoDB URIs

### Missing Dependencies

If you see errors about missing exams/users, ensure MongoDB is accessible and the seeder completed successfully.
