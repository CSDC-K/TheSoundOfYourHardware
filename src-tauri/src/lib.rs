use std::{io};
use std::fs::{DirEntry,create_dir_all, read_dir};
use std::path::{Path, PathBuf};
use infer;

use tauri::{AppHandle, Manager};
use tokio::process::Command;
use sysinfo::{self, System};

use serde::{Deserialize, Serialize};


#[derive(Serialize)]
struct ProcessList{
    #[allow(non_snake_case)]
    PID: u32,
    #[allow(non_snake_case)]
    NAME: String
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "Limit", content = "LimitValue")]
pub enum Limits {
    Cpu { Rate : u8 },
    Gpu { Rate : u8 },
    Memory { Rate : u8 },
    Heat { Rate : u8 },
}

#[derive(serde::Serialize)]
enum SoundFx{
    SPEED125X,
    SPEED150X,
    SPEED175X,
    SPEED200X,
    SPEED250X,
    SPEED300X,
}

#[derive(serde::Serialize)]
struct Adjustments{
    #[allow(non_snake_case)]
    SOUNDLEVEL : u8,
    #[allow(non_snake_case)]
    CHECKINTERVAL : u8,
    #[allow(non_snake_case)]
    SOUNDFX : SoundFx
}

struct TheApp {
    SoundPath : String,
    Limits : Vec<Limits>,
    adjustments : Adjustments
    
}

#[tauri::command]
async fn get_sounds(handle : AppHandle) -> Vec<String> {
    let mut path = handle.path().app_local_data_dir().expect("Got an error while trying to get app_local_data_dir");
    path.push("Sounds/");
    let mut sound_vector : Vec<String> = vec![];
    for entry in read_dir(path).expect("Got an error while trying to read sounds dir"){
        let dir = entry.expect("Got an error while trying to get DirEntry");
        let filematcher = infer::get_from_path(dir.path().as_path());
        match  filematcher{
            Ok(Some(info)) => {
                if info.mime_type() == "audio/mpeg"{
                    sound_vector.push(dir.path().file_name().unwrap().to_string_lossy().into_owned());
                }

            }
            Ok(None) => {
                println!("Found broken or unexpected file.");
            }

            Err(e) => {
                println!("Got an error while trying to read file_type : {}", e);
            }
        }
    }

    sound_vector
}

#[tauri::command]
async fn get_apps() -> Result<Vec<ProcessList>, String> {
    let mut sys = System::new_all();
    let mut process_vector : Vec<ProcessList> = vec![];
    sys.refresh_all();

    for (pid, process) in sys.processes(){
        let process_name = process.name().to_string_lossy().into_owned();
        process_vector.push(ProcessList { PID: pid.as_u32(), NAME: process_name });
    }

    Ok(process_vector)

}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app|{
            let mut path = app.app_handle().path().app_local_data_dir().expect("Got an error while trying to get app_local_data_dir");
            if !path.exists(){
                println!("Trying to create app_local_data_dir");
                create_dir_all(&path).expect("Got an error while trying to create app_local_data_dir");
            }

            path.push("Sounds/");
            if !path.exists(){
                println!("Trying to create Sounds path");
                create_dir_all(&path).expect("Got an error while trying to create Sounds path.");
            }
            Ok(())

        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_sounds,
            get_apps
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
