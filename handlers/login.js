const {consumeBody, signResp, rel_info, parseBody, timestamp} = require("./global.js");
const account = require("./account.js");

async function authKey(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    //console.log(body);
    
    //console.log("login.authkey");
    const resp = {
        "response_data": {
            "authorize_token": btoa(body.device_token),
            "dummy_token": body.dummy_token
        },
        "release_info": rel_info,
        "status_code": 200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

async function startUp(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    //console.log(body);
    const device_token = btoa(body.devtoken);
    const acc = account.getInfo(device_token);
    
    const resp = {
        "response_data": {
            "user_id": acc.user.user_id,
            "release_info": rel_info
        },
        "status_code": 200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

async function login(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    //console.log(body);
    const device_token = btoa(body.devtoken);
    const acc = account.getInfo(device_token);
    
    const resp = {
        "response_data": {
            "authorize_token": device_token,
            "user_id": acc.user.user_id,
            "dummy_token": body.dummy_token,
            "review_version": "",
            "server_timestamp": timestamp(),
            "idfa_enabled": false,
            "skip_login_news": true,
            "release_info": rel_info
        },
        "status_code": 200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}


module.exports = { authKey, startUp, login };
