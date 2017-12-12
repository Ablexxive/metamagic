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
// The VideoMeta struct mirrors structure of JSON Metadata files.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct VideoMeta {
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

fn load_metadata_file(file_path: &str) -> VideoMeta {
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
        video_data.push(load_metadata_file(path.unwrap().path().to_str().unwrap()))
    }
    return video_data
}

fn get_by_device_id(device_id: &str, video_data: &Vec<VideoMeta>) -> Vec<VideoMeta> {
    let filtered_devices = video_data.into_iter().filter(|ref i|i.device_id == device_id);

    let mut device_data: Vec<VideoMeta> = vec![];

    for each in filtered_devices {
        println!("{:?}", each);
        device_data.push(each.clone());
    }
    return device_data
    //return filtered_devices;
    //let device_data = filtered_devices.collect::<Vec<_>>();
    //return device_data
    //video_data.retain(|ref i|i.device_id == device_id);
    //return video_data
}

// function for sorting input, input: list of structs, output: sorted list
fn sort_by_capture_start(mut video_data: Vec<VideoMeta>) -> Vec<VideoMeta> {
//fn sort_by_capture_start(mut video_data: Vec<VideoMeta>) {
    //let data_iter = video_data.into_iter().sort_by(|a, b| a.capture_start.cmp(b.capture_start));
    //return data_iter.collect::<Vec<_>>();
    video_data.sort_by(|a, b| a.capture_start.cmp(&b.capture_start));
    //println!("{:?}", video_data);
    return video_data;
}

// TODO: Write a test case that shows VideoMeta Debug trait
fn main() {
    //let data = load_metadata_file("test.json");
    //println!("{}\n", &data);

    let mut video_data = get_video_metadata("./metadata");
    //println!("{:?}", &video_data);

    //let device_data = get_by_device_id("1fc0c10b0a534202", video_data.to_vec());
    let device_data = get_by_device_id("1fc0c10b0a534202", &video_data);

    for data in device_data {
        println!("{}", data);
    }

    //println!("Pre sorted:");
    //for data in &video_data {
    //        println!("{}", data.capture_start);
    //}
    //println!("\n\n\n\nPost sorted:");
    //video_data = sort_by_capture_start(video_data);
    //for data in &video_data {
    //        println!("{}", data.capture_start);
    //}
}
