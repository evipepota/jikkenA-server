input_file = './data/output.json'
output_file = './data/output2.json'

import json

def remove_flickr_url(json_data):
    if isinstance(json_data, dict):
        for key, value in json_data.items():
            if key == "url" and value.startswith("http://farm9.static.flickr.com/") and value.endswith(".jpg"):
                json_data[key] = value.replace("http://farm9.static.flickr.com/", "").replace(".jpg", "")
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

# 結果を保存
with open(output_file, 'w', encoding='utf-8') as output_file:
    json.dump(data, output_file, ensure_ascii=False)
