input_file = './data/output.json'
output_file = './data/output2.json'

import json
from datetime import datetime

def convert_date_to_timestamp(date_str):
    datetime_obj = datetime.strptime(date_str, "%Y-%m-%d %H:%M:%S")
    timestamp = int(datetime_obj.timestamp())
    return timestamp

def remove_flickr_url(json_data):
    if isinstance(json_data, dict):
        for key, value in json_data.items():
            if key == "url" and value.startswith("http://farm") and value.endswith(".jpg"):
                json_data[key] = value.replace("http://farm", "").replace(".jpg", "").replace(".static.flickr.com/", "")
            else:
                remove_flickr_url(value)
    elif isinstance(json_data, list):
        for item in json_data:
            remove_flickr_url(item)

# ファイルを読み込む
with open(input_file, 'r') as file:
    data = json.load(file)

# URLタグから"http://farm9.static.flickr.com/"と".jpg"を削除する
remove_flickr_url(data)

min_date = 9999999999
max_date = 0

# 日付をintに変換
for item in data['list']:
    for geotag in item['geotags']:
        geotag['date'] = convert_date_to_timestamp(geotag['date'])
        # min_date = min(min_date, geotag['date'])

# 結果を保存
with open(output_file, 'w', encoding='utf-8') as output_file:
    json.dump(data, output_file, ensure_ascii=False)

# maxとminのdateの値の差分を調べたい
#for item in data['list']:
#    for geotag in item['geotags']:
#        max_date = max(geotag['date']-min_date, max_date)

# 最小のdateの値を出力
#print(min_date)
#print(max_date)
