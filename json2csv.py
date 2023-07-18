import csv
import json

inputfile = './data/output.json'
outputfile = './data/output2.csv'

def json_to_csv(json_data):
    # JSONデータを読み込む
    data = json.loads(json_data)
    
    # CSVファイルのヘッダー
    header = ['tagname', 'date', 'latitude', 'longitude', 'url']
    
    # CSVファイルのデータ行
    rows = []
    
    # JSONデータを走査してCSVデータを作成
    for item in data['list']:
        tagname = item['tag_name']
        for geotag in item['geotags']:
            date = geotag['date']
            latitude = geotag['latitude']
            longitude = geotag['longitude']
            url = geotag['url']
            rows.append([tagname, date, latitude, longitude, url])
    
    # CSVファイルに書き込む
    with open(outputfile, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerows(rows)

# JSONファイルのパス
json_file = inputfile

# JSONファイルを読み込む
with open(json_file, 'r') as file:
    json_data = file.read()

# JSONをCSVに変換
json_to_csv(json_data)
