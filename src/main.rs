#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Read;

//TODO: Read about primitive datatypes and choose appropriate ones
//TODO: serde_milliseconds for timestamps?
// TODO: Determine proper way to add code comments
#[derive(Serialize, Deserialize, Debug, Clone)]
struct VideoMeta {
    // The VideoMeta struct mirrors structure of JSON Metadata files.
    fps: i32,
    format: String,
    res_y: i32,
    res_x: i32,
    capture_start: i64,
    logger_id: String,
    device_id: String,
    tick: i64,
}

//TODO: Convert capture start/tick to seconds? or more readable time format?
impl fmt::Display for VideoMeta {
    // Display implementation that cleanly prints out data contained
    // in a VideoMeta struct.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let device_id = format!("Device ID: {}", self.device_id);
        let logger_id = format!("Logger ID: {}", self.logger_id);
        let capture_start = format!("Capture Start: {}", self.capture_start);
        let tick = format!("Tick: {}", self.tick);
        let fps_format = format!("FPS: {} Format: {}", self.fps, self.format);
        let resolution = format!("Resolution: {} by {}", self.res_y, self.res_x);

        let display_string = [device_id, logger_id, capture_start, tick, fps_format, resolution]
            .join("\n");

        write!(f, "{}", &display_string)
    }
}

fn load_metadata_json(file_path: &str) -> VideoMeta {
    // Reads and deseralizes JSON data into VideoMeta struct.
    let mut file = File::open(file_path)
        .expect("Failed to read file");

    let mut string_file = String::new();
    file.read_to_string(&mut string_file).unwrap();

    let data: VideoMeta = serde_json::from_str(&string_file).unwrap();
    return data;
}

fn get_video_metadata(dir_path: &str) -> Vec<VideoMeta> {
    //Function for taking a directory and creating a list of VideoMeta structs
    let paths = fs::read_dir(dir_path)
        .expect("Directory not found.");

    let mut video_data: Vec<VideoMeta> = vec![];
    for path in paths {
        video_data.push(load_metadata_json(path.unwrap().path().to_str().unwrap()))
    }
    return video_data
}

// TODO: What to do when there is nothing matching the device ID?
fn get_by_device_id(device_id: &str, video_data: &Vec<VideoMeta>) -> Vec<VideoMeta> {
    // Filters VideoMeta vector by device_id and returns a new vector.
    let filtered_devices = video_data.into_iter().filter(|ref i|i.device_id == device_id);

    let mut device_data: Vec<VideoMeta> = vec![];

    for each in filtered_devices {
        device_data.push(each.clone());
    }
    return device_data
}

fn sort_by_capture_start(mut video_data: Vec<VideoMeta>) -> Vec<VideoMeta> {
    // Sorts VideoMeta vec by capture time.
    video_data.sort_by(|a, b| a.capture_start.cmp(&b.capture_start));
    return video_data;
}

//TODO: Write function to seralize data and write a new JSON file
#[test]
fn write_metadata_file() {
    let fps = 10;
    let res_y = 1232;
    let res_x = 1640;
    let format = "video/mp4";
    let logger_id = "internal_test";
    let device_id = "faux_device";
    //TODO: Get current time for capture start, round for tick.
    //Create a new VideoMeta data structure.
    //Seralize and write.
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

    video_data = sort_by_capture_start(video_data);
    println!("\n\nPost sorted:");
    for data in &video_data {
            println!("{}", data.capture_start);
    }
}
