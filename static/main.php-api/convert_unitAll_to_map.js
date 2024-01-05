const fs = require('fs')
let src = JSON.parse(fs.readFileSync('unit.unitAll.result.json', 'utf8'))
let ret = {
	active: {},
	waiting: {}
}

function getCardEntry(id) {
        return {
            unit_owning_user_id: id,
            unit_id: 1,
            exp: 9999,
            next_exp: 9999,
            level: 40,
            max_level: 40,
            level_limit_id: 0,
            rank: 2,
            max_rank: 2,
            love: 50,
            max_love: 50,
            unit_skill_exp: 99999,
            unit_skill_level: 1,
            max_hp: 3,
            unit_removable_skill_capacity: 1,
            favorite_flag: false,
            display_rank: 1,
            is_rank_max: true,
            is_love_max: true,
            is_level_max: true,
            is_signed: false,
            is_skill_level_max: true,
            is_removable_skill_capacity_max: true,
            insert_date: "2023-02-02 01:41:10"
        }
}

for (card of src.active) {
    ret.active[card.unit_owning_user_id] = card
}
for (card of src.waiting) {
    ret.waiting[card.unit_owning_user_id] = card
}

fs.writeFileSync('../../data/unitAll.json', JSON.stringify(ret))
console.log('done')