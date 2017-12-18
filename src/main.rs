#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

//TODO: Write test block? Write docs?

/// The VideoMeta struct mirrors structure of Video Metadata JSON files.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct VideoMeta {
    fps: u32,
    format: String,
    res_y: u32,
    res_x: u32,
    capture_start: u64,
    logger_id: String,
    device_id: String,
    tick: u64,
}

//TODO: Convert capture start/tick to seconds? or more readable time format?
/// Display implementation that cleanly prints out data contained
/// in a VideoMeta struct.
impl fmt::Display for VideoMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let device_id = format!("Device ID: {}", self.device_id);
        let logger_id = format!("Logger ID: {}", self.logger_id);
        let capture_start = format!("Capture Start: {}", self.capture_start);
        let tick = format!("Tick: {}", self.tick);
        let fps_format = format!("FPS: {} Format: {}", self.fps, self.format);
        let resolution = format!("Resolution: {} by {}", self.res_y, self.res_x);

        let display_string = [device_id, logger_id, capture_start, tick, fps_format, resolution]
            .join("\n");

        //TODO: Why is there no semicolon at the end of this write?
        write!(f, "{}", &display_string)

        //write!(f,
        //       r#"Device ID: {}
        //       Logger ID: {}
        //       "#, )
    }
}

/// Reads and deserializes JSON data into VideoMeta struct.
fn load_metadata_json<P: AsRef<Path>>(file_path: P) -> VideoMeta {
//fn load_metadata_json(file_path: &str) -> VideoMeta {
    let mut file = File::open(file_path)
        .expect("Failed to read file");

    let mut string_file = String::new();
    file.read_to_string(&mut string_file)
        .unwrap();

    //let data: VideoMeta = serde_json::from_str(&string_file)
    //    .unwrap();

    //return data;

    serde_json::from_str::<VideoMeta>(&string_file)
        .unwrap()
}

///Function for taking a directory and creating a list of VideoMeta structs
fn get_video_metadata(dir_path: &str) -> Vec<VideoMeta> {
    let paths = fs::read_dir(dir_path)
        .expect("Directory not found.");

    let mut video_data: Vec<VideoMeta> = vec![];
    //for path in paths {
    //    video_data
    //        .push(load_metadata_json(path.unwrap().path().to_str().unwrap()))
    //}
    for path in paths {
        video_data.push(
            load_metadata_json(path
                               .expect("Initial path unwrap failed.")
                               .path()
                               .to_str()
                               .expect("Final path unwrap failed.")));
    }
    return video_data;
}

// TODO: What to do when there is nothing matching the device ID?
/// Filters a referenced VideoMeta vector by device_id and
/// returns a new filtered vector.
fn get_by_device_id(device_id: &str, video_data: &Vec<VideoMeta>) -> Vec<VideoMeta> {
    // TODO: What exactly does 'borrow' mean in this context?
    // Here we can use `.iter()` or `.into_iter()` - first one is referencing
    // the original data where as the latter borrows the data
    let filtered_devices = video_data
        .iter()
        .filter(|ref i|i.device_id == device_id);

    let mut device_data: Vec<VideoMeta> = vec![];

    for each in filtered_devices {
        device_data.push(each.clone());
    }
    return device_data
}

fn sort_by_capture_start(video_data: &mut Vec<VideoMeta>) {
    // Sorts VideoMeta vec by capture time.
    video_data.sort_by(|a, b| a.capture_start.cmp(&b.capture_start));
}

/// Creates a new VeideoMeta struct with faux information and current time,
/// then serializes it to write a JSON metadata file.
fn write_metadata_file() {
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

    println!("Listing all metadata files from device ID: {}", device_id);
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
