import json

def replace_key(json_data):
    if isinstance(json_data, dict):
        for key in list(json_data.keys()):
            if key == 'latitude':
                json_data['lat'] = json_data.pop(key)
            elif key == 'longitude':
                json_data['lon'] = json_data.pop(key)
            else:
                replace_key(json_data[key])
    elif isinstance(json_data, list):
        for item in json_data:
            replace_key(item)

# JSONファイルを読み込む
with open('./data/output.json', 'r') as file:
    data = json.load(file)

# キーの名前を変更する
replace_key(data)

# 変更後のJSONをb.jsonに保存
with open('./data/output2.json', 'w', encoding='utf-8') as output_file:
    json.dump(data, output_file, ensure_ascii=False)
