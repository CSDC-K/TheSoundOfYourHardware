import { invoke } from "@tauri-apps/api/core";


const SoundList = document.getElementById("soundlist") as HTMLSelectElement;
const MainDiv = document.getElementById("maindiv") as HTMLDivElement;
const AppDiv = document.getElementById("appdiv") as HTMLDivElement;
const AppCentralDiv = document.getElementById("appcentraldiv") as HTMLDivElement;
const MainTitle = document.getElementById("maintitle") as HTMLHeadElement;

const PlaySound = document.getElementById("playsound") as HTMLButtonElement;
const RefreshSounds = document.getElementById("refreshsounds") as HTMLButtonElement;
const AppList = document.getElementById("applist") as HTMLButtonElement;
const AppCloseBtn = document.getElementById("app_close_btn") as HTMLButtonElement;

const PlaySoundImg = document.getElementById("playimg") as HTMLImageElement;

const CpuRateLimit = document.getElementById("cpuratelimit") as HTMLInputElement;
const GpuRateLimit = document.getElementById("gpuratelimit") as HTMLInputElement;
const RamRateLimit = document.getElementById("ramratelimit") as HTMLInputElement;
const HeatRateLimit = document.getElementById("heatratelimit") as HTMLInputElement;
const Label1 = document.getElementById("label1") as HTMLLabelElement;
const Label2 = document.getElementById("label2") as HTMLLabelElement;
const Label3 = document.getElementById("label3") as HTMLLabelElement;
const Label4 = document.getElementById("label4") as HTMLLabelElement;
const Label5 = document.getElementById("label5") as HTMLLabelElement;
const Label6 = document.getElementById("label6") as HTMLLabelElement;

var HeatRateLimitValue = HeatRateLimit.value;
var PlayingSound = false;

//async function RefreshApps() {
//  console.log("EVENT: refresh_apps")
//  AppList.length = 0;
//  invoke("get_apps").then((apps) => {
//    const Apps = apps as string[];
//    for (const app of Apps) {
//      AppList.add(new Option(app, app));
//    }
//  });
//}

async function LoadAnimations() {
  MainTitle.style.animation = "FontColorer 3s alternate infinite, UpSide 1s ease-out forwards";
  Label1.style.animation = "FontColorer 3s alternate infinite, UpSide 1s ease-out forwards";
  Label2.style.animation = "FontColorer 3s alternate infinite, UpSide 1s ease-out forwards";
  Label3.style.animation = "FontColorer 3s alternate infinite, UpSide 1s ease-out forwards";
  Label4.style.animation = "FontColorer 3s alternate infinite, UpSide 1s ease-out forwards";
  Label5.style.animation = "FontColorer 3s alternate infinite, UpSide 1s ease-out forwards";
  Label6.style.animation = "FontColorer 3s alternate infinite, UpSide 1s ease-out forwards";
}

//(window as any).GrowMenu = GrowMenu;

window.addEventListener("DOMContentLoaded", () => {

  LoadAnimations()

});


CpuRateLimit.addEventListener("input", () => {
  Label2.textContent = "Cpu Limit : " + CpuRateLimit.value + "%";
});

GpuRateLimit.addEventListener("input", () => {
  Label3.textContent = "Cpu Limit : " + GpuRateLimit.value + "%";
});

RamRateLimit.addEventListener("input", () => {
  Label4.textContent = "Cpu Limit : " + RamRateLimit.value + "%";
});

HeatRateLimit.addEventListener("input", () => {
  HeatRateLimitValue = HeatRateLimit.value;
  if (+HeatRateLimitValue > 80) {
    Label5.textContent = "Heat Limit : " + HeatRateLimit.value + "℃ (DAYYUM 🥀)";
    return;
  }

  Label5.textContent = "Heat Limit : " + HeatRateLimit.value + "℃";
});

PlaySound.addEventListener("click", () => {
  if (PlayingSound == true){
    PlaySoundImg.src = "src/assets/pause.png";
    PlayingSound = false;
  } else if (PlayingSound == false){
    PlaySoundImg.src = "src/assets/play.png";
    PlayingSound = true;
  }
});

RefreshSounds.addEventListener("click", async () => {
  SoundList.length = 0;
  SoundList.add(new Option("Select a sound", "None"));
  await invoke("get_sounds").then((sounds) => {
    const Sounds = sounds as string[];
    for (const sound of Sounds) {
      SoundList.add(new Option(sound, sound));
    }
  });
});


AppList.addEventListener("click", () => {
  MainDiv.style.display = "none"
});

AppCloseBtn.addEventListener("click", () =>{
  MainDiv.style.display = "block"
});