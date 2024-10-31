use crate::errors::EzyTutorError;
use crate::models::{NewTutor, Tutor, UpdateTutor};

use sqlx::postgres::PgPool;

pub async fn get_all_tutors(pg_pool: &PgPool) -> Result<Vec<Tutor>, EzyTutorError> {
    let tutors = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile
        FROM ezy_tutor_c6"
    )
    .map(|rec| Tutor {
        tutor_id: rec.tutor_id,
        tutor_name: rec.tutor_name,
        tutor_pic_url: rec.tutor_pic_url,
        tutor_profile: rec.tutor_profile,
    })
    .fetch_all(pg_pool)
    .await?;

    Ok(tutors)
}

pub async fn get_tutor_details(pg_pool: &PgPool, tutor_id: i32) -> Result<Tutor, EzyTutorError> {
    sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile
        FROM ezy_tutor_c6
        WHERE tutor_id = $1",
        tutor_id,
    )
    .map(|rec| Tutor {
        tutor_id: rec.tutor_id,
        tutor_name: rec.tutor_name,
        tutor_pic_url: rec.tutor_pic_url,
        tutor_profile: rec.tutor_profile,
    })
    .fetch_optional(pg_pool)
    .await?
    .ok_or_else(|| EzyTutorError::NotFound("Tutor id not found".to_string()))
}

pub async fn post_new_tutor(pg_pool: &PgPool, new_tutor: NewTutor) -> Result<Tutor, EzyTutorError> {
    let NewTutor {
        tutor_name,
        tutor_pic_url,
        tutor_profile,
    } = new_tutor;
    let tutor = sqlx::query!(
        "INSERT INTO ezy_tutor_c6 (
        tutor_name, tutor_pic_url, tutor_profile)
        VALUES ($1, $2, $3)
        RETURNING
        tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        tutor_name,
        tutor_pic_url,
        tutor_profile,
    )
    .map(|rec| Tutor {
        tutor_id: rec.tutor_id,
        tutor_name: rec.tutor_name,
        tutor_pic_url: rec.tutor_pic_url,
        tutor_profile: rec.tutor_profile,
    })
    .fetch_one(pg_pool)
    .await?;

    Ok(tutor)
}

pub async fn update_tutor_details(
    pg_pool: &PgPool,
    tutor_id: i32,
    update_tutor: UpdateTutor,
) -> Result<Tutor, EzyTutorError> {
    let current = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile
        FROM ezy_tutor_c6
        WHERE tutor_id = $1",
        tutor_id,
    )
    .map(|rec| Tutor {
        tutor_id: rec.tutor_id,
        tutor_name: rec.tutor_name,
        tutor_pic_url: rec.tutor_pic_url,
        tutor_profile: rec.tutor_profile,
    })
    .fetch_optional(pg_pool)
    .await?
    .ok_or_else(|| EzyTutorError::NotFound("Tutor id not found".to_string()))?;

    let name = update_tutor.tutor_name.unwrap_or(current.tutor_name);
    let pic_url = update_tutor.tutor_pic_url.unwrap_or(current.tutor_pic_url);
    let profile = update_tutor.tutor_profile.unwrap_or(current.tutor_profile);

    let updated_tutor = sqlx::query!(
        "UPDATE ezy_tutor_c6 SET
        tutor_name = $1,
        tutor_pic_url = $2,
        tutor_profile = $3
        WHERE tutor_id = $4
        RETURNING
        tutor_id, tutor_name, tutor_pic_url, tutor_profile
        ",
        name,
        pic_url,
        profile,
        tutor_id
    )
    .map(|rec| Tutor {
        tutor_id: rec.tutor_id,
        tutor_name: rec.tutor_name,
        tutor_pic_url: rec.tutor_pic_url,
        tutor_profile: rec.tutor_profile,
    })
    .fetch_one(pg_pool)
    .await?;

    Ok(updated_tutor)
}

pub async fn delete_tutor(pg_pool: &PgPool, tutor_id: i32) -> Result<String, EzyTutorError> {
    let res = sqlx::query!("DELETE FROM ezy_tutor_c6 WHERE tutor_id = $1", tutor_id,)
        .execute(pg_pool)
        .await?;
    Ok(format!("Deleted {:?} record", res))
}
