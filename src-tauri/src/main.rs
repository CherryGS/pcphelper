// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rfd::FileDialog;
use std::{fs, path::PathBuf, sync::Mutex};

struct AppConfig {
    config_folder: PathBuf,
    recent_file: PathBuf,
}

static CONFIG: Mutex<Option<AppConfig>> = Mutex::new(None);
static PATH: Mutex<Option<PathBuf>> = Mutex::new(None);

#[tauri::command]
fn open_main_path_and_save_recent() -> String {
    // 选择文件夹并将文件路径写入 recent
    let path = FileDialog::new().pick_folder();
    if path.is_none() {
        return String::new();
    }
    let r = path
        .clone()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    match &*CONFIG.lock().unwrap() {
        Some(c) => {
            fs::write(&c.recent_file, &r).unwrap();
        }
        None => {
            panic!("'Recent' file does not exist.")
        }
    }

    // 存储到全局
    let mut p = PATH.lock().unwrap();
    *p = path;
    r
}

#[tauri::command]
fn set_main_path(s: String) {
    let mut p = PATH.lock().unwrap();
    *p = Some(PathBuf::from(s));
}

#[tauri::command]
fn read_main_path() -> String {
    match &*PATH.lock().unwrap() {
        Some(p) => p.clone().into_os_string().into_string().unwrap(),
        None => {
            panic!("No main_path exists now.")
        }
    }
}

#[tauri::command]
fn read_recent() -> Vec<String> {
    match &*CONFIG.lock().unwrap() {
        Some(c) => {
            let mut v = Vec::new();
            for line in fs::read_to_string(&c.recent_file).unwrap().lines() {
                v.push(line.to_string())
            }
            v
        }
        None => {
            panic!("'Recent' file does not exist.")
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 初始化 config 文件夹路径
            let config_folder = app
                .path_resolver()
                .app_config_dir()
                .expect("Failed on resolving config dir.");
            fs::create_dir_all(&config_folder)?;

            // 初始化 recent 记录文件
            let recent_file = config_folder.join("Recent.txt");
            if !recent_file.exists() {
                fs::write(&recent_file, "")?;
            }

            // 信息存储到全局
            let mut p = CONFIG.lock().unwrap();
            *p = Some(AppConfig {
                config_folder,
                recent_file,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_main_path_and_save_recent,
            read_recent,
            set_main_path,
            read_main_path
        ])
        .run(tauri::generate_context!())
        .expect("Failed on running app.");
}
