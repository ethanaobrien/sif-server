use crate::router::global;
use json::object;
use actix_web::{HttpResponse, HttpRequest};

pub fn personalnotice_get(req: HttpRequest, _body: String) -> HttpResponse {
    let resp = object!{
        "response_data": {
            has_notice: false,
            notice_id: 0,
            type: 0,
            title: "",
            contents: "",
            server_timestamp: global::timestamp()
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn handover_kidstatus(req: HttpRequest, _body: String) -> HttpResponse {
    let resp = object!{
        "response_data": {
            has_klab_id: true,
            server_timestamp: global::timestamp()
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
