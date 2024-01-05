const fs = require('fs')

const secretboxes = JSON.parse(fs.readFileSync("secretbox.all.result.json")).member_category_list
var slimbox = {}

for (group of secretboxes) {
    let boxes = []
    for (box of group.page_list) {
	if (box.secret_box_info.secret_box_type != 0) continue
        boxes.push({
			"box_id": box.secret_box_info.secret_box_id,
			"name": box.secret_box_info.name,
			"menu_asset": box.menu_asset,
			"animation_assets": box.animation_assets
		})
    }
    slimbox[group.member_category] = boxes
}

fs.writeFileSync("slimbox.json", JSON.stringify(slimbox))
