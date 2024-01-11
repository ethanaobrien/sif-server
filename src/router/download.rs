use crate::router::global;
use json::object;
use actix_web::{HttpResponse, HttpRequest, http::header::HeaderValue};
use json::array;

#[cfg(not(debug_assertions))]
use include_dir::{include_dir, Dir};

const DIR: &str = "http://ll.sif.moe/v7/micro_download/<OS>/<VER>/";
const OFFICIAL_DOMAIN: &str = "http://dnw5grz2619mn.cloudfront.net";
const DOMAIN: &str = "http://ll.sif.moe";

#[cfg(not(debug_assertions))]
const ASSETS_DIR: Dir = include_dir!("assets/download_targets/");

#[cfg(debug_assertions)]
use std::fs;

fn get_dl_response(host: &str, dl_type: &str, body: json::JsonValue) -> json::JsonValue {
    let mut os = String::from("android");
    if !body["target_os"].is_null() {
        os = body["target_os"].to_string();
    } else if !body["os"].is_null() {
        os = body["os"].to_string();
    }
    if os.to_lowercase() == "ios" {
        os = String::from("iphone");
    }
    let mut basedir = format!("{}/{}/", os, dl_type);
    #[cfg(debug_assertions)]
    let mut basedir = format!("assets/download_targets/{}", basedir);
    
    let path: String;
    match dl_type {
        "additional" => {
            basedir = format!("{}{}/", basedir, body["package_type"].to_string());
            path = format!("{}{}.json", basedir, body["package_id"].to_string());
        }
        "batch" => {
            if !body["package_type"].is_null() && body["package_type"].to_string() == "4" {
                return array![];
            }
            basedir = format!("{}{}/", basedir, body["package_type"].to_string());
            path = format!("{}all.json", basedir);
        }
        "update" => {
            path = format!("{}{}.json", basedir, body["install_version"].to_string());
        },
        &_ => {
            path = basedir.clone();
        }
    }
    #[cfg(not(debug_assertions))]
    if ASSETS_DIR.get_file(path.clone()).is_none() {
        println!("Download target {} not found. Returning []", path);
        return array![];
    }
    #[cfg(debug_assertions)]
    if !fs::metadata(path.clone()).is_ok() {
        println!("Download target {} not found. Returning []", path);
        return array![];
    }
    
    let mut blacklist = array![];
    
    if !body["excluded_package_ids"].is_null() {
        for (_i, data) in body["excluded_package_ids"].members().enumerate() {
            let path2 = format!("{}{}.json", basedir, data);
            #[cfg(debug_assertions)]
            let pathdata = json::parse(&fs::read_to_string(path2).unwrap()).unwrap();
            #[cfg(not(debug_assertions))]
            let pathdata = json::parse(&ASSETS_DIR.get_file(path2).unwrap().contents_utf8().unwrap()).unwrap();
            
            for (_i, data) in pathdata.members().enumerate() {
                let pa = data["url"].to_string();
                let pa = pa.split('/').collect::<Vec<_>>();
                let pa = pa.iter().rev();
                let pa = pa.collect::<Vec<_>>()[0].split('?').collect::<Vec<_>>()[0];
                let pa = pa.split('.').collect::<Vec<_>>()[0];
                blacklist.push(pa).unwrap();
            }
        }
    }
    
    let mut filtered_data = array![];
    #[cfg(not(debug_assertions))]
    let data = json::parse(&ASSETS_DIR.get_file(path).unwrap().contents_utf8().unwrap()).unwrap();
    #[cfg(debug_assertions)]
    let data = json::parse(&fs::read_to_string(path).unwrap()).unwrap();
    
    for (_i, data) in data.members().enumerate() {
        let url = data["url"].to_string().replace(OFFICIAL_DOMAIN, DOMAIN);
        let fn1 = url.split('/').last().unwrap_or("");
        let fn2 = fn1.split('?').next().unwrap_or("").split('.').next().unwrap_or("");
        if blacklist.contains(fn2) {
            continue;
        }
        let mut to_send = data.clone();
        let url = url.replace(fn1, fn2);
        to_send["url"] = json::JsonValue::String(url);
        filtered_data.push(to_send).unwrap();
    }
    
    if dl_type == "update" {
        let info = include_bytes!("../../assets/server_info.zip");
        let to_push = object!{
            size: info.len(),
            url: format!("{}/server_info.zip", host),
            version: "59.4"
        };
        filtered_data.push(to_push).unwrap();
    }
    return filtered_data;
}

pub fn update(req: HttpRequest, body: String) -> HttpResponse {
    let body = global::process_body(body);
    let blank_header = HeaderValue::from_static("");
    let host = format!("http://{}", req.headers().get("host").unwrap_or(&blank_header).to_str().unwrap_or(""));
    let resp = object!{
        "response_data": get_dl_response(&host, "update", body),
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
pub fn event(req: HttpRequest, _body: String) -> HttpResponse {
    //let body = global::process_body(body);
    let resp = object!{
        "response_data": [],
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
pub fn additional(req: HttpRequest, body: String) -> HttpResponse {
    let body = global::process_body(body);
    let blank_header = HeaderValue::from_static("");
    let host = format!("http://{}", req.headers().get("host").unwrap_or(&blank_header).to_str().unwrap_or(""));
    let resp = object!{
        "response_data": get_dl_response(&host, "additional", body),
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
pub fn batch(req: HttpRequest, body: String) -> HttpResponse {
    let body = global::process_body(body);
    let blank_header = HeaderValue::from_static("");
    let host = format!("http://{}", req.headers().get("host").unwrap_or(&blank_header).to_str().unwrap_or(""));
    let resp = object!{
        "response_data": get_dl_response(&host, "batch", body),
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
pub fn get_url(req: HttpRequest, body: String) -> HttpResponse {
    let body = global::process_body(body);
    let mut ver = "59.4";
    if let Some(version) = req.headers().get("Client-Version") {
        ver = version.to_str().unwrap();
    }
    let mut list = array![];
    let base = DIR.replace("<OS>", &body["os"].to_string().to_lowercase()).replace("<VER>", ver);
    for (_i, data) in body["path_list"].members().enumerate() {
        let link = format!("{}{}", base, data);
        list.push(link).unwrap();
    }
    
    let resp = object!{
        "response_data": {
            url_list: list
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
