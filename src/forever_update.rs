use std::{fs, io, process::Command, thread, time::Duration};
use chrono::{DateTime, Local};
use http::{HeaderMap, Method, Request, Response, StatusCode, Uri};
use json::{from_slice, JsonValue};
use log::{debug, error, info, warn};
use regex::Regex;
use reqwest::Client;

// Configuration variables
const EW: &str = "172.17.120.25:9070";
const UC: &str = "default";
const CI: &str = "mongodb://172.17.120.26:27017/";
const DB: &str = "esl17yace9";
const COLLECT: &str = "esl";
const BAK_URL: &str = "http://10.11.163.211:8080/shopweb/ogi/ew/httpHandler";

fn main() {
    // Initialize logging
    env_logger::init().unwrap();

    // Determine OS and set paths accordingly
    let os = std::env::consts::OS;
    let ew_fp = match os {
        "windows" => r"D:\BBIT_ROUND2\eslw-5.0.2rc0",
        "linux" => "/usr/local/eslw_5.0.2rc3yace",
        _ => panic!("Unsupported OS"),
    };

    let log_path = format!("{}/run.log", ew_fp);
    let ew_wl = format!("{}/config/white_list_bak.txt", ew_fp);
    let ew_log_path = format!("{}/log/eslworking.log", ew_fp);
    let api_log_fp = format!("{}/log/api.log", ew_fp);
    let battery_fp = format!("{}/var/log/battery.log", ew_fp);
    let battery_api_fp = format!("{}/var/log/batteryapi.log", ew_fp);

    // Run the update loop
    forever_update(ew_log_path).unwrap();
}

fn get_esl(f: &str) -> Vec<String> {
    let mut esl_list = Vec::new();
    let content = fs::read_to_string(f).unwrap();
    for line in content.lines() {
        if !line.starts_with("#") {
            esl_list.push(line.replace("\n", "").replace(&format!("={UC}"), ""));
        }
    }
    info!("ESL list length: {}", esl_list.len());
    esl_list
}

fn is_during(start: &str, end: &str) -> bool {
    let now = Local::now();
    let start_time = DateTime::parse_from_str(format!("{}{}", now.date(), start), "%Y-%m-%d%H:%M").unwrap();
    let end_time = DateTime::parse_from_str(format!("{}{}", now.date(), end), "%Y-%m-%d%H:%M").unwrap();
    now.ge(&start_time) && now.lt(&end_time)
}

fn get_file_seek(file_path: &str) -> u64 {
    let mut f = fs::OpenOptions::new().read(true).open(file_path).unwrap();
    let size = f.seek(io::SeekPos::End).unwrap();
    debug!("Current file seek: {}", size);
    size
}

fn create_api_file(pid: &str) -> Result<(), reqwest::Error> {
    let url = format!("http://{}/api3/{}/esls/product/{}", EW, UC, pid);
    let client = Client::new();
    let req = Request::builder()
        .method(Method::GET)
        .uri(url.parse::<Uri>()?)
        .body(Body::empty())
        .build()?;
    client.execute(req)?;
    info!("Created API file");
    Ok(())
}

fn send_battery_api(esl_list: &[String]) -> Result<(), reqwest::Error> {
    let mut data = Vec::new();
    for esl in esl_list {
        data.push(json!({
            "esl_id": esl,
            "sid": "1
            use std::{fs, io, process::Command, thread, time::Duration};
            use chrono::{DateTime, Local};
            use http::{HeaderMap, Method, Request, Response, StatusCode, Uri};
            use json::{from_slice, JsonValue};
            use log::{debug, error, info, warn};
            use regex::Regex;
            use reqwest::Client;
            
            // Configuration variables
            const EW: &str = "172.17.120.25:9070";
            const UC: &str = "default";
            const CI: &str = "mongodb://172.17.120.26:27017/";
            const DB: &str = "esl17yace9";
            const COLLECT: &str = "esl";
            const BAK_URL: &str = "http://10.11.163.211:8080/shopweb/ogi/ew/httpHandler";
            
            fn main() {
                // Initialize logging
                env_logger::init().unwrap();
            
                // Determine OS and set paths accordingly
                let os = std::env::consts::OS;
                let ew_fp = match os {
                    "windows" => r"D:\BBIT_ROUND2\eslw-5.0.2rc0",
                    "linux" => "/usr/local/eslw_5.0.2rc3yace",
                    _ => panic!("Unsupported OS"),
                };
            
                let log_path = format!("{}/run.log", ew_fp);
                let ew_wl = format!("{}/config/white_list_bak.txt", ew_fp);
                let ew_log_path = format!("{}/log/eslworking.log", ew_fp);
                let api_log_fp = format!("{}/log/api.log", ew_fp);
                let battery_fp = format!("{}/var/log/battery.log", ew_fp);
                let battery_api_fp = format!("{}/var/log/batteryapi.log", ew_fp);
            
                // Run the update loop
                forever_update(ew_log_path).unwrap();
            }
            
            fn get_esl(f: &str) -> Vec<String> {
                let mut esl_list = Vec::new();
                let content = fs::read_to_string(f).unwrap();
                for line in content.lines() {
                    if !line.starts_with("#") {
                        esl_list.push(line.replace("\n", "").replace(&format!("={UC}"), ""));
                    }
                }
                info!("ESL list length: {}", esl_list.len());
                esl_list
            }
            
            fn is_during(start: &str, end: &str) -> bool {
                let now = Local::now();
                let start_time = DateTime::parse_from_str(format!("{}{}", now.date(), start), "%Y-%m-%d%H:%M").unwrap();
                let end_time = DateTime::parse_from_str(format!("{}{}", now.date(), end), "%Y-%m-%d%H:%M").unwrap();
                now.ge(&start_time) && now.lt(&end_time)
            }
            
            fn get_file_seek(file_path: &str) -> u64 {
                let mut f = fs::OpenOptions::new().read(true).open(file_path).unwrap();
                let size = f.seek(io::SeekPos::End).unwrap();
                debug!("Current file seek: {}", size);
                size
            }
            
            fn create_api_file(pid: &str) -> Result<(), reqwest::Error> {
                let url = format!("http://{}/api3/{}/esls/product/{}", EW, UC, pid);
                let client = Client::new();
                let req = Request::builder()
                    .method(Method::GET)
                    .uri(url.parse::<Uri>()?)
                    .body(Body::empty())
                    .build()?;
                client.execute(req)?;
                info!("Created API file");
                Ok(())
            }
            
            fn send_battery_api(esl_list: &[String]) -> Result<(), reqwest::Error> {
                let mut data = Vec::new();
                for esl in esl_list {
                    data.push(json!({
                        "esl_id": esl,
                        "sid": "1
            