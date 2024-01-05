const users = {};
const {timestamp} = require("./global.js");

function createAccount(token) {
    users[token] = {
        user: {
            "user_id": 10000001,
            "name": "New person",
            "level": 0,
            "exp": 0,
            "previous_exp": 778427,
            "next_exp": 786041,
            "game_coin": 696903030,
            "sns_coin": 6903069,
            "free_sns_coin": 6903069,
            "paid_sns_coin": 0,
            "social_point": 696969,
            "unit_max": 9999,
            "waiting_unit_max": 9999,
            "energy_max": 6969,
            "energy_full_time": "2023-03-21 13:13:41",
            "license_live_energy_recoverly_time": 60,
            "energy_full_need_time": 0,
            "over_max_energy": 6969,
            "training_energy": 69,
            "training_energy_max": 69,
            "friend_max": 69,
            "invite_code": "258506983",
            "insert_date": "2018-05-01 19:28:30",
            "update_date": "2018-05-01 19:28:30",
            "tutorial_state": -1,
            "lp_recovery_item": []
        },
        "ad_status": true,
        "server_timestamp": "<TIMESTAMP>"
    }
}

function getInfo(token) {
    //erm... is this secure...?
    
    if (!users[token]) {
        createAccount(token);
    }
    users[token].server_timestamp = timestamp();
    return users[token];
}

function getUserData(token) {
    if (!users[token]) {
        createAccount(token);
    }
    users[token].server_timestamp = timestamp();
    return users[token];
}

function setName(token, name) {
    if (!users[token]) {
        createAccount(token);
    }
    users[token].server_timestamp = timestamp();
    users[token].user.name = name;
}

module.exports = {getInfo, getUserData, setName};
