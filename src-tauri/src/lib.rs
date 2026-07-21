use std::collections::HashMap;
use std::default;
use rodio::{Decoder, DeviceSinkBuilder, Player, Source};
use std::sync::Mutex;
use std::fs::{File, create_dir_all, read_dir};
use infer;
use tokio::time::{sleep, Duration};
use tauri::{AppHandle, Manager};
use sysinfo::{self, Components, CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System};

use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;


#[derive(Default)]
struct AppState{
    watcher_key : HashMap<String, CancellationToken>,
    player_key : HashMap<String, CancellationToken>,
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
    BASSBOOST1,
    BASSBOOST2,
    BASSBOOST3,
    SLOWDOWN75X,
    SLOWDOWN50X,
    SLOWDOWN25X,
    SLOWDOWN15X,
}

fn soundfx_profile(soundfx: &SoundFx) -> (f32, f32) {
    match soundfx {
        SoundFx::NONE => (1.0, 1.0),
        SoundFx::SPEED125X => (1.25, 1.0),
        SoundFx::SPEED150X => (1.50, 1.0),
        SoundFx::SPEED175X => (1.75, 1.0),
        SoundFx::SPEED200X => (2.00, 1.0),
        SoundFx::SPEED250X => (2.50, 1.0),
        SoundFx::SPEED300X => (3.00, 1.0),
        SoundFx::BASSBOOST1 => (1.0, 1.15),
        SoundFx::BASSBOOST2 => (1.0, 1.25),
        SoundFx::BASSBOOST3 => (1.0, 1.35),
        SoundFx::SLOWDOWN75X => (0.75, 1.0),
        SoundFx::SLOWDOWN50X => (0.50, 1.0),
        SoundFx::SLOWDOWN25X => (0.25, 1.0),
        SoundFx::SLOWDOWN15X => (0.15, 1.0),
    }
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

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TheAppTest {
    SoundPath : String,
    Adjustments : Adjustments
}


impl TheApp {
    pub async fn create(self, handle : AppHandle) {

        let watcher_token = CancellationToken::new();
        let player_token = CancellationToken::new();
        let active_token = watcher_token.clone();
        let active_player_token = player_token.clone();
        let worker_handle = handle.clone();

        tokio::spawn(async move {
            watch_hardware(active_token, active_player_token, self, worker_handle).await;
        });

        let state = handle.state::<Mutex<AppState>>();
        let mut app_state = state.lock().unwrap();
        app_state.watcher_key.insert(format!("watcher_key"), watcher_token);
        app_state.player_key.insert(format!("player_key"), player_token);
        app_state.thestate = true;
    }
}

impl TheAppTest {
    pub async fn test_sound(self, handle : AppHandle) {
        let watcher_token = CancellationToken::new();
        let player_token = CancellationToken::new();
        let active_token = watcher_token.clone();
        let active_player_token = player_token.clone();
        let worker_handle = handle.clone();

        tokio::spawn(async move{
            playsound(worker_handle, TheApp { SoundPath: self.SoundPath, Limits: vec![Limits::Cpu { Rate: 0 }], Adjustments: self.Adjustments }, active_player_token).await;
        });

        let state = handle.state::<Mutex<AppState>>();
        let mut app_state = state.lock().unwrap();
        app_state.player_key.insert(format!("player_key"), player_token);
        app_state.thestate = true;
    }
}

#[tauri::command]
async fn create(
    handle : AppHandle,
    StructState : TheApp,
    test : bool
) -> Result<(), String>{
    let mut mutablehandle = handle.clone();
    let should_stop = {
        let state = handle.state::<Mutex<AppState>>();
        let mut app_state = state.lock().unwrap();
        app_state.thestate
    };

    println!("STATE : {}", should_stop);

    if should_stop == true{
        let _ = stop(&mut mutablehandle).await;
        notifysend(format!("Stop"), format!("App was already running in background, now it's stopped.")).await;
        return Ok(());
    }

    else if test == true{
        let mut should_stop = {
            let state = handle.state::<Mutex<AppState>>();
            let mut app_state = state.lock().unwrap();
            app_state.thestate = true;
        };

        let TheAppStruct : TheAppTest = TheAppTest { SoundPath: StructState.SoundPath, Adjustments: StructState.Adjustments };
        TheAppStruct.test_sound(handle.clone()).await;
        
        return Ok(());
    }

    else{
        println!("SoundPath : {}", StructState.SoundPath);
        println!("Limits : {:?}", StructState.Limits);
        println!("Adjustments-SOUNDFX : {:?}", StructState.Adjustments.SOUNDFX); 
        println!("Adjustments-CHECKINTERVAL : {}", StructState.Adjustments.CHECKINTERVAL);
        println!("Adjustments-SOUNDLEVEL : {}", StructState.Adjustments.SOUNDLEVEL);
        let mut should_stop = {
            let state = handle.state::<Mutex<AppState>>();
            let mut app_state = state.lock().unwrap();
            app_state.thestate = true;
        };

        let TheAppStruct : TheApp = TheApp { SoundPath: StructState.SoundPath, Limits: StructState.Limits, Adjustments: StructState.Adjustments };
        TheAppStruct.create(handle.clone()).await;
        
        Ok(())
    }


}


#[tauri::command]
async fn stop(handle : &mut AppHandle) -> Result<(), String>{

    let watcher_token = {
        let state = handle.state::<Mutex<AppState>>();
        let mut app_state = state.lock().unwrap();
        app_state.thestate = false;
        app_state.watcher_key.remove("watcher_key")
    };

    let player_token = {
        let state = handle.state::<Mutex<AppState>>();
        let mut app_state = state.lock().unwrap();
        app_state.thestate = false;
        app_state.player_key.remove("player_key")
    };

    match watcher_token {
        Some(token) => {
            token.cancel();
        }
        None => println!("err"),
    };

    match player_token {
        Some(token) => {
            token.cancel();
            Ok(())
        }
        None => Ok(()),
    }

}

async fn stop_watcher_only(handle : &mut AppHandle) -> Result<(), String> {
    let watcher_token = {
        let state = handle.state::<Mutex<AppState>>();
        let mut app_state = state.lock().unwrap();
        app_state.watcher_key.remove("watcher_key")
    };

    match watcher_token {
        Some(token) => {
            token.cancel();
            Ok(())
        }
        None => Ok(()),
    }
}

async fn watch_hardware(cancellationtoken : CancellationToken, playercancellationtoken : CancellationToken,app : TheApp, handle : AppHandle){

    for limit in app.Limits.iter().cloned(){
        let WorkerStruct = app.clone();
        let mut WorkerHandle = handle.clone();
        let workertoken = cancellationtoken.clone();
        let workerplayertoken = playercancellationtoken.clone();
        match limit{
            Limits::Cpu { Rate } => {
                tokio::spawn(async move {
                    Async_Cpu( &mut WorkerHandle, WorkerStruct, workertoken, workerplayertoken, Rate).await
                });
            },
            Limits::Memory { Rate } => {
                tokio::spawn(async move {
                    Async_Memory( &mut WorkerHandle, WorkerStruct, workertoken, workerplayertoken, Rate).await
                });
            },
            Limits::Heat { Rate } => {
                tokio::spawn(async move {
                    Async_Heat( &mut WorkerHandle, WorkerStruct, workertoken, workerplayertoken,Rate).await
                });
            },
        }
    }
}


async fn Async_Cpu(handle : &mut AppHandle, app : TheApp, cancellationtoken : CancellationToken, playercancellationtoken : CancellationToken, rate : u8) {

    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
    );


    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs_f32(app.Adjustments.CHECKINTERVAL)) => {
                sys.refresh_cpu_all();
                let total_cpus = sys.cpus().len();
                let mut total_cpu_usage = 0.0;
                for cpu in sys.cpus(){
                    println!("{} : {}%",cpu.name(), cpu.cpu_usage());
                    total_cpu_usage = total_cpu_usage + cpu.cpu_usage();
                }

                total_cpu_usage = total_cpu_usage / total_cpus as f32;
                println!("Total Usage : {}", total_cpu_usage);
                if total_cpu_usage > rate as f32 {
                    let mut mutablehandle = handle.clone();
                    let p_handle = handle.clone();
                    let p_app = app.clone();
                    let p_token = playercancellationtoken.clone();
                    tokio::spawn(async move {
                        let _ = playsound(p_handle, p_app, p_token).await;
                    });
                    let _ = notifysend(
                        String::from("Hardware limit reached"),
                        format!("CPU usage crossed the configured limit: {}%", rate)
                    ).await;
                    let _ = stop_watcher_only(&mut mutablehandle).await;
                    break;
                }

            },

            _ = cancellationtoken.cancelled() => {
                break;
            }
        }
    }
}


