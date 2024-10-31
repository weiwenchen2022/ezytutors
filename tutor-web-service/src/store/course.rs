use crate::errors::EzyTutorError;
use crate::models::{Course, NewCourse, UpdateCourse};

use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor(
    pg_pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, EzyTutorError> {
    // Prepare SQL statement
    let courses = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 WHERE tutor_id = $1",
        tutor_id,
    )
    .fetch_all(pg_pool)
    .await?;

    Ok(courses)
}

pub async fn get_course_details(
    pg_pool: &PgPool,
    totur_id: i32,
    course_id: i32,
) -> Result<Course, EzyTutorError> {
    // Prepare SQL statement
    let course = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_c6
        WHERE tutor_id = $1 and course_id = $2
        ",
        totur_id,
        course_id,
    )
    .fetch_optional(pg_pool)
    .await?;

    course.ok_or_else(|| EzyTutorError::NotFound("Course id not found".to_string()))
}

pub async fn post_new_course(
    pg_pool: &PgPool,
    new_course: NewCourse,
) -> Result<Course, EzyTutorError> {
    let NewCourse {
        tutor_id,
        course_name,
        course_description,
        course_format,
        course_structure,
        course_duration,
        course_price,
        course_language,
        course_level,
    } = new_course;
    let new_course = sqlx::query_as!(
        Course,
        "INSERT INTO ezy_course_c6 (
        tutor_id, course_name,
        course_description, course_duration,
        course_level, course_format, course_language,
        course_structure, course_price
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING course_id, tutor_id, course_name,
        course_description, course_duration,
        course_level, course_format, course_language,
        course_structure, course_price,
        posted_time",
        tutor_id,
        course_name,
        course_description,
        course_duration,
        course_level,
        course_format,
        course_language,
        course_structure,
        course_price
    )
    .fetch_one(pg_pool)
    .await?;

    Ok(new_course)
}

pub async fn delete_course(
    pg_pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<String, EzyTutorError> {
    let res = sqlx::query!(
        "DELETE FROM ezy_course_c6 WHERE tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id,
    )
    .execute(pg_pool)
    .await?;

    Ok(format!("Deleted {:?} record", res))
}

pub async fn update_course_datails(
    pg_pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, EzyTutorError> {
    eprintln!("tutor_id: {}, course_id: {}", tutor_id, course_id);

    // Retrieve current record
    let current = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 WHERE tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id,
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|_| EzyTutorError::NotFound("Course id not found".to_string()))?;

    let name = update_course.course_name.unwrap_or(current.course_name);
    let description = update_course
        .course_description
        .unwrap_or_else(|| current.course_description.unwrap_or_default());
    let format = update_course
        .course_format
        .unwrap_or_else(|| current.course_format.unwrap_or_default());
    let structure = update_course
        .course_structure
        .unwrap_or_else(|| current.course_structure.unwrap_or_default());
    let duration = update_course
        .course_duration
        .unwrap_or_else(|| current.course_duration.unwrap_or_default());
    let level = update_course
        .course_level
        .unwrap_or_else(|| current.course_level.unwrap_or_default());
    let language = update_course
        .course_language
        .unwrap_or_else(|| current.course_language.unwrap_or_default());
    let price = update_course
        .course_price
        .unwrap_or_else(|| current.course_price.unwrap_or_default());

    let updated_course = sqlx::query_as!(
        Course,
        "UPDATE ezy_course_c6 SET
            course_name = $1,
            course_description = $2,
            course_format = $3,
            course_structure = $4,
            course_duration = $5,
            course_price = $6,
            course_language = $7,
            course_level = $8
        WHERE tutor_id = $9 and course_id = $10
        RETURNING
            tutor_id, course_id,
            course_name, course_description,
            course_duration, course_level,
            course_format, course_language,
            course_structure, course_price, posted_time",
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        tutor_id,
        course_id
    )
    .fetch_one(pg_pool)
    .await?;
    Ok(updated_course)
}
