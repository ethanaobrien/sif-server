#![recursion_limit = "256"]
mod router;
use actix_web::{post, get};
use actix_web::HttpResponse;
use actix_web::HttpRequest;
use actix_web::Responder;
use actix_web::web;

#[post("/main.php/login/authkey")]
async fn login_authkey(req: HttpRequest, body: String) -> HttpResponse {
    println!("Request: {}", req.path());
    router::login::authkey(req, body)
}
#[post("/main.php/login/startUp")]
async fn login_start_up(req: HttpRequest, body: String) -> HttpResponse {
    println!("Request: {}", req.path());
    router::login::start_up(req, body)
}
#[post("/main.php/login/login")]
async fn login_login(req: HttpRequest, body: String) -> HttpResponse {
    println!("Request: {}", req.path());
    router::login::login(req, body)
}
#[post("/main.php/login/topInfo")]
async fn login_top_info(req: HttpRequest, body: String) -> HttpResponse {
    println!("Request: {}", req.path());
    router::login::top_info(req, body)
}
#[post("/main.php/user/userInfo")]
async fn user_user_info(req: HttpRequest, body: String) -> HttpResponse {
    println!("Request: {}", req.path());
    router::user::user_info(req, body)
}
#[post("/main.php/gdpr/get")]
async fn gdpr_get(req: HttpRequest, body: String) -> HttpResponse {
    println!("Request: {}", req.path());
    router::gdpr::get(req, body)
}
#[post("/main.php/lbonus/execute")]
async fn lbonus_execute(req: HttpRequest, body: String) -> HttpResponse {
    println!("Request: {}", req.path());
    router::user::lbonus_execute(req, body)
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
        .service(login_authkey)
        .service(login_start_up)
        .service(login_login)
        .service(login_top_info)
        .service(user_user_info)
        .service(gdpr_get)
        .service(lbonus_execute)
        .default_service(web::route().to(log_unknown_request)))
        .bind(("0.0.0.0", 8080))?
        .run();
    println!("Server started: http://127.0.0.1:{}", 8080);
    rv.await
}

/*
    fn post(mut res: Request) {
        //this is not acceptible me
        if res.path == "/" {
            router::login::authkey(res);
        } else if res.path == "/main.php/login/startUp" {
            router::login::start_up(res);
        } else if res.path == "/main.php/login/login" {
            router::login::login(res);
        } else if res.path == "/main.php/login/topInfo" {
            router::login::top_info(res);
        } else if res.path.starts_with("/main.php/tos/") {
            
        } else if res.path.starts_with("/main.php/gdpr/") {
            
        } else if res.path.starts_with("/main.php/tutorial/") {
            
        } else if res.path == "/main.php/lbonus/execute" {
            
        } else if res.path == "/main.php/handover/kidStatus" {
            
        } else if res.path == "/main.php/friend/list" {
            
        } else if res.path == "/main.php/user/userInfo" {
            
        } else if res.path == "/main.php/user/changeName" {
            
        } else if res.path == "/main.php/user/changeNavi" {
            
        } else if res.path == "/main.php/download/update" {
            
        } else if res.path == "/main.php/download/event" {
            
        } else if res.path == "/main.php/download/additional" {
            
        } else if res.path == "/main.php/download/batch" {
            
        } else if res.path == "/main.php/download/getUrl" {
            
        } else if res.path == "/main.php/api" {
            
        } else if res.path == "/main.php/ranking/live" {
            
        } else if res.path == "/main.php/ranking/player" {
            
        } else if res.path == "/main.php/unit/deck" {
            
        } else if res.path == "/main.php/unit/deckName" {
            
        } else if res.path == "/main.php/unit/setDisplayRank" {
            
        } else if res.path == "/main.php/live/partyList" {
            
        } else if res.path == "/main.php/live/preciseScore" {
            
        } else if res.path == "/main.php/live/play" {
            
        } else if res.path == "/main.php/live/reward" {
            
        } else if res.path == "/main.php/notice/noticeFriendGreeting" {
            
        } else if res.path == "/main.php/notice/noticeFriendVariety" {
            
        } else if res.path == "/main.php/notice/noticeUserGreetingHistory" {
            
        } else if res.path == "/main.php/award/set" {
            
        } else if res.path == "/main.php/background/set" {
            
        } else if res.path == "/main.php/profile/profileRegister" {
            
        } else if res.path == "/main.php/area/list" {
            
        } else if res.path == "/main.php/announce/checkState" {
            
        } else if res.path == "/main.php/personalnotice/get" {
            
        } else if res.path == "/main.php/event/eventList" {
            
        } else if res.path == "/main.php/payment/productList" {
            
        } else if res.path == "/main.php/payment/month" {
            
        } else if res.path == "/main.php/exchange/itemInfo" {
            
        } else if res.path == "/main.php/album/seriesAll" {
            
        } else if res.path == "/main.php/museum/info" {
            
        } else if res.path == "/main.php/banner/bannerList" {
            
        } else if res.path == "/main.php/secretbox/pon" {
            
        } else if res.path == "/main.php/secretbox/multi" {
            
        } else if res.path == "/main.php/scenario/reward" {
            
        } else if res.path == "/main.php/subscenario/reward" {
            
        } else if res.path == "/main.php/multiunit/scenarioReward" {
            
        } else if res.path == "/main.php/reward/rewardHistory" {
            
        }
    }
}
*/