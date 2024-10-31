use actix_web::{web, Error, HttpResponse};
use tera::Tera;

use crate::model::{
    CourseResponse, NewCourse, NewCourseResponse, UpdateCourse, UpdateCourseResponse,
};
use crate::state::AppState;

use serde_json::{json, Value};

use awc::Client;

pub async fn handle_get_course(params: web::Path<(i32,)>) -> Result<HttpResponse, Error> {
    let (tutor_id,) = params.into_inner();

    let get_url = format!("http://localhost:3030/courses/{}", tutor_id);
    let client = Client::new();
    let resp = client.get(get_url).send().await.unwrap().body().await?;

    let courses: Vec<CourseResponse> = serde_json::from_str(std::str::from_utf8(&resp)?)?;

    Ok(HttpResponse::Ok().json(courses))
}

pub async fn handle_insert_course(
    _tmpl: web::Data<Tera>,
    _app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
    new_course: web::Json<NewCourse>,
) -> Result<HttpResponse, Error> {
    let (tutor_id,) = params.into_inner();
    let NewCourse {
        course_name,
        course_description,
        course_format,
        course_duration,
        course_structure,
        course_price,
        course_language,
        course_level,
    } = new_course.into_inner();

    let new_course = json!({
        "tutor_id": tutor_id,
        "course_name": course_name,
        "course_description": course_description,
        "course_format": course_format,
        "course_structure": course_structure,
        "course_duration": course_duration,
        "course_price": course_price,
        "course_language": course_language,
        "course_level": course_level
    });
    let client = Client::new();
    let resp = client
        .post("http://localhost:3030/courses")
        .send_json(&new_course)
        .await
        .unwrap()
        .body()
        .await?;

    println!("Finished call: {:?}", resp);

    let course: NewCourseResponse = serde_json::from_str(std::str::from_utf8(&resp)?)?;

    Ok(HttpResponse::Ok().json(course))
}

pub async fn hanlde_update_course(
    _tmpl: web::Data<Tera>,
    _app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
    update_course: web::Json<UpdateCourse>,
) -> Result<HttpResponse, Error> {
    let (tutor_id, course_id) = params.into_inner();
    let UpdateCourse {
        course_name,
        course_description,
        course_format,
        course_duration,
        course_structure,
        course_price,
        course_language,
        course_level,
    } = update_course.into_inner();

    let update_course = json!({
         "course_name": course_name,
        "course_description": course_description,
        "course_format":course_format,
        "course_duration": course_duration,
       "course_structure": course_structure,
        "course_price": course_price,
        "course_language": course_language,
        "course_level":course_level,
    });

    let client = Client::new();
    let update_url = format!("http://localhost:3030/courses/{}/{}", tutor_id, course_id);
    let resp = client
        .put(update_url)
        .send_json(&update_course)
        .await
        .unwrap()
        .body()
        .await?;

    let course: UpdateCourseResponse = serde_json::from_str(std::str::from_utf8(&resp)?)?;

    Ok(HttpResponse::Ok().json(course))
}

pub async fn handle_delete_course(
    _tmpl: web::Data<Tera>,
    _app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let (tutor_id, course_id) = params.into_inner();

    let client = Client::new();
    let delete_url = format!("http://localhost:3030/courses/{}/{}", tutor_id, course_id);
    let resp = client
        .delete(delete_url)
        .send()
        .await
        .unwrap()
        .body()
        .await?;

    let resp: Value = serde_json::from_str(std::str::from_utf8(&resp)?)?;
    Ok(HttpResponse::Ok().json(resp))
}
