use actix_web::{web, Error, HttpResponse};

use tera::Tera;

use crate::errors::EzyTutorError;
use crate::model::{TutorRegisterForm, TutorResponse, TutorSigninForm, User};
use crate::state::AppState;
use crate::store;

use awc::Client;

use serde_json::json;

use argon2::Config;
use rand::Rng;

pub async fn show_register_form(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");

    tmpl.render("register.html", &ctx)
        .map_err(|err| EzyTutorError::TeraError(err).into())
        .map(|s| HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();

    let username = &params.username;
    let user = store::get_user_record(&app_state.pg_pool, username).await;
    let mut user_found = false;
    let mut password_not_match = false;
    if user.is_ok() {
        user_found = true;
        ctx.insert("error", "User Id already exists");
    } else if params.password != params.confirmation {
        password_not_match = true;
        ctx.insert("error", "Passwords do not match");
    }

    if user_found || password_not_match {
        ctx.insert("current_username", &params.username);
        ctx.insert("current_password", "");
        ctx.insert("current_confirmation", "");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);

        return tmpl
            .render("register.html", &ctx)
            .map_err(|err| EzyTutorError::TeraError(err).into())
            .map(|s| HttpResponse::Ok().content_type("text/html").body(s));
    }

    let new_tutor = json!({
        "tutor_name": &params.name,
        "tutor_pic_url": &params.imageurl,
        "tutor_profile": &params.profile,
    });

    let client = Client::new();
    let resp = client
        .post("http://localhost:3030/tutors/")
        .send_json(&new_tutor)
        .await
        .unwrap()
        .body()
        .await?;

    let tutor_response: TutorResponse = serde_json::from_str(std::str::from_utf8(&resp)?)?;
    let s = format!(
        "Congratulations. You have been successfully registered with EzyTutor \
        and your tutor id is: {}. To start using EzyTutor, please login with your credentials.",
        tutor_response.tutor_id
    );

    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    let hashed = argon2::hash_encoded(params.password.as_bytes(), &salt, &config).unwrap();

    let new_user = User {
        username: params.username.clone(),
        user_password: hashed,
        tutor_id: Some(tutor_response.tutor_id),
    };

    let _created_user = store::post_new_user(&app_state.pg_pool, new_user).await?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_signin_form(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_password", "");

    tmpl.render("signin.html", &ctx)
        .map_err(|err| EzyTutorError::TeraError(err).into())
        .map(|s| HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_signin(
    tmpl: web::Data<Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorSigninForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let username = &params.username;
    let user = store::get_user_record(&app_state.pg_pool, username).await;
    let s;
    match user {
        Ok(user) => {
            let dose_password_match =
                argon2::verify_encoded(&user.user_password, params.password.as_bytes()).unwrap();

            if !dose_password_match {
                ctx.insert("error", "Invalid login");
                ctx.insert("current_name", &params.username);
                ctx.insert("current_password", &params.password);
                s = tmpl
                    .render("signin.html", &ctx)
                    .map_err(|err| Error::from(EzyTutorError::TeraError(err)))?;
            } else {
                ctx.insert("name", &params.username);
                ctx.insert("title", "Signin confirmation!");
                ctx.insert("message", "You have successfully logged in to EzyTutor!");
                s = tmpl
                    .render("user.html", &ctx)
                    .map_err(|err| Error::from(EzyTutorError::TeraError(err)))?;
            }
        }
        Err(_) => {
            ctx.insert("error", "User id not found");
            ctx.insert("current_name", &params.username);
            ctx.insert("current_password", &params.password);
            s = tmpl
                .render("signin.html", &ctx)
                .map_err(|err| Error::from(EzyTutorError::TeraError(err)))?;
        }
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
