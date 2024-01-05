const {maintenance, consumeBody, signResp, rel_info, parseBody, timestamp} = require("./global.js");
const fs = require("fs");

const workaroundUnitBatchDL = true;
const official_cdn_domain = 'http://dnw5grz2619mn.cloudfront.net';
const cdn_path = 'http://192.168.1.15:51376';
const microdl_dir = 'http://192.168.1.15:51376/v7/micro_download/<OS>/<VER>/'

function getHostPort(req) {
    return req.protocol + "://" + req.headers.host.replace("localhost", "127.0.0.1")
}

function getDLResponse(host, download_type, req_body) {
    let os = (req_body.target_os != undefined ? req_body.target_os : req_body.os);
    if (os.toLowerCase() === 'ios') os = 'iphone';
    let basedir = 'static/download_targets/' + os + '/' + download_type + '/';
    let path = basedir;
    switch (download_type) {
        case "additional": {
            basedir = basedir + req_body.package_type + '/';
            path = basedir + req_body.package_id + '.json';
            break;
        }
        case "batch": {
            if (req_body.package_type == 4 && workaroundUnitBatchDL) return [];

            basedir = basedir + req_body.package_type + '/'
            path = basedir + (req_body.package_type != 4 ? 'all.json' : 'all-out.json')
            break
        }
        case "update": {
            path = basedir + req_body.install_version + '.json'
            break
        }
    }
    if (!fs.existsSync(path)) {
        console.warn("download target", path, "not found, returning empty array...");
        return [];
    }

    let blacklist = []
    if (req_body.excluded_package_ids != undefined && typeof req_body.excluded_package_ids[Symbol.iterator] == 'function') {
        for (id of req_body.excluded_package_ids) {
            let ppath = basedir + id + '.json';
            let pdata = JSON.parse(fs.readFileSync(ppath, 'utf8'));
            for (p of pdata) {
                let fn = p.url.split('/').reverse()[0].split('?')[0].split('.')[0];
                blacklist.push(fn);
            }
        }
    }

    let filtered_data = [];
    let data = JSON.parse(fs.readFileSync(path, 'utf8'));
    for (i in data) {
        let newUrl = data[i].url.replace(official_cdn_domain, cdn_path);
        let fn1 = newUrl.split('/').reverse()[0];
        let fn2 = fn1.split('?')[0].split('.')[0];

        if (blacklist.indexOf(fn2) > -1) continue;

        data[i].url = newUrl.replace(fn1, fn2);
        filtered_data.push(data[i]);
    }
    if (download_type === "update") {
        let size = fs.statSync("server_info.zip").size;
        let url = host + "/server_info.zip";

        if (size > 0) {
            console.log("serving additional server_info", url);
            filtered_data.push({
                "size": size,
                "url": url,
                "version": "59.4"
            })
        }
    }
    return filtered_data;
}

async function update(req, res) {
    if (maintenance) {
        res.setHeader('Maintenance', 1);
        signResp(req, res, " ");
        res.send(" ");
        res.end();
        return;
    }
    const body = parseBody((await consumeBody(req)).toString());
    //console.log(body);
    
    const toSend = {
        "response_data": getDLResponse(getHostPort(req), 'update', body),
        "release_info": rel_info,
        "status_code": 200
    }
    //console.log(toSend);
    
    const data = JSON.stringify(toSend);
    signResp(req, res, data);
    res.send(data);
    res.end();
}

async function event(req, res) {
    if (maintenance) {
        res.setHeader('Maintenance', 1);
        signResp(req, res, " ");
        res.send(" ");
        res.end();
        return;
    }
    const toSend = {
        "response_data": [],
        "release_info": rel_info,
        "status_code": 200
    }
    
    const data = JSON.stringify(toSend);
    signResp(req, res, data);
    res.send(data);
    res.end();
}

async function additional(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    //console.log(body);
    
    const toSend = {
        "response_data": getDLResponse(getHostPort(req), 'additional', body),
        "release_info": rel_info,
        "status_code": 200
    }
    //console.log(toSend);
    
    const data = JSON.stringify(toSend);
    signResp(req, res, data);
    res.send(data);
    res.end();
}

async function batch(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    //console.log(body);
    
    const toSend = {
        "response_data": getDLResponse(getHostPort(req), 'batch', body),
        "release_info": rel_info,
        "status_code": 200
    }
    //console.log(toSend);
    
    const data = JSON.stringify(toSend);
    signResp(req, res, data);
    res.send(data);
    res.end();
}

async function getUrl(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
	let ver = '59.4';
	if (req.header('Client-Version') != undefined) {
		ver = req.header('Client-Version');
	}

    let base_dir = microdl_dir.replace('<OS>', body.os.toLowerCase()).replace("<VER>", ver);
    const list = [];
    for (x of body.path_list) {
        let link = (base_dir + x);
        list.push(link);
    }

	let toSend = {
		response_data: {
			url_list: list
		},
		release_info: rel_info,
		status_code: 200
	}
    console.log(toSend);
    const data = JSON.stringify(toSend);
    signResp(req, res, data);
    res.send(data);
    res.end();
}

module.exports = {update, event, additional, batch, getUrl};
