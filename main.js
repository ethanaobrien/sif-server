const express = require('express');
const app = express();
const fs = require('fs');

const ip = '0.0.0.0';
const port = 8080;

app.use(express.static('http'))

app.use(function (req, res, next) {
    console.log(req.method, ":", req.url);
    next();
})

const login = require("./handlers/login.js");
app.post("/main.php/login/authkey", login.authKey);
app.post("/main.php/login/startUp", login.startUp);
app.post("/main.php/login/login", login.login);

const user = require("./handlers/user.js");
app.post("/main.php/user/userInfo", user.userInfo);
app.post("/main.php/user/changeName", user.changeName);

app.post("/main.php/gdpr/*", user.gdpr);

app.post("/main.php/tos/*", user.tos);

// TODO!!
app.post("/main.php/tutorial/*", user.tutorial);


const download = require("./handlers/download.js");
app.post("/main.php/download/update", download.update);
app.post("/main.php/download/event", download.event);


app.get("/server_info.zip", (req, res) => {
    res.sendFile(__dirname + "/server_info.zip");
})

function createDirFromFile(path) {
    fs.mkdirSync(require('path').dirname(path), { recursive: true });
}

app.get('/v7/*', function (req, res) {
    const expectedPath = __dirname + "/resources" + req.url;
    createDirFromFile(expectedPath);
    if (fs.existsSync(expectedPath)) {
        res.sendFile(expectedPath);
    } else {
        const file = fs.createWriteStream(expectedPath);
        const request = require('https').get("https://ll.sif.moe" + req.url, function(response) {
           response.pipe(file);

           // after download completed close filestream
           file.on("finish", () => {
               file.close();
               console.log("Download Completed " + req.url);
               res.sendFile(expectedPath);
           });
        });
        
    }
})

app.get('/*', function (req, res) {
    console.log("---------- UNHANDLED request: GET:", req.url);
    res.end("asd");
    //res.redirect(307, 'https://www.school-fes.klabgames.net/');
})

app.post('/*', function (req, res) {
    console.log("---------- UNHANDLED request: POST:", req.url, '\n', req.body);
    res.end();
})

app.listen(port, ip, callback = () => {
    let url = 'http://' + ip + ':' + port
    console.log('Server is listening at', url)
})
