import json

inputfile = './data/test.json'
outputfile = './data/test_output.json'

# JSONファイルの読み込み
with open(inputfile, 'r') as f:
    data = json.load(f)

# geotagsをdateでソート
n = len(data['list'])
i = 0
while i < n:
    data['list'][i]['geotags'] = sorted(data['list'][i]['geotags'], key=lambda x: x['date'])
    i += 1

# ソート結果をoutput.jsonに出力
with open(outputfile, 'w', encoding='utf-8') as f:
    json.dump(data, f, ensure_ascii=False)
