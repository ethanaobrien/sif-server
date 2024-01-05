const {consumeBody, signResp, rel_info, parseBody, timestamp} = require("./global.js");
const account = require("./account.js");

async function userInfo(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    //console.log(body, req.headers);
    const acc = account.getUserData(req.headers.authorize.split("token=").pop().split("&")[0]);
    //console.log("user info: ", acc);
    
    const resp = {
        "response_data": acc,
        "release_info": rel_info,
        "status_code": 200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

async function gdpr(req, res) {
    const resp = {
        "response_data": {
            "enable_gdpr": true,
            "is_eea": false,
            "server_timestamp": timestamp()
        },
        "release_info": rel_info,
        "status_code":200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

async function tos(req, res) {
    const resp = {
        "response_data": {
            "tos_id": 8,
            "tos_type": 3,
            "is_agreed": true,
            "server_timestamp": timestamp()
        },
        "release_info": rel_info,
        "status_code":200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

async function changeName(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    const token = req.headers.authorize.split("token=").pop().split("&")[0];
    const acc = account.getUserData(token);
    
    const resp = {
        "response_data": {
            "before_name": acc.user.name,
            "after_name": body.name,
            "server_timestamp": timestamp()
        },
        "release_info": rel_info,
        "status_code":200
    }
    account.setName(token, body.name);
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

async function tutorial(req, res) {
    
    const resp = {
        "response_data": [],
        "release_info": rel_info,
        "status_code": 200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

module.exports = {userInfo, gdpr, tos, changeName, tutorial};
