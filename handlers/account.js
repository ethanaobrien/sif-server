const users = {};
const uids = {};
const fs = require("fs");
// todo - cleanup
const live_status_template = require("./liveStatus.json");
const unit_status_template = fs.readFileSync("./handlers/unitStatus.json");
const {timestamp} = require("./global.js");

//needs to be changed...
const extraUser = {"uid":58969696,"bio":"erm...","award_id":10011,"bg_id":149,"navi_id":460498696,"rank":11};
function getUnit(uid, card) {
    //return uids[uid].active[card];
    return unit_status_template.active[card];
}

function createAccount(token) {
    users[token] = {
        userData: {
            user: {
                "user_id": 67969696,
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
        },
        liveData: JSON.parse(JSON.stringify(live_status_template)),
        unitData: JSON.parse(unit_status_template),
        unitDataReadable: JSON.parse(unit_status_template),
        deckData: {
            "unit_deck_id": 1,
            "main_flag": true,
            "deck_name": "deck 1",
            "unit_owning_user_ids": [
                {
                    "position": 1,
                    "unit_owning_user_id": 460498708
                },
                {
                    "position": 2,
                    "unit_owning_user_id": 460498704
                },
                {
                    "position": 3,
                    "unit_owning_user_id": 460498703
                },
                {
                    "position": 4,
                    "unit_owning_user_id": 460498718
                },
                {
                    "position": 5,
                    "unit_owning_user_id": 460500692
                },
                {
                    "position": 6,
                    "unit_owning_user_id": 460498719
                },
                {
                    "position": 7,
                    "unit_owning_user_id": 460498716
                },
                {
                    "position": 8,
                    "unit_owning_user_id": 460498715
                },
                {
                    "position": 9,
                    "unit_owning_user_id": 460498714
                }
            ]
        }
    }
    uids[users[token]] = users[token].unitData;
    refreshUnitAll(token);
}

function getInfo(token) {
    //erm... is this secure...?
    
    if (!users[token]) {
        createAccount(token);
    }
    users[token].userData.server_timestamp = timestamp();
    return users[token].userData;
}

function getUserData(token) {
    if (!users[token]) {
        createAccount(token);
    }
    users[token].userData.server_timestamp = timestamp();
    return users[token].userData;
}

function setName(token, name) {
    if (!users[token]) {
        createAccount(token);
    }
    users[token].userData.user.name = name;
}

function getLiveStatus(token) {
    if (!users[token]) {
        createAccount(token);
    }
    return users[token].liveData;
}
function getUnitData(token, readable) {
    if (!users[token]) {
        createAccount(token);
    }
    return readable ? users[token].unitDataReadable : users[token].unitData;
}
function getDeckData(token) {
    if (!users[token]) {
        createAccount(token);
    }
    return users[token].deckData;
}
function refreshUnitAll(token) {
	let newList = {
		active: [],
		waiting: []
	}

	for (entries of Object.entries(users[token].unitData.active)) {
		newList.active.push(entries[1]);
	}
	for (entries of Object.entries(users[token].unitData.waiting)) {
		newList.waiting.push(entries[1]);
	}
    users[token].unitDataReadable = newList;
}

module.exports = {getInfo, getUserData, setName, getLiveStatus, getUnitData, extraUser, getUnit, getDeckData};
