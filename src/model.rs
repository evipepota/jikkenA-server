use rustc_hash::FxHasher;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    hash::BuildHasherDefault,
    io::{self, BufRead, BufReader},
    path::Path,
};

type Hasher = BuildHasherDefault<FxHasher>;

#[derive(Serialize, Deserialize)]
pub struct TagJSON {
    pub list: Vec<TagGeotag>,
}

impl TagJSON {
    // json を読み込んで TagJSON に unmarshal して返す
    pub fn from_path<P: AsRef<Path>>(p: P) -> Result<Self, Box<dyn Error>> {
        let f = File::open(p)?;
        let r = BufReader::new(f);
        let tag_json = serde_json::from_reader(r)?;
        Ok(tag_json)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagData {
    tag_name: String,
    geotags: Vec<Geotag>,
}

pub fn read_csv_to_hashmap<P: AsRef<Path>>(
    file_path: P,
) -> Result<HashMap<String, Vec<Geotag>, Hasher>, Box<dyn Error>> {
    let mut tag_map: HashMap<String, Vec<Geotag>, Hasher> = HashMap::default();
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() >= 5 {
            let tagname = fields[0].to_string();
            let date = fields[1].parse::<i32>()?;
            let latitude = fields[2].parse::<f64>()?;
            let longitude = fields[3].parse::<f64>()?;
            let url = fields[4].to_string();

            let geotag = Geotag {
                date,
                lat: latitude,
                lon: longitude,
                url,
            };

            tag_map.entry(tagname).or_insert(vec![]).push(geotag);
        }
    }
    Ok(tag_map)
}

#[derive(Serialize, Deserialize)]
pub struct TagGeotag {
    pub tag_name: String,
    pub geotags: Vec<Geotag>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Geotag {
    pub date: i32,
    pub lat: f64,
    pub lon: f64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeotagReal {
    pub date: String,
    pub lat: f64,
    pub lon: f64,
    pub url: String,
}
