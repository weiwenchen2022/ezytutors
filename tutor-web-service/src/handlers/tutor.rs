use crate::errors::EzyTutorError;
use crate::models::{NewTutor, UpdateTutor};
use crate::state::AppState;
use crate::store;
use actix_web::{web, HttpResponse};

pub async fn get_all_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, EzyTutorError> {
    store::get_all_tutors(&app_state.pg_pool)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.0;
    store::get_tutor_details(&app_state.pg_pool, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn post_new_tutor(
    app_state: web::Data<AppState>,
    new_tutor: web::Json<NewTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    store::post_new_tutor(&app_state.pg_pool, new_tutor.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
    update_tutor: web::Json<UpdateTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.0;
    store::update_tutor_details(&app_state.pg_pool, tutor_id, update_tutor.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.0;
    store::delete_tutor(&app_state.pg_pool, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
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
    async fn get_all_tutors_success_test() {
        let app_state = new_app_state().await;
        let resp = get_all_tutors(app_state).await.unwrap();
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[actix_rt::test]
    async fn get_tutor_detail_success() {
        let app_state = new_app_state().await;
        let params: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_tutor_details(app_state, params).await.unwrap();
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[ignore = "reason"]
    #[actix_rt::test]
    async fn post_tutor_success() {
        let app_state = new_app_state().await;
        let new_tutor = web::Json(NewTutor {
            tutor_name: "Third tutor".into(),
            tutor_pic_url: "http://tutor.s3.com/ssdfds".to_string(),
            tutor_profile: "Experienced tutor in Statistics".to_string(),
        });
        let resp = post_new_tutor(app_state, new_tutor).await.unwrap();
        assert_eq!(StatusCode::OK, resp.status());
    }

    #[actix_rt::test]
    async fn delete_tutor_success() {
        let app_state = new_app_state().await;
        let params: web::Path<(i32,)> = web::Path::from((2,));
        let resp = delete_tutor(app_state, params).await.unwrap();
        assert_eq!(StatusCode::OK, resp.status());
    }
}
