use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_csv_to_hashmap<P: AsRef<Path>>(
    file_path: P,
) -> Result<HashMap<String, Vec<Geotag>>, Box<dyn Error>> {
    let mut tag_map = HashMap::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() >= 5 {
            let tagname = fields[0].to_string();
            let date = fields[1].to_string();
            let latitude = fields[2].parse::<f64>()?;
            let longitude = fields[3].parse::<f64>()?;
            let url = fields[4].to_string();

            let geotag = Geotag {
                date,
                latitude,
                longitude,
                url,
            };

            tag_map.entry(tagname).or_insert(vec![]).push(geotag);
        }
    }
    Ok(tag_map)
}

// tag.json の struct
/*
[hint] 現状は愚直に各フィールドを読み込んでいるので RAM が 1GB だと多分メモリに乗せるのは無理です
乗せる方法としては以下の方法が挙げられます。
1. url の表現方法を変えてみる
    URL の形式は http://farm9.static.flickr.com/8050/8376611070_aeb13ec0fe.jp
    http://farm と .static.flickr.com/ と .jp は共通部分だから取り除くことができそう
    8050 は文字列だと 4byte だが整数にすれば 2byte(16bit) で表現できそう
    こんな感じのことをうまい具合にやると 60byte から 10byte ぐらいまで節約できると思います
2. date の表現方法を変えてみる
    ある時間を基準とした経過時間として表現すれば 32bit まで削減できそう
3. など
*/
#[derive(Serialize, Deserialize)]
pub struct TagGeotag {
    pub tag_name: String,
    pub geotags: Vec<Geotag>,
}

#[derive(Serialize, Deserialize)]
pub struct Geotag {
    pub date: String,
    pub latitude: f64,
    pub longitude: f64,
    pub url: String,
}
