use crate::errors::EzyTutorError;
use crate::models::{NewCourse, UpdateCourse};
use crate::state::AppState;
use crate::store;

use actix_web::{web, HttpResponse};

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<NewCourse>,
) -> Result<HttpResponse, EzyTutorError> {
    store::post_new_course(&app_state.pg_pool, new_course.into_inner())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id,) = params.into_inner();
    store::get_courses_for_tutor(&app_state.pg_pool, tutor_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    store::get_course_details(&app_state.pg_pool, tutor_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    store::delete_course(&app_state.pg_pool, tutor_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
    update_course: web::Json<UpdateCourse>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();

    store::update_course_datails(
        &app_state.pg_pool,
        tutor_id,
        course_id,
        update_course.into_inner(),
    )
    .await
    .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    async fn new_app_state() -> web::Data<AppState> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pg_pool = PgPool::connect(&database_url).await.unwrap();
        web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            pg_pool,
        })
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        let app_state = new_app_state().await;

        let params: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_courses_for_tutor(app_state, params).await.unwrap();
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[actix_rt::test]
    async fn get_course_detail_success() {
        let app_state = new_app_state().await;

        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = get_course_details(app_state, params).await.unwrap();
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[actix_rt::test]
    async fn get_course_detail_failure() {
        let app_state = new_app_state().await;

        let params: web::Path<(i32, i32)> = web::Path::from((1, 21));
        let resp = get_course_details(app_state, params).await.unwrap_err();
        assert_eq!(StatusCode::NOT_FOUND, resp.status_code());
    }

    #[actix_rt::test]
    #[ignore = "reason"]
    async fn post_course_success() {
        let app_state = new_app_state().await;

        let new_course = web::Json(NewCourse {
            tutor_id: 1,
            course_name: "Third course".to_string(),
            course_description: Some("This is a test course".to_string()),
            course_format: None,
            course_level: Some("Beginner".to_string()),
            course_price: None,
            course_duration: None,
            course_language: Some("English".to_string()),
            course_structure: None,
        });
        let resp = post_new_course(app_state, new_course).await.unwrap();
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[actix_rt::test]
    async fn update_course_success() {
        let app_state = new_app_state().await;

        let update_course = web::Json(UpdateCourse {
            course_name: Some("Course name changed".to_string()),
            course_description: Some("This is yet another test course".to_string()),
            course_format: None,
            course_level: Some("Intermediate".to_string()),
            course_price: None,
            course_duration: None,
            course_language: Some("German".to_string()),
            course_structure: None,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = update_course_details(app_state, params, update_course)
            .await
            .unwrap();
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[actix_rt::test]
    async fn delete_course_success() {
        let app_state = new_app_state().await;

        let params: web::Path<(i32, i32)> = web::Path::from((1, 5));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[actix_rt::test]
    async fn delete_course_failure() {
        let app_state = new_app_state().await;

        let params: web::Path<(i32, i32)> = web::Path::from((1, 21));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(StatusCode::OK, resp.status());
    }
}
