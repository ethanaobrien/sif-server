const {consumeBody, signResp, rel_info, parseBody, timestamp} = require("./global.js");
const account = require("./account.js");
const fs = require("fs");

const game_consts = {
    gachaPools: JSON.parse(fs.readFileSync('static/main.php-api/partial/secretbox.all.result.json', 'utf8')),
    normalChartMap: JSON.parse(fs.readFileSync('static/normal_live_map.json', 'utf8')),
    specialChartMap: JSON.parse(fs.readFileSync('static/special_live_map.json', 'utf8')),
    chartRankMap: JSON.parse(fs.readFileSync('static/live_setting_map.json')),
    liveGoalMap: JSON.parse(fs.readFileSync('static/live_goal_reward_map.json')),
    museum_info_txt: fs.readFileSync('static/museum_info.json', 'utf8'),
    scenarioList: JSON.parse(fs.readFileSync('static/scenario/scenario_m.json', 'utf8')),
    subscenarioList: JSON.parse(fs.readFileSync('static/card_navi_ids.json', 'utf8')),
    multiUnitScenarioList: JSON.parse(fs.readFileSync('static/scenario/multi_unit_scenario_open_m.json', 'utf8')),
    multiUnitScenarioMap: JSON.parse(fs.readFileSync('static/scenario/multi_unit_scenario_map.json', 'utf8')),
    units: JSON.parse(fs.readFileSync('static/all_units.json')),
    availableAwardRanges: [
        [1, 572],
        [1300, 1300],
        [10000, 10011],
        [999001, 999030]
    ],
    availableAwardBlacklists: [
        [455, 456, 457, 458, 475, 477, 478],
        [],
        [],
        [999007, 999015]
    ],
    diffNames: [
        "EASY",
        "NORMAL",
        "HARD",
        "EXPERT",
        "MASTER",
        "MASTER"
    ],
    gachaItemIds: [
        1, 5, 23, 48, 49,
        50, 61, 62, 63, 168,
        193, 207, 256, 272, 349,
        353, 380, 382, 422, 1000,
        1200
    ]
}

