const {consumeBody, signResp, rel_info, parseBody, timestamp} = require("./global.js");
const account = require("./account.js");

async function userInfo(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    //console.log(body, req.headers);
    const acc = account.getUserData(req.headers.authorize.split("token=").pop().split("&")[0]);
    //console.log("user info: ", acc);
    
    const resp = {
        "response_data": acc,
        "release_info": rel_info,
        "status_code": 200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

async function gdpr(req, res) {
    const resp = {
        "response_data": {
            "enable_gdpr": true,
            "is_eea": false,
            "server_timestamp": timestamp()
        },
        "release_info": rel_info,
        "status_code":200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

async function tos(req, res) {
    const resp = {
        "response_data": {
            "tos_id": 8,
            "tos_type": 3,
            "is_agreed": true,
            "server_timestamp": timestamp()
        },
        "release_info": rel_info,
        "status_code":200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

async function changeName(req, res) {
    const body = parseBody((await consumeBody(req)).toString());
    const token = req.headers.authorize.split("token=").pop().split("&")[0];
    const acc = account.getUserData(token);
    
    const resp = {
        "response_data": {
            "before_name": acc.user.name,
            "after_name": body.name,
            "server_timestamp": timestamp()
        },
        "release_info": rel_info,
        "status_code":200
    }
    account.setName(token, body.name);
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

async function tutorial(req, res) {
    const resp = {
        "response_data": [],
        "release_info": rel_info,
        "status_code": 200
    }
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}


//needs to be cleaned up
async function lbonus_execute(req, res) {
    const resp = {
        "response_data":{"sheets":[],"calendar_info":{"current_date":"2023-02-02","current_month":{"year":2023,"month":2,"days":[{"day":1,"day_of_the_week":3,"special_day":true,"special_image_asset":"assets/image/ui/login_bonus/loge_icon_01.png","received":false,"ad_received":false,"item":{"item_id":3,"add_type":3000,"amount":250000}},{"day":2,"day_of_the_week":4,"special_day":false,"special_image_asset":"","received":true,"ad_received":false,"item":{"item_id":2,"add_type":3002,"amount":1000}},{"day":3,"day_of_the_week":5,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3006,"amount":1}},{"day":4,"day_of_the_week":6,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"unit_id":382,"add_type":1001,"amount":1,"is_rank_max":false,"is_signed":false}},{"day":5,"day_of_the_week":0,"special_day":true,"special_image_asset":"assets/image/ui/login_bonus/loge_icon_01.png","received":false,"ad_received":false,"item":{"item_id":4,"add_type":3001,"amount":2}},{"day":6,"day_of_the_week":1,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":3,"add_type":3000,"amount":50000}},{"day":7,"day_of_the_week":2,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3002,"amount":1000}},{"day":8,"day_of_the_week":3,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3006,"amount":1}},{"day":9,"day_of_the_week":4,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"unit_id":382,"add_type":1001,"amount":1,"is_rank_max":false,"is_signed":false}},{"day":10,"day_of_the_week":5,"special_day":true,"special_image_asset":"assets/image/ui/login_bonus/loge_icon_01.png","received":false,"ad_received":false,"item":{"item_id":4,"add_type":3001,"amount":2}},{"day":11,"day_of_the_week":6,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":3,"add_type":3000,"amount":50000}},{"day":12,"day_of_the_week":0,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3002,"amount":1000}},{"day":13,"day_of_the_week":1,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3006,"amount":1}},{"day":14,"day_of_the_week":2,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"unit_id":382,"add_type":1001,"amount":1,"is_rank_max":false,"is_signed":false}},{"day":15,"day_of_the_week":3,"special_day":true,"special_image_asset":"assets/image/ui/login_bonus/loge_icon_01.png","received":false,"ad_received":false,"item":{"item_id":4,"add_type":3001,"amount":2}},{"day":16,"day_of_the_week":4,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":3,"add_type":3000,"amount":50000}},{"day":17,"day_of_the_week":5,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3002,"amount":1000}},{"day":18,"day_of_the_week":6,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3006,"amount":1}},{"day":19,"day_of_the_week":0,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"unit_id":382,"add_type":1001,"amount":1,"is_rank_max":false,"is_signed":false}},{"day":20,"day_of_the_week":1,"special_day":true,"special_image_asset":"assets/image/ui/login_bonus/loge_icon_01.png","received":false,"ad_received":false,"item":{"item_id":4,"add_type":3001,"amount":2}},{"day":21,"day_of_the_week":2,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":3,"add_type":3000,"amount":50000}},{"day":22,"day_of_the_week":3,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3002,"amount":1000}},{"day":23,"day_of_the_week":4,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3006,"amount":1}},{"day":24,"day_of_the_week":5,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"unit_id":382,"add_type":1001,"amount":1,"is_rank_max":false,"is_signed":false}},{"day":25,"day_of_the_week":6,"special_day":true,"special_image_asset":"assets/image/ui/login_bonus/loge_icon_01.png","received":false,"ad_received":false,"item":{"item_id":4,"add_type":3001,"amount":2}},{"day":26,"day_of_the_week":0,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3006,"amount":1}},{"day":27,"day_of_the_week":1,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"unit_id":382,"add_type":1001,"amount":1,"is_rank_max":false,"is_signed":false}},{"day":28,"day_of_the_week":2,"special_day":true,"special_image_asset":"assets/image/ui/login_bonus/loge_icon_01.png","received":false,"ad_received":false,"item":{"item_id":4,"add_type":3001,"amount":2}}]},"next_month":{"year":2023,"month":3,"days":[{"day":1,"day_of_the_week":3,"special_day":true,"special_image_asset":"assets/image/ui/login_bonus/loge_icon_01.png","received":false,"ad_received":false,"item":{"item_id":3,"add_type":3000,"amount":250000}},{"day":2,"day_of_the_week":4,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3002,"amount":1000}},{"day":3,"day_of_the_week":5,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3006,"amount":1}},{"day":4,"day_of_the_week":6,"special_day":true,"special_image_asset":"assets/image/ui/login_bonus/loge_icon_01.png","received":false,"ad_received":false,"item":{"unit_id":382,"add_type":1001,"amount":1,"is_rank_max":false,"is_signed":false}},{"day":5,"day_of_the_week":0,"special_day":true,"special_image_asset":"assets/image/ui/login_bonus/loge_icon_01.png","received":false,"ad_received":false,"item":{"item_id":4,"add_type":3001,"amount":2}},{"day":6,"day_of_the_week":1,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":3,"add_type":3000,"amount":50000}},{"day":7,"day_of_the_week":2,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3002,"amount":1000}},{"day":8,"day_of_the_week":3,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3006,"amount":1}},{"day":9,"day_of_the_week":4,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"unit_id":382,"add_type":1001,"amount":1,"is_rank_max":false,"is_signed":false}},{"day":10,"day_of_the_week":5,"special_day":true,"special_image_asset":"assets/image/ui/login_bonus/loge_icon_01.png","received":false,"ad_received":false,"item":{"item_id":4,"add_type":3001,"amount":2}},{"day":11,"day_of_the_week":6,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":3,"add_type":3000,"amount":50000}},{"day":12,"day_of_the_week":0,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3002,"amount":1000}},{"day":13,"day_of_the_week":1,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"item_id":2,"add_type":3006,"amount":1}},{"day":14,"day_of_the_week":2,"special_day":false,"special_image_asset":"","received":false,"ad_received":false,"item":{"unit_id":382,"add_type":1001,"amount":1,"is_rank_max":false,"is_signed":false}}]}},"ad_info":{"ad_id":1,"term_id":683,"reward_list":[{"item_id":168,"add_type":1000,"amount":1,"item_category_id":0,"reward_box_flag":true},{"item_id":349,"add_type":1000,"amount":1,"item_category_id":0,"reward_box_flag":true}]},"total_login_info":{"login_count":1,"remaining_count":1,"reward":[{"item_id":2,"add_type":3002,"amount":1000}]},"license_lbonus_list":[],"class_system":' + common_data.classSystem + ',"start_dash_sheets":[{"start_dash_lbonus_id":11,"start_dash_lbonus_item_num":10,"bg_asset":"assets/image/start_dash_lbonus/start_dash_lbonus_1.png","show_next_item":false,"items":[{"seq":1,"start_dash_lbonus_item_id":114,"reward":[{"item_id":4,"add_type":3001,"amount":3},{"item_id":1,"add_type":1000,"amount":1}]},{"seq":2,"start_dash_lbonus_item_id":115,"reward":[{"item_id":4,"add_type":3001,"amount":3}]},{"seq":3,"start_dash_lbonus_item_id":116,"reward":[{"item_id":4,"add_type":3001,"amount":3}]},{"seq":4,"start_dash_lbonus_item_id":117,"reward":[{"item_id":4,"add_type":3001,"amount":3}]},{"seq":5,"start_dash_lbonus_item_id":118,"reward":[{"item_id":4,"add_type":3001,"amount":3}]},{"seq":6,"start_dash_lbonus_item_id":119,"reward":[{"item_id":4,"add_type":3001,"amount":3}]},{"seq":7,"start_dash_lbonus_item_id":120,"reward":[{"item_id":4,"add_type":3001,"amount":3}]},{"seq":8,"start_dash_lbonus_item_id":121,"reward":[{"item_id":4,"add_type":3001,"amount":3}]},{"seq":9,"start_dash_lbonus_item_id":122,"reward":[{"item_id":4,"add_type":3001,"amount":3}]},{"seq":10,"start_dash_lbonus_item_id":123,"reward":[{"item_id":4,"add_type":3001,"amount":3}]}],"stamp_num":1}],"effort_point":[{"live_effort_point_box_spec_id":4,"capacity":2000000,"before":400000,"after":400000,"rewards":[]}],"limited_effort_box":[],"new_achievement_cnt":0,"museum_info": + {"parameter":{"smile":2948,"pure":2948,"cool":2948},"contents_id_list":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,127,128,129,130,131,132,133,134,135,136,137,138,139,140,141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,160,161,162,163,164,165,166,167,168,169,170,171,172,173,174,175,176,177,178,179,180,181,182,183,184,185,186,187,188,189,190,191,192,193,194,195,196,197,198,199,200,201,202,203,204,205,206,207,208,209,210,211,212,213,214,215,216,217,218,219,220,221,222,223,224,225,226,227,228,229,230,231,232,233,234,235,236,237,238,239,240,241,242,243,244,245,246,247,248,249,250,251,252,253,254,255,256,257,258,259,260,261,262,263,264,265,266,267,268,269,270,300,301,302,303,304,305,306,307,308,309,310,311,312,313,314,315,316,317,318,319,320,321,322,323,324,325,326,327,328,329,330,331,332,333,334,335,336,337,338,339,340,341,342,343,344,345,346,347,348,349,350,351,352,353,354,355,356,357,358,359,360,361,362,363,364,365,366,367,368,369,370,371,372,373,374,375,376,377,378,379,380,381,382,383,384,385,386,387,388,389,390,391,392,393,394,395,396,397,398,399,400,401,402,403,404,405,406,407,408,409,410,411,412,413,414,415,416,417,418,419,420,421,422,423,424,425,426,427,428,429,430,431,432,433,434,435,436,437,438,439,440,441,442,443,444,445,446,447,500,501,502,503,504,505,506,507,508,509,510,511,512,513,514,600,601,602,603,604,605,606,607,608,609,610,611,612,613,614,615,616,617,618,619,620,621,622,623,624,625,626,627,628,629,630,631,632,633,634,635,636,637,638,639,640,641,642,643,644,645,646,647,648,649,650,651,652,653,654,655,656,657,658,659,660,661,662,663,664,665,666,667,668,669,670,671,672,673,674,675,676,677,678,679,680,681,682,683,684,685,686,687,688,689,690,691,692,693,694,695,696,697,698,699,700,701,702,703,704,705,706,707,708,709,710,711,712,713,714,715,716,717,718,719,720,721,722,723,724,725,726,727,728,729,730,731,732,733,734,735,736,737,738,739,740,741,742,743,744,745,746,747,748,749,750,751,752,753,754,755,756,757,758,759,760,761,762,763,764,765,766,767,768,769,770,771,772,773,774,775,776,777,778,779,780,781,782,783,784,785,786,787,788,789,790,791,792,793,794,795,796,797,798,799,800,801,802,803,804,805,806,807,808,809,810,811,812,813,814,815,816,817,818,819,820,821,822,823,824,825,826,827,828,829,830,831,832,833,834,835,836,837,838,839,840,841,842,843,844,845,846,847,848,849,850,851,852,853,854,855,856,857,858,859,860,861,862,863,864,865,866,867,868,869,870,871,872,873,874,875,876,877,878,879,880,881,882,883,884,885,886,887,888,889,890,891,892,893,894,895,896,897,898,899,900,901,902,903,904,905,906,907,908,909,910,911,912,913,914,915,916,917,918,919,920,921,922,923,924,925,926,927,928,929,930,931,932,933,934,935,936,937,938,939,940,941,942,943,944,945,946,947,948,949,950,951,952,953,954,955,956,957,958,959,960,961,962,963,964,965,966,967,968,969,970,971,972,973,974,975,976,977,978,979,980,981,982,983,984,985,986,987,988,989,990,991,992,993,994,995,996,997,998,999,1000,1001,1002,1003,1004,1005,1006,1007,1008,1009,1010,1011,1012,1013,1014,1015,1016,1017,1018,1019,1020,1021,1022,1023,1024,1025,1026,1027,1028,1029,1030,1031,1032,1033,1034,1035,1036,1037,1038,1039,1040,1041,1042,1043,1044,1045,1046,1047,1048,1049,1050,1051,1052,1053,1054,1055,1056,1057,1058,1059,1060,1061,1062,1063,1064,1065,1066,1067,1068,1069,1070,1071,1072,1073,1074,1075,1076,1077,1078,1079,1080,1081,1082,1083,1084,1085,1086,1087,1160,1161,1162,1163,1164,1165,1166,1167,1168,1169,1170,1171,1172,1173,1174,1175,1176,1177,1178,1179,1180,1181,1182,1183,1184,1185,1186,1187,1188,1189,1190,1191,1192,1193,1194,1195,1196,1197,1198,1199,1200,1201,1202,1203,1204,1205,1206,1207,1208,1209,1210,1211,1212,1213,1214,1215,1216,1217,1218,1219,1220,1221,1222,1223,1224,1225,1226,1227,1228,1229,1230,1231,1232,1233,1234,1235,1236,1237,1238,1239,1240,1241,1242,1243,1244,1245,1246,1247,1248,1249,1250,1251,1252,1253,1254,1255,1256,1257,1258,1259,1260,1261,1262,1263,1264,1265,1266,1267,1268,1269,1270,1271,1272,1273,1274,1275,1276,1277,1278,1279,1280,1281,1282,1283,1284,1285,1286,1287,1288,1289,1290,1291,1292,1293,1294,1295,1296,1297,1298,1299,1300,1301,1302,1303,1304,1305,1306,1307,1308,1309,1310,1311,1312,1313,1314,1315,1316,1317,1318,1319,1320,1321,1322,1323,1324,1325,1326,1327,1328,1329,1330,1331,1332,1333,1334,1335,1336,1337,1338,1339,1340,1341,1342,1343,1344,1345,1346,1347,1348,1349,1350,1351,1352,1353,1354,1355,1356,1357,1358,1359,1360,1361,1362,1363,1364,1365,1366,1367,1368,1369,1370,1371,1372,1373,1374,1375,1376,1377,1378,1379,1380,1381,1382,1383,1384,1385,1386,1387,1388,1389,1390,1391,1392,1393,1394,1395,1396,1397,1398,1399,1400,1401,1402,1403,1404,1405,1406,1407,1408,1409,1410,1411,1412,1413,1414,1415,1416,1417,1418,1419,1420,1421,1422,1423,1424,1425,1426,1427,1428,1429,1430,1431,1432,1433,1434,1435,1436,1437,1438,1439,1440,1441,1442,1443,1444,1445,1446,1447,1448,1449,1450,1451,1452,1453,1454,1455,1456,1457,1458,1459,1460,1461,1462,1463,1464,1465,1597,1598,1599,1600,1601,1694,1695,1696,1697,1698,1699,1700,1701,1702,1703,1704,1705,1706,1707,1708,1709,1710,1711,1857,1858,1859,1860,1861,1862,1863,1864,1865,1866,1867,1868,1869,1870,1871,1872,1873,1874,1875,1876,1877,1878,1879,1880,1881,1882,1883,1884,1885,1886,1887,1888,1889,1890,1891,1892,1893,1894,1895,1896,1897,1898,1899,1900,1901,1902,1903,1904,1905,1906,1907,1908,1909,1910,1911,1912,1913,1914,1915,1916,1917,1918,1919,1920,1921,1922,1923,1924,1925,1926,1927,1928,1929,1930,1931,1932,1933,1934,1935,1936,1937,1938,1939,1940,1941,1942,1943,1944,1945,1946,1947,1948,1949,1950,1951,1952,1953,1954,1955,1956,1957,1958,1959,1960,1961,1962,1963,1964,1965,1966]}, "server_timestamp": timestamp(),"present_cnt":0},"release_info": rel_info, "status_code":200}
    
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
}

async function kidStatus(req, res) {
    const resp = {
        "response_data": {
            "has_klab_id": true,
            "server_timestamp": timestamp()
        },
        "release_info": rel_info,
        "status_code":200
    }
    const toSend = JSON.stringify(resp);
    signResp(req, res, toSend);
    res.send(toSend);
    res.end();
    
}

module.exports = {userInfo, gdpr, tos, changeName, tutorial, lbonus_execute, kidStatus};
