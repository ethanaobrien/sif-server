const crypto = require('crypto');
const fs = require('fs');

const privkey = fs.readFileSync('priv.pem');
const maintenance = false;
const rel_info = [
    {"id":423,"key":"UDKkj/dmBRbz+CIB+Ekqyg=="},
    {"id":1870,"key":"Lckl38UoH8CfOMqMSmMYsA=="},
    {"id":1871,"key":"acAmAWyPOCrO+R5qY9UTtQ=="},
    {"id":1872,"key":"LaLzU62pKnTftSEGFhMqfA=="},
    {"id":1873,"key":"wiaaGZSJexvY0u4poRrGSw=="},
    {"id":1874,"key":"T18sDsU+81wLXTjCURNxJw=="}
]

function timestamp() {
    return Math.floor(Date.now() / 1000);
}

function consumeBody(res) {
    return new Promise(function(resolve, reject) {
        let body = Buffer.from('');
        res.on('data', (chunk) => {
            if (chunk) {
                body = Buffer.concat([body, chunk]);
            }
        })
        res.on('end', async function() {
            resolve(body);
        })
    })
}

function signResp(req, res, body) {
    let msgCode = '';
    res.setHeader('version_up', '0');
    res.setHeader('status_code', '200');
    res.setHeader('server_version', '20120129');
    res.setHeader('Server-Version', '59.4');
    res.setHeader('Authorize', 'consumerKey=lovelive_test&timeStamp=' + timestamp() + '&version=1.1&nonce=1');
    
    if (req.header('x-message-code') != undefined) {
        msgCode = body + req.header('x-message-code');
        let msgSign = crypto.sign('sha1', Buffer.from(msgCode, 'utf-8'), {key: privkey}).toString('base64');
        res.setHeader('X-Message-Sign', msgSign);
    }
}

function parseBody(body) {
    // This should be done better...
    try {
        return JSON.parse(body.split("\n")[3].split("\n")[0]);
    } catch(e) {
        return JSON.parse(body); //eehhh
    }
}

module.exports = {consumeBody, signResp, rel_info, parseBody, timestamp, maintenance};
