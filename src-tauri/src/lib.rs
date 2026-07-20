use std::collections::HashMap;
use std::sync::Mutex;
use std::{io};
use std::fs::{DirEntry,create_dir_all, read_dir};
use std::path::{Path, PathBuf};
use infer;
use tokio::time::{sleep, Duration};
use tauri::{AppHandle, Manager};
use sysinfo::{self, System};

use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;


#[derive(Default)]
struct AppState{
    thekey : HashMap<String, CancellationToken>,
    thestate : bool
}

#[derive(Debug,Deserialize,Serialize)]
struct TheSound{
    SoundPath : String,
    SoundName : String,
}

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

#[derive(serde::Serialize, Deserialize,Debug, Clone)]
enum SoundFx{
    NONE,
    SPEED125X,
    SPEED150X,
    SPEED175X,
    SPEED200X,
    SPEED250X,
    SPEED300X,
}

#[derive(serde::Serialize, Deserialize, Debug, Clone)]
struct Adjustments{
    #[allow(non_snake_case)]
    SOUNDLEVEL : u8,
    #[allow(non_snake_case)]
    CHECKINTERVAL : f32,
    #[allow(non_snake_case)]
    SOUNDFX : SoundFx
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TheApp {
    SoundPath : String,
    Limits : Vec<Limits>,
    Adjustments : Adjustments
}

impl TheApp {
    pub async fn create(self, handle : AppHandle) {

        let cancellationtoken = CancellationToken::new();
        let active_token = cancellationtoken.clone();
        let worker_handle = handle.clone();

        tokio::spawn(async move {
            watch_hardware(active_token, self, worker_handle).await;
        });

        let state = handle.state::<Mutex<AppState>>();
        let mut app_state = state.lock().unwrap();
        app_state.thekey.insert(format!("TheKey"), cancellationtoken);
        app_state.thestate = true;
        

    }
}


#[tauri::command]
async fn create(
    handle : AppHandle,
    StructState : TheApp
) -> Result<(), String>{
    let mut mutablehandle = handle.clone();
    let should_stop = {
        let state = handle.state::<Mutex<AppState>>();
        let mut app_state = state.lock().unwrap();
        app_state.thestate
    };

    if should_stop == true{
        stop(&mut mutablehandle).await;
        notifysend(format!("Stop"), format!("App was already running in background, now it's stopped.")).await;
        Ok(())
    }

    else{
        println!("SoundPath : {}", StructState.SoundPath);
        println!("Limits : {:?}", StructState.Limits);
        println!("Adjustments-SOUNDFX : {:?}", StructState.Adjustments.SOUNDFX); 
        println!("Adjustments-CHECKINTERVAL : {}", StructState.Adjustments.CHECKINTERVAL);
        println!("Adjustments-SOUNDLEVEL : {}", StructState.Adjustments.SOUNDLEVEL);
        

        let TheAppStruct : TheApp = TheApp { SoundPath: StructState.SoundPath, Limits: StructState.Limits, Adjustments: StructState.Adjustments };
        TheAppStruct.create(handle.clone()).await;
        
        Ok(())
    }


}


#[tauri::command]
async fn stop(handle : &mut AppHandle) -> Result<(), String>{

    let cancellation_token = {
        let state = handle.state::<Mutex<AppState>>();
        let mut app_state = state.lock().unwrap();
        app_state.thestate = false;
        app_state.thekey.remove("TheKey")
    };

    match cancellation_token {
        Some(token) => {
            token.cancel();
            Ok(())
        }
        None => Ok(()),
    }

}

async fn watch_hardware(cancellationtoken : CancellationToken, app : TheApp, handle : AppHandle){

    for limit in app.Limits.iter(){
        let WorkerStruct = app.clone();
        let WorkerHandle = handle.clone();
        let workertoken = cancellationtoken.clone();
        match limit{
            Limits::Cpu { Rate } => {
                tokio::spawn(async move {
                    Async_Cpu( WorkerHandle, WorkerStruct, workertoken).await
                });
            },
            Limits::Gpu { Rate } => {
                tokio::spawn(async move {
                    Async_Gpu( WorkerHandle, WorkerStruct, workertoken).await
                });
            },
            Limits::Memory { Rate } => {
                tokio::spawn(async move {
                    Async_Memory( WorkerHandle, WorkerStruct, workertoken).await
                });
            },
            Limits::Heat { Rate } => {
                tokio::spawn(async move {
                    Async_Heat( WorkerHandle, WorkerStruct, workertoken).await
                });
            },
        }
    }
}


async fn Async_Cpu(handle : AppHandle, app : TheApp, cancellationtoken : CancellationToken) {
    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs_f32(app.Adjustments.CHECKINTERVAL)) => {
                println!("test for cpu");
            },

            _ = cancellationtoken.cancelled() => {
                break;
            }
        }
    }
}

async fn Async_Gpu(handle : AppHandle, app : TheApp, cancellationtoken : CancellationToken) {
    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs_f32(app.Adjustments.CHECKINTERVAL)) => {
                println!("test for gpu");
            },
            _ = cancellationtoken.cancelled() => {
                break;
            }
        }
    }
}

async fn Async_Memory(handle : AppHandle, app : TheApp, cancellationtoken : CancellationToken) {
    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs_f32(app.Adjustments.CHECKINTERVAL)) => {
                println!("test for mem");
            },
            _ = cancellationtoken.cancelled() => {
                break;
            }
        }
    }
}

async fn Async_Heat(handle : AppHandle, app : TheApp, cancellationtoken : CancellationToken) {
    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs_f32(app.Adjustments.CHECKINTERVAL)) => {
                println!("test for heat");
            },
            _ = cancellationtoken.cancelled() => {
                break;
            }
        }
    }
}

#[tauri::command]
async fn get_sounds(handle : AppHandle) -> Vec<TheSound> {
    let mut path = handle.path().app_local_data_dir().expect("Got an error while trying to get app_local_data_dir");
    path.push("Sounds/");
    let mut sound_vector : Vec<TheSound> = vec![];
    for entry in read_dir(path).expect("Got an error while trying to read sounds dir"){
        let dir = entry.expect("Got an error while trying to get DirEntry");
        let filematcher = infer::get_from_path(dir.path().as_path());
        match  filematcher{
            Ok(Some(info)) => {
                if info.mime_type() == "audio/mpeg"{
                    sound_vector.push(TheSound { SoundPath: dir.path().as_path().to_string_lossy().to_string(), SoundName: dir.path().file_name().unwrap().to_string_lossy().into_owned() });
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
async fn notifysend(title : String, msg : String){
    match tokio::process::Command::new("notify-send").arg(title).arg(msg).output().await {
        Ok(output) => {
            println!("notify-send exit: {}", output.status);
            if !output.stdout.is_empty() {
                println!("notify-send stdout: {}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprintln!("notify-send stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => eprintln!("notify-send fallback failed to run: {}", e),
    }
}


#[tauri::command]
async fn get_sound_path(handle : AppHandle) -> String{
    let mut path = handle.path().app_local_data_dir().expect("s");

    path.push("Sounds/");

    return  path.to_string_lossy().to_string();
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

            app.manage(Mutex::new(AppState::default()));
            Ok(())

        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_sounds,
            get_apps,
            create,
            get_sound_path,
            notifysend
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
