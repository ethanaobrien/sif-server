const {consumeBody, signResp, rel_info, parseBody, timestamp} = require("./global.js");
const account = require("./account.js");

async function personalnoticeGet(req, res) {
    //const body = parseBody((await consumeBody(req)).toString());
    //console.log(body);
    
    const resp = {
        "response_data": {
            "has_notice": false,
            "notice_id": 0,
            "type": 0,
            "title": "",
            "contents": "",
            "server_timestamp": timestamp()
        },
        "release_info": rel_info,
        "status_code": 200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

module.exports = {personalnoticeGet};