async function api(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    const auth = req.headers.authorize.split("token=").pop().split("&")[0];
    const userData = account.getUserData(auth);
    const userLiveStatus = account.getLiveStatus(auth);
    const userUnitData = account.getUnitData(auth, true);
    const userDeckData = account.getDeckData(auth);
    //console.log(body);
    let resp_data = [];
    let time = req.headers.authorize.split("timeStamp=").pop().split("&")[0];
    time = timestamp();
    
    body.forEach(request => {
        let sfilename = "static/main.php-api/" + request.module + "." + request.action + ".result.json";
        if (fs.existsSync(sfilename)) {
            let sresult = JSON.parse(fs.readFileSync(sfilename, "utf8"));
            resp_data.push({
                result: sresult,
                status: 200,
                commandNum: false,
                timeStamp: parseInt(time)
            })
            return;
        }
        let dlen = resp_data.length;
        switch (request.module) {
            case "login": {
                switch (request.action) {
                    case "topInfo": {
                        resp_data.push({
                            result: {
                                friend_action_cnt: 0,//1291,
                                friend_greet_cnt: 0,
                                friend_variety_cnt: 0,//1289,
                                friend_new_cnt: 0,
                                present_cnt: 0,
                                secret_box_badge_flag: false,
                                server_datetime: (new Date()).toISOString().replace('T', ' ').split('.')[0],
                                server_timestamp: timestamp(),
                                notice_friend_datetime: (new Date()).toISOString().replace('T', ' ').split('.')[0],
                                notice_mail_datetime: "2019-12-22 13:03:23",
                                friends_approval_wait_cnt: 0,
                                friends_request_cnt: 0,
                                is_today_birthday: false,
                                license_info: {
                                    license_list: [],
                                    licensed_info: [],
                                    expired_info: [],
                                    badge_flag: false
                                },
                                using_buff_info: [],
                                is_klab_id_task_flag: false,
                                klab_id_task_can_sync: false,
                                has_unread_announce: false, // true,
                                live_skip_open_flag: true,
                                exchange_badge_cnt: [
                                    493,
                                    12,
                                    345
                                ],
                                ad_flag: true,
                                has_ad_reward: true
                            },
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break
                    }
                    case "topInfoOnce": {
                        resp_data.push({
                            result: {
                                new_achievement_cnt: 0,
                                unaccomplished_achievement_cnt: 0,
                                live_daily_reward_exist: false,
                                training_energy: 69,
                                training_energy_max: 69,
                                notification: {
                                    push: false,
                                    lp: false,
                                    update_info: false,
                                    campaign: false,
                                    live: false,
                                    lbonus: false,
                                    event: false,
                                    secretbox: false,
                                    birthday: false
                                },
                                open_arena: false,// true,
                                costume_status: false,
                                open_accessory: false,
                                arena_si_skill_unique_check: false, // true,
                                open_v98: false
                            },
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break
                    }
                }
                break
            }
            case "museum": {
                if (request.action === "info") {
                    
                }
                break
            }

            case "achievement": {
                switch (request.action) {
                    case "unaccomplishList": {
                        let r = []
                        for (let i = 1; i < 10; i++) {
                            if (i == 2) continue
                            r.push({
                                filter_category_id: i,
                                achievement_list: [],
                                count: 0,
                                is_last: true
                            })
                        }
                        resp_data.push({
                            result: r,
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break
                    }
                }
                break
            }

            case "live": {
                switch (request.action) {
                    case "liveStatus": {
                        resp_data.push({
                            result: userLiveStatus,
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break    
                    }
                    case "schedule": {
                        break
                    }
                }
                break
            }
            
            case "unit": {
                switch (request.action) {
                    case "unitAll": {
                        resp_data.push({
                            result: userUnitData,
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break
                    }
                    case "deckInfo": {
                        resp_data.push({
                            result: userDeckData,
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break
                    }
                    case "supporterAll": {
                        resp_data.push({
                            result: {
                                unit_support_list: []
                            },
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break
                    }
                    case "removableSkillInfo": {
                        break
                    }
                    case "accessoryAll": {
                        resp_data.push({
                            result: {
                                accessory_list: [],
                                wearing_info: [],
                                especial_create_flag: false
                            },
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break
                    }
                }
                break
            }

            case "costume": {
                if (request.action == "costumeList") {
                    resp_data.push({
                        result: {
                            costume_list: []
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }

            case "album": {
                if (request.action == "albumAll") {
                    
                }
                break
            }

            case "scenario": {
                if (request.action == "scenarioStatus") {
                    let scenario_status_list = []
                    for (s of game_consts.scenarioList) {
                        scenario_status_list.push({
                            "scenario_id": s.scenario_id,
                            "status": 1
                        })
                    }
                    resp_data.push({
                        result: {
                            "scenario_status_list": scenario_status_list
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }

            case "subscenario": {
                if (request.action == "subscenarioStatus") {
                    let subscenario_status_list = []
                    for (id of game_consts.subscenarioList) {
                        subscenario_status_list.push({
                            "subscenario_id": id,
                            "status": 1
                        })
                    }
                    resp_data.push({
                        result: {
                            "subscenario_status_list": subscenario_status_list,
                            "unlocked_subscenario_ids": []
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }

            case "eventscenario": {
                if (request.action == "status") {

                }
                break
            }

            case "multiunit": {
                if (request.action == "multiunitscenarioStatus") {
                    let multi_unit_scenario_status_list = []
                    for (story of game_consts.multiUnitScenarioList) {
                        multi_unit_scenario_status_list.push({
                            "multi_unit_id": story.multi_unit_id,
                            "status": 1,
                            "multi_unit_scenario_btn_asset": story.multi_unit_scenario_btn_asset,
                            "open_date": story.open_date,
                            "chapter_list": [
                                {
                                    "multi_unit_scenario_id": game_consts.multiUnitScenarioMap[story.multi_unit_id].multi_unit_scenario_id,
                                    "chapter": 1,
                                    "status": 1
                                }
                            ]
                        })
                    }
                    resp_data.push({
                        result: {
                            "multi_unit_scenario_status_list": multi_unit_scenario_status_list,
                            "unlocked_multi_unit_scenario_ids": []
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }
            
            case "payment": {
                if (request.action == "productList") {
                    resp_data.push({
                        result: {
                            restriction_info: {
                                restricted: false
                            },
                            under_age_info: {
                                birth_set: true,
                                has_limit: false,
                                limit_amount: null,
                                month_used: 0
                            },
                            sns_product_list: [],
                            product_list: [],
                            subscription_list: [],
                            show_point_shop: true
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }

            case "secretbox": {
                switch (request.action) {
                    case "all": {
                        resp_data.push({
                            result: {
                                use_cache: 0, // 1
                                is_unit_max: true,
                                item_list: (() => {
                                    let ret = []
                                    for (id of game_consts.gachaItemIds) {
                                        ret.push({ item_id: id, amount: 69 })
                                    }
                                    return ret
                                })(),
                                gauge_info: {
                                    max_gauge_point: 100,
                                    gauge_point: 0
                                },
                                member_category_list: game_consts.gachaPools.member_category_list
                            },
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break
                    }
                }
                break
            }

            case "banner": {
                if (request.action == "bannerList") {

                }
                break
            }

            case "notice": {
                if (request.action == "noticeMarquee") {
                    resp_data.push({
                        result: {
                            item_count: 0,
                            marquee_list: []
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }

            case "user": {
                if (request.action == "getNavi") {
                    resp_data.push({
                        result: {
                            user: {
                                user_id: userData.user.user_id,
                                unit_owning_user_id: account.extraUser.navi_id
                            }
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                } else if (request.action == "userInfo") {
                    resp_data.push({
                        result: userData,
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }
            
            case "navigation": {
                if (request.action == "specialCutin") {
                    resp_data.push({
                        result: {
                            special_cutin_list: []
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }
            
            case "award": {
                if (request.action == "awardInfo") {
                    let awards = []
                    for (var batch = 0; batch < game_consts.availableAwardRanges.length; batch++) {
                        for (var i = game_consts.availableAwardRanges[batch][0]; i <= game_consts.availableAwardRanges[batch][1]; i++) {
                            if (game_consts.availableAwardBlacklists[batch].indexOf(i) > -1) continue
                            awards.push({
                                award_id: i,
                                is_set: (account.extraUser.award_id == i),
                                insert_date: "2018-05-01 19:37:44"
                            })
                        }
                    }

                    resp_data.push({
                        result: {
                            award_info: awards
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }

            case "background": {
                if (request.action == "backgroundInfo") {
                    let bgInfo = []
                    for (var i = 1; i < 162; i++) {
                        bgInfo.push({
                            "background_id": i,
                            "is_set": (account.extraUser.bg_id == i),
                            "insert_date": "2019-04-13 15:04:41"
                        })
                    }

                    let r = { background_info: bgInfo }
                    resp_data.push({
                        result: r,
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }
            
            case "stamp": {
                if (request.action == "stampInfo") {
                }
                break
            }
            
            case "exchange": {
                if (request.action == "owningPoint") {
                    resp_data.push({
                        result: {
                            exchange_point_list: [
                                {
                                    rarity: 2,
                                    exchange_point: 2024
                                },
                                {
                                    rarity: 3,
                                    exchange_point: 167
                                },
                                {
                                    rarity: 5,
                                    exchange_point: 62
                                },
                                {
                                    rarity: 10000,
                                    exchange_point: 5038
                                },
                                {
                                    rarity: 20002,
                                    exchange_point: 10
                                }
                            ]
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }

            case "livese": {
                if (request.action == "liveseInfo") {
                    resp_data.push({
                        result: {
                            live_se_list: [1, 2, 3]
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }

            case "liveicon": {
                if (request.action == "liveiconInfo") {
                    resp_data.push({
                        result: {
                            live_notes_icon_list: [1, 2, 3]
                        },
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }

            case "item": {
                if (request.action == "list") {

                }
                break
            }

            case "marathon": {
                if (request.action == "marathonInfo") {
                    resp_data.push({
                        result: [],
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }
            
            case "challenge": {
                if (request.action == "challengeInfo") {
                    resp_data.push({
                        result: [],
                        status: 200,
                        commandNum: false,
                        timeStamp: parseInt(time)
                    })
                }
                break
            }

            case "reward": {
                resp_data.push({
                    result: { item_count:0, limit:20, order:0, items:[], history:[] },
                    status: 200,
                    commandNum: false,
                    timeStamp: parseInt(time)
                })
                break
            }

            case "profile": {
                switch (request.action) {
                    case "liveCnt": {
                        let ret = {
                            1: 0,
                            2: 0,
                            3: 0,
                            4: 0,
                            5: 0,
                            6: 0
                        }
                        let difficulty_ids = []
                        let lists = [
                            userLiveStatus.normal_live_status_list,
                            userLiveStatus.special_live_status_list,
                            userLiveStatus.training_live_status_list
                        ]
                        for (list of lists) {
                            for (data of list) {
                                if (data.clear_cnt > 0 && difficulty_ids.indexOf(data.live_difficulty_id) == -1) {
                                    let chart = game_consts.specialChartMap[data.live_difficulty_id]
                                    if (chart == undefined) {
                                        chart = game_consts.normalChartMap[data.live_difficulty_id]
                                    }
                                    if (chart == undefined) {
                                        chart = data.live_difficulty_id
                                    }
                                    ret[game_consts.chartRankMap[chart].difficulty] += 1
                                    difficulty_ids.push(data.live_difficulty_id)
                                }
                            }
                        }
                        resp_data.push({
                            result: [
                                {
                                    "difficulty": 1,
                                    "clear_cnt": ret[1]
                                },
                                {
                                    "difficulty": 2,
                                    "clear_cnt": ret[2]
                                },
                                {
                                    "difficulty": 3,
                                    "clear_cnt": ret[3]
                                },
                                {
                                    "difficulty": 4,
                                    "clear_cnt": ret[4]
                                },
                                {
                                    "difficulty": 6,
                                    "clear_cnt": ret[6]
                                }
                            ],
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break
                    }
                    case "cardRanking": {
                        resp_data.push({
                            result: [
                                {
                                    "unit_id": 19,
                                    "total_love": 9999,
                                    "rank": 2,
                                    "sign_flag": false
                                },
                                {
                                    "unit_id": 8,
                                    "total_love": 9999,
                                    "rank": 2,
                                    "sign_flag": false
                                },
                                {
                                    "unit_id": 20,
                                    "total_love": 9999,
                                    "rank": 2,
                                    "sign_flag": false
                                },
                                {
                                    "unit_id": 24,
                                    "total_love": 9999,
                                    "rank": 2,
                                    "sign_flag": false
                                },
                                {
                                    "unit_id": 13,
                                    "total_love": 9999,
                                    "rank": 2,
                                    "sign_flag": false
                                },
                                {
                                    "unit_id": 1997,
                                    "total_love": 9999,
                                    "rank": 2,
                                    "sign_flag": false
                                },
                                {
                                    "unit_id": 21,
                                    "total_love": 9999,
                                    "rank": 2,
                                    "sign_flag": false
                                },
                                {
                                    "unit_id": 9,
                                    "total_love": 9999,
                                    "rank": 2,
                                    "sign_flag": false
                                },
                                {
                                    "unit_id": 23,
                                    "total_love": 9999,
                                    "rank": 2,
                                    "sign_flag": false
                                }
                            ],
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break
                    }
                    case "profileInfo": {
                        let unit_info = account.getUnit(account.extraUser.navi_id)
                        unit_info.removable_skill_ids = [] // todo: fill data in here
                        unit_info.setting_award_id = (account.extraUser.award_id == 10000 ? (account.extraUser.award_id + account.extraUser.rank) : account.extraUser.award_id)
                        resp_data.push({
                            result: {
                                user_info: {
                                    user_id: userData.user.user_id,
                                    name: userData.user.name,
                                    level: 6969,
                                    cost_max: 9999,
                                    unit_max: 9999,
                                    energy_max: 9999,
                                    friend_max: 9999,
                                    unit_cnt: 9999,
                                    invite_code: userData.user.user_id.toString(),
                                    elapsed_time_from_login: "030?",
                                    introduction: account.extraUser.bio
                                },
                                center_unit_info: unit_info,
                                navi_unit_info: unit_info,
                                is_alliance: false,
                                friend_status: 0,
                                setting_award_id: (account.extraUser.award_id == 10000 ? (account.extraUser.award_id + account.extraUser.rank) : account.extraUser.award_id),
                                setting_background_id: 7
                            },
                            status: 200,
                            commandNum: false,
                            timeStamp: parseInt(time)
                        })
                        break
                    }
                }
                break
            }

            default: {
                console.log("-------------- unhandled main api!", request.module, request.action)
                resp_data.push({
                    result: {},
                    status: 200,
                    commandNum: false,
                    timeStamp: parseInt(time)
                })
            }
        }
        console.log(resp_data.length != dlen ? 'handled' : 'unhandled', request.module + '.' + request.action)
    
    })
    
    full_resp_body = {
        response_data: resp_data,
        release_info: rel_info,
        status_code: 200
    }
    let toSend = JSON.stringify(full_resp_body).replaceAll('1680254953', time);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

module.exports = {api, game_consts};
