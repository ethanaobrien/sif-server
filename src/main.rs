#![recursion_limit = "256"]
mod router;
use actix_web::{post, get};
use actix_web::HttpResponse;
use actix_web::HttpRequest;
use actix_web::web;
use actix_web::dev::Service;

#[post("/main.php/login/authkey")]
async fn login_authkey(req: HttpRequest, body: String) -> HttpResponse { router::login::authkey(req, body) }

#[post("/main.php/login/startUp")]
async fn login_start_up(req: HttpRequest, body: String) -> HttpResponse { router::login::start_up(req, body) }

#[post("/main.php/login/login")]
async fn login_login(req: HttpRequest, body: String) -> HttpResponse { router::login::login(req, body) }

#[post("/main.php/login/topInfo")]
async fn login_top_info(req: HttpRequest, body: String) -> HttpResponse { router::login::top_info(req, body) }

#[post("/main.php/user/userInfo")]
async fn user_user_info(req: HttpRequest, body: String) -> HttpResponse { router::user::user_info(req, body) }

#[post("/main.php/user/changeName")]
async fn user_change_name(req: HttpRequest, body: String) -> HttpResponse { router::user::change_name(req, body) }

#[post("/main.php/gdpr/get")]
async fn gdpr_get(req: HttpRequest, body: String) -> HttpResponse { router::gdpr::get(req, body) }

#[post("/main.php/lbonus/execute")]
async fn lbonus_execute(req: HttpRequest, body: String) -> HttpResponse { router::user::lbonus_execute(req, body) }

#[post("/main.php/tos/tosCheck")]
async fn tos_check(req: HttpRequest, body: String) -> HttpResponse { router::user::tos(req, body) }

#[post("/main.php/tos/tosAgree")]
async fn tos_agree(req: HttpRequest, body: String) -> HttpResponse { router::user::tos(req, body) }

#[post("/main.php/tutorial/progress")]
async fn tutorial_progress(req: HttpRequest, body: String) -> HttpResponse { router::tutorial::progress(req, body) }

#[post("/main.php/api")]
async fn main_api(req: HttpRequest, body: String) -> HttpResponse { router::main::api(req, body) }

#[post("/main.php/download/update")]
async fn download_update(req: HttpRequest, body: String) -> HttpResponse { router::download::update(req, body) }

#[post("/main.php/download/event")]
async fn download_event(req: HttpRequest, body: String) -> HttpResponse { router::download::event(req, body) }

#[post("/main.php/download/additional")]
async fn download_additional(req: HttpRequest, body: String) -> HttpResponse { router::download::additional(req, body) }

#[post("/main.php/download/batch")]
async fn download_batch(req: HttpRequest, body: String) -> HttpResponse { router::download::batch(req, body) }

#[post("/main.php/download/getUrl")]
async fn download_get_url(req: HttpRequest, body: String) -> HttpResponse { router::download::get_url(req, body) }

#[get("/server_info.zip")]
async fn server_info() -> HttpResponse {
    HttpResponse::Ok().body(&include_bytes!("../assets/server_info.zip")[..])
}

async fn log_unknown_request(req: HttpRequest) -> HttpResponse {
    println!("Unhandled request: {}", req.path());
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //println!("{}", crate::router::userdata::get_uid("person1"));
    use actix_web::{App, HttpServer};

    let rv = HttpServer::new(|| App::new()
        .wrap_fn(|req, srv| {
            println!("Request: {}", req.path());
            srv.call(req)
        })
        .service(login_authkey)
        .service(login_start_up)
        .service(login_login)
        .service(login_top_info)
        .service(user_user_info)
        .service(user_change_name)
        .service(gdpr_get)
        .service(lbonus_execute)
        .service(tos_check)
        .service(tos_agree)
        .service(tutorial_progress)
        .service(main_api)
        .service(download_update)
        .service(download_event)
        .service(download_additional)
        .service(download_batch)
        .service(download_get_url)
        .service(server_info)
        .default_service(web::route().to(log_unknown_request)))
        .bind(("0.0.0.0", 8080))?
        .run();
    println!("Server started: http://127.0.0.1:{}", 8080);
    rv.await
}
