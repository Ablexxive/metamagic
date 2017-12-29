#![crate_name = "metamagic"]
extern crate chrono;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::*;

/// The VideoMeta struct mirrors structure of Video Metadata JSON files.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoMeta {
    fps: u32,
    format: String,
    res_y: u32,
    res_x: u32,
    capture_start: u64,
    logger_id: String,
    device_id: String,
    tick: u64,
}

/// Display implementation that cleanly prints out data contained
/// in a VideoMeta struct.
///
/// # Remarks
///
/// Displayed 'Capture Time' and 'Tick' are calculated from incomplete data and may be incorrect.
/// To get exact nanosecond values, use the Debug trait.
impl fmt::Display for VideoMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmted_capture = NaiveDateTime::from_timestamp((self.capture_start/1000) as i64,
                                                          (self.capture_start % 1000) as u32);
        let fmted_tick = NaiveDateTime::from_timestamp((self.tick/1000) as i64, 0);

        // Could have both halfs in one but its a silly long line and I'm not
        // sure how to escape an implict new line to split it up.
        let first_half = format!("Device ID: {}\nLogger ID: {}\nCapture Start: {}\nTick: {}",
                                 self.device_id, self.logger_id, fmted_capture, fmted_tick);
        let second_half = format!("FPS: {} Format: {}\nResolution: {} by {}",
                                   self.fps, self.format, self.res_y, self.res_x);
        let display_string = [first_half, second_half].join("\n");
        write!(f, "{}", display_string)
    }
}

/// Reads and deserializes JSON data into VideoMeta struct.
/// # Arguments
///
/// * `file_path` - path specifying JSON file location.
///
/// # Remarks
///
/// Argument type is defined as an AsRef<Path> so
/// many types are accepted (Path, String &str, ect.)
//pub fn load_metadata_json<P: AsRef<Path>>(file_path: P) -> Option<VideoMeta> {
pub fn load_metadata_json<P: AsRef<Path>>(file_path: P) -> Result<VideoMeta, serde_json::Error> {
    //TODO: Figure out a generic error return for Result so you can return both IO errors and serde
    //errors -> Maybe that's what Option is used for, w/ VideoMeta and None
    let mut f = File::open(file_path)
        .expect("Failed to open file.");

    let mut string_file = String::new();
    f.read_to_string(&mut string_file)
        .expect("Failed to read file to string.");

    serde_json::from_str::<VideoMeta>(&string_file)
}

/// Creates a vector of VideoMeta structs populated by all JSON files in directory.
/// # Arguments
///
/// * `dir_path` - path specifying directory of JSON metadata files
///
/// # Remarks
///
/// Argument type is defined as an AsRef<Path> so
/// many types are accepted (Path, String &str, ect.)
pub fn get_video_metadata<P: AsRef<Path>>(dir_path: P) -> Vec<VideoMeta> {
    let entries = fs::read_dir(dir_path)
        .expect("Directory not found.");

    let mut video_data: Vec<VideoMeta> = vec![];

    for entry in entries {
        if let Err(err) = entry {
            println!("Invalid Entry: {}", err);
            continue;
        }
        let file_path = entry.unwrap().path();

        match load_metadata_json(&file_path) {
            Ok(data) => {
                video_data.push(data);
                println!("File {:?} loaded.",
                         file_path);
            },
            Err(e) => {
                println!("File {:?} is not a valid video metadata JSON.\nError: {}",
                         file_path, e);
            },
        };
    }
    return video_data;
}

/// Filters a referenced VideoMeta vector by device_id and
/// returns a new filtered vector.
/// # Arguments
///
/// * `device_id` - String reference to ID number of desired device metadata.
/// * `video_data` - vector of VideoMeta structs to be filtered
pub fn get_by_device_id(device_id: &str, video_data: &Vec<VideoMeta>) -> Vec<VideoMeta> {
    let filtered_devices = video_data
        .iter()
        .filter(|ref i|i.device_id == device_id);

    let mut device_data: Vec<VideoMeta> = vec![];

    for each in filtered_devices {
        device_data.push(each.clone());
    }

    return device_data
}

/// Sorts a vector of VideoMeta by capture time
/// # Arguments
///
/// * `video_data` - vector of VideoMeta structs to be filtered
pub fn sort_by_capture_start(video_data: &mut Vec<VideoMeta>) {
    video_data.sort_by(|a, b| a.capture_start.cmp(&b.capture_start));
}

/// Creates a new VeideoMeta struct with faux information and current time (ms),
/// then serializes it to write a JSON metadata file.
pub fn write_metadata_file() {
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let capture_start = since_epoch.as_secs() * 1_000
                + (since_epoch.subsec_nanos() / 1_000_000) as u64;

    let tick = ((capture_start as f64 / 10_000.0).round() * 10_000.0) as u64;

    let metadata = VideoMeta {
        fps: 10,
        res_y: 1232,
        res_x: 1640,
        format: "video/mp4".to_string(),
        logger_id: "internal_test".to_string(),
        device_id: "faux_device".to_string(),
        capture_start: capture_start,
        tick: tick,
    };
    let json_to_write = serde_json::to_string(&metadata)
        .expect("Could not seralize VideoMeta.");

    let path = Path::new("test_file.json");
    let mut f = File::create(&path)
        .expect("Could not create file.");

    f.write_all(json_to_write.as_bytes())
        .expect("Could not write file.");
}

fn main() {
    let metadata_folder = "./metadata";
    let device_id = "1fc0c10b0a534202";

    let mut video_data = get_video_metadata(metadata_folder);

    let device_data = get_by_device_id(device_id, &video_data);

    println!("\nListing all metadata files from device ID: {}", device_id);
    for data in device_data {
        println!("{}\n", data);
    }

    println!("Sorting metadata files by capture start times:");
    println!("Pre sorted:");
    for data in &video_data {
            println!("{}", data.capture_start);
    }

    sort_by_capture_start(&mut video_data);
    println!("\n\nPost sorted:");
    for data in &video_data {
            println!("{}", data.capture_start);
    }
    write_metadata_file();
}
