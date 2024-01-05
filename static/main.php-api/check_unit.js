const fs = require('fs')

let unit_all = JSON.parse(fs.readFileSync('unit.unitAll.result.json', 'utf8'))
let deck = JSON.parse(fs.readFileSync('unit.deckInfo.result.json', 'utf8'))
let equip = JSON.parse(fs.readFileSync('unit.removableSkillInfo.result.json', 'utf8'))

let ids = []
for (card of unit_all.active) {
	ids.push(card.unit_owning_user_id)
}

for (d of deck) {
	console.log('checking deck', d.deck_name)
	for (card of d.unit_owning_user_ids) {
		if (ids.indexOf(card.unit_owning_user_id) == -1) {
			console.log('card on deck not found in collection:', card.unit_owning_user_id)
		}
	}
}

let index = 0
let verified_eqp = {
	owning_info: {},
	equipment_info: {}
}
for (info of [equip.owning_info, equip.equipment_info]) {
	console.log('checking equipment info')
	for (card of Object.entries(info)) {
		if (ids.indexOf(parseInt(card[0])) == -1) {
			console.log('card in equipment info not found in collection:', card[0])
		} else if (index == 0) {
			verified_eqp.owning_info[card[0]] = info[card[0]]
		} else if (index == 1) {
			verified_eqp.equipment_info[card[0]] = info[card[0]]
		}
	}
	++index
}

fs.writeFileSync('unit.removableSkillInfo.result.verified.json', JSON.stringify(verified_eqp))
