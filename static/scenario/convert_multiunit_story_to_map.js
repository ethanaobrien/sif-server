const fs = require('fs')

const source = JSON.parse(fs.readFileSync('multi_unit_scenario_m.json'))
let ret = {}

for (data of source) {
	ret[data.multi_unit_id] = data
}

fs.writeFileSync('multi_unit_scenario_map.json', JSON.stringify(ret))