async fn Async_Memory(handle : &mut AppHandle, app : TheApp, cancellationtoken : CancellationToken, playercancellationtoken : CancellationToken, rate : u8) {

    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::everything().with_ram())
    );

    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs_f32(app.Adjustments.CHECKINTERVAL)) => {
                sys.refresh_memory();

                let used_memory = sys.used_memory() / 1_000000000;
                let total_memory = sys.total_memory() / 1_000000000;
                println!("test : {}", (total_memory * rate as u64) / 100);
                if used_memory > (total_memory * rate as u64) / 100{
                    let mut mutablehandle = handle.clone();
                    let p_handle = handle.clone();
                    let p_app = app.clone();
                    let p_token = playercancellationtoken.clone();
                    tokio::spawn(async move {
                        let _ = playsound(p_handle, p_app, p_token).await;
                    });
                    let _ = notifysend(
                        String::from("Hardware limit reached"),
                        format!("Memory usage crossed the configured limit: {}%", rate)
                    ).await;
                    let _ = stop_watcher_only(&mut mutablehandle).await;
                    break;
                }

            },
            _ = cancellationtoken.cancelled() => {
                break;
            }
        }
    }
}

async fn Async_Heat(handle : &mut AppHandle, app : TheApp, cancellationtoken : CancellationToken, playercancellationtoken : CancellationToken,rate : u8) {
    let mut components = Components::new_with_refreshed_list();
    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs_f32(app.Adjustments.CHECKINTERVAL)) => {
                components.refresh(false);
                for component in components.list().iter(){
                    let label = component.label().to_lowercase();
                    if label.contains("tctl") || label.contains("package") { //k10temp for amd cpus and coretemp for intel cpus.
                        println!("HEAT : {:?}", component.temperature());
                        if component.temperature().unwrap() > rate as f32{
                            let mut mutablehandle = handle.clone();
                            let p_handle = handle.clone();
                            let p_app = app.clone();
                            let p_token = playercancellationtoken.clone();
                            tokio::spawn(async move {
                                let _ = playsound(p_handle, p_app, p_token).await;
                            });
                            let _ = notifysend(
                                String::from("Hardware limit reached"),
                                format!("Temperature crossed the configured limit: {}°C", rate)
                            ).await;
                            let _ = stop_watcher_only(&mut mutablehandle).await;
                            break;
                        }
                    }
                }
            },
            _ = cancellationtoken.cancelled() => {
                break;
            }
        }
    }
}



