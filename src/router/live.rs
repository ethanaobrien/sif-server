use base64::{Engine as _, engine::general_purpose};
use crate::router::{global, userdata};
use json::{object, array};
use actix_web::{HttpResponse, HttpRequest, http::header::HeaderValue};

pub fn party_list(req: HttpRequest, _body: String) -> HttpResponse {
    //let blank_header = HeaderValue::from_static("");
    //let decoded = general_purpose::STANDARD.decode(req.headers().get("authorize").unwrap_or(&blank_header).to_str().unwrap_or("").split("token=").collect::<Vec<_>>().pop().unwrap().split("&").collect::<Vec<_>>()[0]).unwrap();
    //let key = String::from_utf8_lossy(&decoded);
    //let userdata = userdata::get_acc(&key);
    //println!("{}", body);
    
    //todo
    let random_user = object!{
        user_info: {
            user_id: 696969696,
            name: "Hello",
            level: 2
        },
        center_unit_info: {
            unit_owning_user_id: 460500692,
            unit_id: 1997, // ?
            exp: 26660,
            next_exp: 26908,
            level: 100,
            level_limit_id: 1,
            max_level: 100,
            rank: 2,
            max_rank: 2,
            love: 1000,
            max_love: 1000,
            unit_skill_level: 1,
            max_hp: 5,
            favorite_flag: false,
            display_rank: 2,
            unit_skill_exp: 0,
            unit_removable_skill_capacity: 4,
            attribute: 2,
            smile: 3700,
            cute: 4460,
            cool: 3170,
            is_love_max: true,
            is_level_max: true,
            is_rank_max: true,
            is_signed: false,
            is_skill_level_max: true,
            setting_award_id: 1,
            removable_skill_ids: []
        },
        setting_award_id: 10011,
        available_social_point: 10,
        friend_status: 1
    };
    
    let resp = object!{
        "response_data": {
            "party_list": [random_user],
            "training_energy": 69,
            "training_energy_max": 69,
            "server_timestamp": global::timestamp()
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn play(req: HttpRequest, body: String) -> HttpResponse {
    let blank_header = HeaderValue::from_static("");
    let decoded = general_purpose::STANDARD.decode(req.headers().get("authorize").unwrap_or(&blank_header).to_str().unwrap_or("").split("token=").collect::<Vec<_>>().pop().unwrap().split("&").collect::<Vec<_>>()[0]).unwrap();
    let key = String::from_utf8_lossy(&decoded);
    let userdata = userdata::get_acc(&key);
    let body = global::process_body(body);
    println!("{}", body);
    
    let special_map = json::parse(include_str!("../../assets/special_live_map.json")).unwrap();
    let mut map = special_map[body["live_difficulty_id"].to_string()].clone();
    if map.is_null() {
        let normal_map = json::parse(include_str!("../../assets/normal_live_map.json")).unwrap();
        map = normal_map[body["live_difficulty_id"].to_string()].clone();
        if map.is_null() {
            map = body["live_difficulty_id"].clone();
        }
    }
    let path = format!("notes/{}", map.to_string());
    let file = path.split("/").last().unwrap();
    let chart = json::parse(&std::fs::read_to_string(path.clone()).unwrap()).unwrap();
    
    
    let charts = &json::parse(include_str!("../../assets/live_setting_map.json")).unwrap()[file];
    let all_units = json::parse(include_str!("../../assets/all_units.json")).unwrap();
    
    let current_deck = userdata["current_deck"].as_usize().unwrap_or(1);
    let mut deck = userdata["deck_info"][0].clone();
    for (_i, data) in userdata["deck_info"].members().enumerate() {
        if data["unit_deck_id"].as_usize().unwrap() == current_deck {
            deck = data.clone();
            break;
        }
    }
    let mut center = deck["unit_owning_user_ids"][0].clone();
    for (_i, data) in deck["unit_owning_user_ids"].members().enumerate() {
        if data["position"].as_usize().unwrap() == 5 {
            center = data.clone();
            break;
        }
    }
    let mut rank_info = array![];
    //println!("{}", charts);
    for i in 0..(charts["score_rank"].len()+1) {
        let rank_min = if i == 0 {
            0
        } else {
            charts["score_rank"][4 - i].as_usize().unwrap_or(0)
        };
        let rank_max = if i > 4 {
            charts["score_rank"][3 - i].as_usize().unwrap_or(0) - 1
        } else {
            0
        };
        rank_info.push(object!{
            rank: (5 - i),
            rank_min: rank_min,
            rank_max: rank_max
        }).unwrap();
    }
    let mut unit_list = array![];
    let mut total_power_smile = 0.0;
    let mut total_power_pure = 0.0;
    let mut total_power_cool = 0.0;
    let units = &deck["unit_owning_user_ids"];
    let mut multiplier = 1.22;
    //println!("{}", center);
    let c = center["unit_owning_user_id"].to_string();
    let center_unit_id = userdata["unit_all"]["active"][c]["unit_id"].as_usize().unwrap();
    let center_unit_data = &all_units[center_unit_id];
    if !center_unit_data["center_skill"].is_null() {
        multiplier = multiplier * (1.0 + (center_unit_data["center_skill"]["effect_value"].as_f32().unwrap() / 100.0));
    }
    if !center_unit_data["center_skill_extra"].is_null() {
       multiplier = multiplier * (1.0 + (center_unit_data["center_skill_extra"]["effect_value"].as_f32().unwrap() / 100.0));
    }
    println!("calculated multiplier: {}", multiplier);
    for i in 0..9 {
        let unit_id = userdata["unit_all"]["active"][units[i]["unit_owning_user_id"].to_string()]["unit_id"].to_string();
        let unit_data = &all_units[unit_id]["unit"];
        let love = unit_data["after_love_max"].as_f32().unwrap();
        let power_smile = ((unit_data["smile_max"].as_f32().unwrap() + love) * multiplier).ceil();
        let power_pure = ((unit_data["pure_max"].as_f32().unwrap() + love) * multiplier).ceil();
        let power_cool = ((unit_data["cool_max"].as_f32().unwrap() + love) * multiplier).ceil();
        total_power_smile += power_smile;
        total_power_pure += power_pure;
        total_power_cool += power_cool;
        unit_list.push(object!{
            smile: power_smile,
            cute: power_pure,
            cool: power_cool
        }).unwrap();
    }
    
    let resp = object!{
        "response_data": {
			"rank_info": rank_info,
			"energy_full_time": global::timestamp_str(),
			"over_max_energy": 6969,
			"available_live_resume": false,
			"live_list": [
				{
					"live_info": {
						"live_difficulty_id": body["live_difficulty_id"].to_string().parse::<i32>().unwrap(),
						"is_random": false,
						"ac_flag": charts["ac_flag"].clone(),
						"swing_flag": charts["swing_flag"].clone(),
						"notes_list": chart.clone()
					},
					"deck_info": {
						"unit_deck_id": current_deck,
						"total_smile": total_power_smile,
						"total_cute": total_power_pure,
						"total_cool": total_power_cool,
						"total_hp": 200,
						"prepared_hp_damage": 0,
						"unit_list": unit_list.clone()
					}
				}
			],
			"is_marathon_event": false,
			"marathon_event_id": null,
			"no_skill": false,
			"can_activate_effect": true,
			"server_timestamp": global::timestamp()
		},
        "release_info": global::release_info(),
        "status_code":200
    };
    
    let body = json::stringify(resp);
    //println!("{}", body);
    global::sign(&req, &body).body(body)
}

pub fn precise_score(req: HttpRequest, body: String) -> HttpResponse {
    println!("{}", body);
    
    //todo
    let resp = object!{
        "response_data": {
            error_code: 3421
        },
        "status_code":600
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn reward(req: HttpRequest, body: String) -> HttpResponse {
    let blank_header = HeaderValue::from_static("");
    let decoded = general_purpose::STANDARD.decode(req.headers().get("authorize").unwrap_or(&blank_header).to_str().unwrap_or("").split("token=").collect::<Vec<_>>().pop().unwrap().split("&").collect::<Vec<_>>()[0]).unwrap();
    let key = String::from_utf8_lossy(&decoded);
    let userdata = userdata::get_acc(&key);
    let body = global::process_body(body);
    
    let score = body["score_cool"].as_f32().unwrap() + body["score_cute"].as_f32().unwrap() + body["score_smile"].as_f32().unwrap();
	//console.log("live.reward", "is_training:", body.is_training, "id:", body.live_difficulty_id, "score:", score, "combo:", body.max_combo)
    let current_deck = userdata["current_deck"].as_usize().unwrap_or(1);
    let mut deck = userdata["deck_info"][0].clone();
    for (_i, data) in userdata["deck_info"].members().enumerate() {
        if data["unit_deck_id"].as_usize().unwrap() == current_deck {
            deck = data.clone();
            break;
        }
    }
    
    let special_map = json::parse(include_str!("../../assets/special_live_map.json")).unwrap();
    let mut map = special_map[body["live_difficulty_id"].to_string()].clone();
    if map.is_null() {
        let normal_map = json::parse(include_str!("../../assets/normal_live_map.json")).unwrap();
        map = normal_map[body["live_difficulty_id"].to_string()].clone();
        if map.is_null() {
            map = body["live_difficulty_id"].clone();
        }
    }
    
    let path = format!("notes/{}", map.to_string());
    let file = path.split("/").last().unwrap();
    
    let charts = &json::parse(include_str!("../../assets/live_setting_map.json")).unwrap()[file];
    
	// rank: 1 = S, 2 = A, 3 = B, 4 = C
	let mut combo_rank = 1;
	let mut score_rank = 1;
	//console.log("score info: ", score, map.score_rank)
    for i in 0..map["combo_rank"].len() {
        if body["max_combo"].as_usize().unwrap() >= map["combo_rank"][i].as_usize().unwrap() {
            combo_rank = i + 1;
            break;
        }
        if i == map["combo_rank"].len() - 1 {
            combo_rank = 5;
        }
    }
    for i in 0..map["score_rank"].len() {
        if score >= map["combo_rank"][i].as_f32().unwrap() {
            score_rank = i + 1;
            break;
        }
        if i == map["score_rank"].len() - 1 {
            score_rank = 5;
        }
    }
    
	//let saveResult = common_data.updateLiveStatus(body.is_training ? 3 : 1, body.live_difficulty_id, map.difficulty, score, body.max_combo, {score: scoreRank, combo: comboRank}, map)
	//console.log("save result:", saveResult)
	//common_data.saveLiveStatus()
    let save_result = object!{success: false, new_record: false, hi_score: 0}; //todo
    
	let mut unit_list = array![];
    for (i, data) in deck["unit_owning_user_ids"].members().enumerate() {
        let unit = userdata["unit_all"]["active"][data["unit_owning_user_id"].to_string()].clone();
		unit_list.push(object!{
            unit_owning_user_id: unit["unit_owning_user_id"].clone(),
			unit_id: unit["unit_id"].clone(),
			position: 1 + i,
			level: unit["level"].clone(),
			level_limit_id: unit["level_limit_id"].clone(),
			display_rank: unit["rank"].clone(),
			love: unit["love"].clone(),
			unit_skill_level: unit["unit_skill_level"].clone(),
			is_rank_max: unit["is_rank_max"].clone(),
			is_love_max: unit["is_love_max"].clone(),
			is_level_max: unit["is_level_max"].clone(),
			is_signed: unit["is_signed"].clone(),
			before_love: unit["love"].clone(),
			max_love: unit["max_love"].clone(),
		}).unwrap();
    }
	let resp = object!{
		response_data: {
			live_info: [
				{
					live_difficulty_id: body["live_difficulty_id"].clone(),
					is_random: false,
					ac_flag: charts["ac_flag"].clone(),
					swing_flag: charts["swing_flag"].clone(),
				}
			],
			rank: score_rank,
			combo_rank: combo_rank,
			total_love: 9999,
			is_high_score: save_result["new_record"].clone(),
			hi_score: save_result["hi_score"].clone(),
			base_reward_info: {
				player_exp: 999999,
				player_exp_unit_max: {
					before: 999999,
					after: 999999
				},
				player_exp_friend_max: {
					before: 999999,
					after: 999999
				},
				player_exp_lp_max: {
					before: 999999,
					after: 999999
				},
				game_coin: 999999,
				game_coin_reward_box_flag: false,
				social_point: 6969
			},
			reward_unit_list: {
				live_clear: [],
				live_rank: [],
				live_combo: []
			},
			unlocked_subscenario_ids: [],
			unlocked_multi_unit_scenario_ids: [],
			effort_point: [],
			is_effort_point_visible: false,
			limited_effort_box: [],
			unit_list: unit_list,
			before_user_info: userdata["user_info"]["user"].clone(),
			after_user_info: userdata["user_info"]["user"].clone(),
			next_level_info: [],
			goal_accomp_info: {
				achieved_ids: [],
				rewards: []
			},
			special_reward_info: [],
			event_info: [],
			daily_reward_info: [],
			can_send_friend_request: false,
			using_buff_info: [],
			class_system: {
				rank_info: {
					before_class_rank_id: 1,
					after_class_rank_id: 1,
					rank_up_date: "2023-02-02 01:41:13"
				},
				complete_flag: false,
				is_opened: false,
				is_visible: false
			},
			accomplished_achievement_list: [],
			unaccomplished_achievement_cnt: 0,
			added_achievement_list: [],
			new_achievement_cnt: 0,
			museum_info: json::stringify(json::parse(include_str!("../../assets/museum_info.json")).unwrap()),
			server_timestamp: global::timestamp(),
			present_cnt: 0
		},
		release_info: global::release_info(),
		status_code: 200
	};
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn gameover(req: HttpRequest, body: String) -> HttpResponse {
    reward(req, body) //idk I guess this works?
    /*
    //todo
    let resp = object!{
        "response_data": {
            error_code: 3421
        },
        "status_code":600
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)*/
}
