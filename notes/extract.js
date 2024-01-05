const fs = require('fs')
const src = JSON.parse(fs.readFileSync('live_notes.json', 'utf8'))

for (chart of src) {
    console.log("Saving", chart.notes_setting_asset)
    fs.writeFileSync(chart.notes_setting_asset, chart.json)
}