pub async fn playsound(handle: AppHandle, app: TheApp, cancel_token: CancellationToken) -> Result<(), String> {
    let sound_path = app.SoundPath.clone();
    let sound_level = app.Adjustments.SOUNDLEVEL as f32 / 100.0;
    let soundfx = app.Adjustments.SOUNDFX.clone();

    tokio::task::spawn_blocking(move || {
        let file = File::open(&sound_path)
            .map_err(|e| format!("Failed to open sound file '{}': {}", sound_path, e))?;

        let sink_handle = DeviceSinkBuilder::open_default_sink()
            .map_err(|e| format!("Failed to open audio output: {}", e))?;

        let player = Player::connect_new(&sink_handle.mixer());

        let source = Decoder::try_from(file)
            .map_err(|e| format!("Failed to decode sound file '{}': {}", sound_path, e))?;


        match soundfx {

            SoundFx::SPEED125X => player.append(source.speed(1.25).amplify(sound_level)),
            SoundFx::SPEED150X => player.append(source.speed(1.50).amplify(sound_level)),
            SoundFx::SPEED175X => player.append(source.speed(1.75).amplify(sound_level)),
            SoundFx::SPEED200X => player.append(source.speed(2.00).amplify(sound_level)),
            SoundFx::SPEED250X => player.append(source.speed(2.50).amplify(sound_level)),
            SoundFx::SPEED300X => player.append(source.speed(3.00).amplify(sound_level)),

            SoundFx::SLOWDOWN75X => player.append(source.speed(0.75).amplify(sound_level)),
            SoundFx::SLOWDOWN50X => player.append(source.speed(0.50).amplify(sound_level)),
            SoundFx::SLOWDOWN25X => player.append(source.speed(0.25).amplify(sound_level)),
            SoundFx::SLOWDOWN15X => player.append(source.speed(0.15).amplify(sound_level)),

            SoundFx::BASSBOOST1 | SoundFx::BASSBOOST2 | SoundFx::BASSBOOST3 => {
                let boost_factor = match soundfx {
                    SoundFx::BASSBOOST1 => 1.8,
                    SoundFx::BASSBOOST2 => 2.8,
                    SoundFx::BASSBOOST3 => 4.2,
                    _ => 1.0,
                };

                let buffered = source.buffered();
                let bass = buffered.clone().low_pass(180).amplify(boost_factor);
                let mixed_source = buffered.mix(bass).amplify(sound_level);

                player.append(mixed_source);
            }

            SoundFx::NONE => player.append(source.amplify(sound_level)),
        }
        while !player.empty() && !player.is_paused() {
            if cancel_token.is_cancelled() {
                let state_change = {
                    println!("PLAYER_SHUTTED");
                    let state = handle.state::<Mutex<AppState>>();
                    let mut app_state = state.lock().unwrap();
                    app_state.thestate = false;
                    app_state.watcher_key.remove("watcher_key")
                };
                player.stop();
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }

        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("Audio playback task failed: {}", e))?
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
