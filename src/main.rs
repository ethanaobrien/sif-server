use std::{thread, time::Duration};
use server::{
    Server,
    Settings,
    file_system::GetByPath,
    Request,
    wsparser::WebSocketParser,
    httpcodes::get_http_message,
    decode_base64
};

pub struct LlsifServer {
    server: Server
}

#[allow(dead_code)]
impl LlsifServer {
    fn log(msg: String) {
        println!("{}", msg);
    }
    pub fn new(opts: Settings<'static>) -> LlsifServer {
        LlsifServer {
            server: Server::new(opts, LlsifServer::on_request, LlsifServer::on_websocket)
        }
    }
    pub fn start(&mut self) -> bool {
        self.server.start()
    }
    pub fn terminate(&mut self) {
        self.server.terminate()
    }
    fn on_websocket(mut res: WebSocketParser, _opts: Settings) {
        //ignore for now... I need to add the ability to terminate the connection
    }
    fn on_request(mut res:Request, opts: Settings) {
        println!("Request: {} {}", res.method, res.path);
        res.set_header("Connection", "keep-alive");
        res.set_header("Accept-ranges", "bytes");
        
        //https://docs.rs/sqlite/latest/sqlite/
        //https://docs.rs/json/latest/json/
        
        //if res.method == "GET" || res.method == "HEAD" {
        //    Self::get(res, opts, rewrite_to);
        //} else if res.method == "OPTIONS" {
        //    res.end();
        //} else {
            Self::error(res, opts, "", 501);
        //}
    }
    fn error(mut res:Request, opts: Settings, msg: &str, code: i32) {
        if code == 401 {
            res.set_header("WWW-Authenticate", "Basic realm=\"Server\", charset=\"UTF-8\"");
        }
        let def_msg = format!("<h1>{} - {}</h1>\n\n<p>{}</p>", code, get_http_message(code), msg);
        let default_msg = def_msg.as_bytes();
        let size = default_msg.len();
        res.set_header("Content-length", &size.to_string());
        res.set_status(code);
        if res.method != "HEAD" {
            res.write(default_msg);
        }
        res.end();
    }
    fn get(mut res:Request, opts: Settings, rewrite_to: &str) {
		
    }
}

fn main() {

    let cert = String::new();
    let key = String::new();
    
	/*
    if args.https {
        match generate_dummy_cert_and_key() {
            Ok((certt, keyy)) => {
                cert = certt;
                key = keyy;
            }
            Err(err) => {
                eprintln!("Error generating certificate and key: {:?}", err);
            }
        }
    }*/
    

    let settings = Settings {
        path: "/",
        index: false,
        local_network: true,
        port: 8080,
        spa: false,
        rewrite_to: "",
        directory_listing: false,
        exclude_dot_html: false,
        ipv6: false,
        hidden_dot_files: true,
        cors: false,
        upload: false,
        replace: false,
        delete: false,
        hidden_dot_files_directory_listing: true,
        custom500: "",
        custom404: "",
        custom403: "",
        custom401: "",
        http_auth: false,
        http_auth_username: "く",
        http_auth_password: "password",
        https: false,
        https_cert: "/",
        https_key: "/"
    };
    let mut server = LlsifServer::new(settings);
    println!("Server started: {}", server.start());
    //let mut i = 0;
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}
