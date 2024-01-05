const {consumeBody, signResp, rel_info, parseBody, timestamp} = require("./global.js");
const account = require("./account.js");

async function partyList(req, res) {
    let list = [];
    //const body = parseBody((await consumeBody(req)).toString());
    const token = req.headers.authorize.split("token=").pop().split("&")[0];
    const userDeckData = account.getDeckData(token);
	for (deck of userDeckData) {
		for (card of deck.unit_owning_user_ids) {
			if (card.position === 5) {
				list.push({
					user_info: {
						user_id: (158669000 + list.length),
						name: "030",
						level: 69
					},
					center_unit_info: {
						unit_owning_user_id: card.unit_owning_user_id,
						unit_id: account.getUnit(0, card.unit_owning_user_id).unit_id, // ?
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
				})
				break
			}
		}
	}
    console.log(list);
    
    const toSend = {
        "response_data": {
            "party_list": list,
            "training_energy": 69,
            "training_energy_max": 69,
            "server_timestamp": timestamp()
        },
        "release_info": rel_info,
        "status_code": 200
    }
    const data = JSON.stringify(toSend);
    signResp(req, res, data);
    res.send(data);
    res.end();
    
}

async function play(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    const token = req.headers.authorize.split("token=").pop().split("&")[0];
    const acc = account.getInfo(token);
    const userDeckData = account.getDeckData(token);
	let chartFile = require("./api.js").game_consts.specialChartMap[body.live_difficulty_id]
	if (chartFile == undefined) {
		chartFile = require("./api.js").game_consts.normalChartMap[body.live_difficulty_id]
	}
	if (chartFile == undefined) {
		chartFile = body.live_difficulty_id
	}

	let chartPath = '../notes/' + chartFile
	console.log("playing", chartPath)
	let chart = require(chartPath);
    let deckId = body.unit_deck_id;
    account.setCurrentDeckId(token, deckId);
	let deck = userDeckData.filter((x) => { return x.unit_deck_id == body.unit_deck_id })[0]
	let cMember = deck.unit_owning_user_ids.filter((x) => { return x.position == 5 })[0]
	let score_rank = require("./api.js").game_consts.chartRankMap[chartFile].score_rank
	let rank_info = []
	for (let i = 0; i < (score_rank.length + 1); i++) {
		rank_info.push({
			rank: (5 - i),
			rank_min: (i == 0) ? 0 : score_rank[4 - i],
			rank_max: (i >= 4) ? 0 : (score_rank[3 - i] - 1)
		})
	}
	let unit_list = []
	let total_hp = 0
	let total_power_smile = 0
	let total_power_pure = 0
	let total_power_cool = 0
	let units = deck.unit_owning_user_ids
	let multiplier = 1.22
	let centerUnitId = account.getUnit(acc.user.user_id, cMember.unit_owning_user_id).unit_id
	let centerUnitData = require("./api.js").game_consts.units[centerUnitId]
	if (centerUnitData.center_skill != undefined) {
		multiplier = (multiplier * (1.0 + (centerUnitData.center_skill.effect_value / 100)))
	}
	if (centerUnitData.center_skill_extra != undefined) {
		multiplier = (multiplier * (1.0 + (centerUnitData.center_skill_extra.effect_value / 100)))
	}
	console.log("calculated multiplier:", multiplier)
	for (let i = 0; i < 9; i++) {
		let unitId = account.getUnit(acc.user.user_id, units[i].unit_owning_user_id).unit_id
		let unitData = require("./api.js").game_consts.units[unitId].unit
		let love = unitData.after_love_max
		let power_smile = Math.ceil((unitData.smile_max + love) * multiplier)
		let power_pure = Math.ceil((unitData.pure_max + love) * multiplier)
		let power_cool = Math.ceil((unitData.cool_max + love) * multiplier)
		total_power_smile += power_smile
		total_power_pure += power_pure
		total_power_cool += power_cool
		unit_list.push({
			smile: power_smile,
			cute: power_pure,
			cool: power_cool
		})
	}
	let toSend = {
		"response_data": {
			"rank_info": rank_info,
			"energy_full_time": "2023-02-02 22:28:03",
			"over_max_energy": 6969,
			"available_live_resume": false,
			"live_list": [
				{
					"live_info": {
						"live_difficulty_id": parseInt(body.live_difficulty_id),
						"is_random": false,
						"ac_flag": require("./api.js").game_consts.chartRankMap[chartFile].ac_flag,
						"swing_flag": require("./api.js").game_consts.chartRankMap[chartFile].swing_flag,
						"notes_list": chart
					},
					"deck_info": {
						"unit_deck_id": deckId,
						"total_smile": total_power_smile,
						"total_cute": total_power_pure,
						"total_cool": total_power_cool,
						"total_hp": 69,
						"prepared_hp_damage": 0,
						"unit_list": unit_list
					}
				}
			],
			"is_marathon_event": false,
			"marathon_event_id": null,
			"no_skill": false,
			"can_activate_effect": true,
			"server_timestamp": timestamp()
		},
		"release_info": rel_info,
		"status_code": 200
	}
    const data = JSON.stringify(toSend);
    signResp(req, res, data);
    res.send(data);
    res.end();
}

async function gameOver(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    console.log(body);

    let toSend = {
        response_data: [],
        release_info: rel_info,
        status_code: 200
    }
    const data = JSON.stringify(toSend);
    signResp(req, res, data);
    res.send(data);
    res.end();
}

async function preciseScore(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    console.log(body);

    let toSend = {
        response_data: [],
        release_info: rel_info,
        status_code: 200
    }
    const data = JSON.stringify(toSend);
    signResp(req, res, data);
    res.send(data);
    res.end();
}

async function reward(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    console.log(body);
    const token = req.headers.authorize.split("token=").pop().split("&")[0];
    const acc = account.getInfo(token);
    const userDeckData = account.getDeckData(token);
	let chartFile = require("./api.js").game_consts.specialChartMap[body.live_difficulty_id]
	if (chartFile == undefined) {
		chartFile = require("./api.js").game_consts.normalChartMap[body.live_difficulty_id]
	}
    
    let deckId = account.getCurrentDeckId(token);
	let score = body.score_cool + body.score_cute + body.score_smile
	console.log("live.reward", "is_training:", body.is_training, "id:", body.live_difficulty_id, "score:", score, "combo:", body.max_combo)
	let deckInfo = userDeckData
	let unitUids = []
	for (deck of deckInfo) {
		if (deck.unit_deck_id == deckId) {
			for (card of deck.unit_owning_user_ids) {
				unitUids.push(card.unit_owning_user_id)
			}
			break
		}
	}
	// rank: 1 = S, 2 = A, 3 = B, 4 = C
	let comboRank = 1
	let scoreRank = 1
	let rankMap = require("./api.js").game_consts.chartRankMap[chartFile]
	console.log("score info: ", score, rankMap.score_rank)
	for (var i = 0; i < rankMap.combo_rank.length; i++) {
		if (body.max_combo >= rankMap.combo_rank[i]) {
			comboRank = (i + 1)
			break
		}
		if (i == (rankMap.combo_rank.length - 1))
			comboRank = 5
	}
	for (var i = 0; i < rankMap.score_rank.length; i++) {
		if (score >= rankMap.score_rank[i]) {
			scoreRank = (i + 1)
			break
		}
		if (i == (rankMap.score_rank.length - 1))
			scoreRank = 5
	}
	let saveResult = account.updateLiveStatus(body.is_training ? 3 : 1, body.live_difficulty_id, rankMap.difficulty, score, body.max_combo, {score: scoreRank, combo: comboRank}, rankMap)
	console.log("save result:", saveResult);
	let liveUserInfo = acc.userInfo.user
	liveUserInfo.current_energy = 6969
	let unit_list = []
	for (let i = 0; i < 9; i++) {
		let unit = account.getUnit(liveUserInfo.user_id, unitUids[i])
		unit_list.push({
			unit_owning_user_id: unitUids[i],
			unit_id: account.getUnitId(liveUserInfo.user_id, unitUids[i]).unit_id,
			position: 1 + i,
			level: unit.level,
			level_limit_id: unit.level_limit_id,
			display_rank: unit.rank,
			love: unit.love,
			unit_skill_level: unit.unit_skill_level,
			is_rank_max: unit.is_rank_max,
			is_love_max: unit.is_love_max,
			is_level_max: unit.is_level_max,
			is_signed: unit.is_signed,
			before_love: unit.love,
			max_love: unit.max_love

		})
	}
	let toSend = {
		response_data: {
			live_info: [
				{
					live_difficulty_id: body.live_difficulty_id,
					is_random: false,
					ac_flag: require("./api.js").game_consts.chartRankMap[currentChartFile].ac_flag,
					swing_flag: require("./api.js").game_consts.chartRankMap[currentChartFile].swing_flag
				}
			],
			rank: scoreRank,
			combo_rank: comboRank,
			total_love: 9999,
			is_high_score: saveResult.new_record,
			hi_score: saveResult.hi_score,
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
			before_user_info: liveUserInfo,
			after_user_info: liveUserInfo,
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
			museum_info: game_consts.museum_info_txt,
			server_timestamp: timestamp(),
			present_cnt: 0
		},
		release_info: rel_info,
		status_code: 200
	}
    const data = JSON.stringify(toSend);
    signResp(req, res, data);
    res.send(data);
    res.end();
}

module.exports = {partyList, play, gameOver, preciseScore, reward};
