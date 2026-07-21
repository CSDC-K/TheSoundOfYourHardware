import { invoke } from "@tauri-apps/api/core";


const SoundList = document.getElementById("soundlist") as HTMLSelectElement;
const SoundFXList = document.getElementById("soundfxlist") as HTMLSelectElement;

const MainDiv = document.getElementById("maindiv") as HTMLDivElement;
const DocDiv = document.getElementById("docdiv") as HTMLDivElement;

const MainTitle = document.getElementById("maintitle") as HTMLHeadElement;
const AdjDivTitle = document.getElementById("adjdivtitle") as HTMLHeadElement;

const DocBtn = document.getElementById("docbtn") as HTMLButtonElement;
const PlaySound = document.getElementById("playsound") as HTMLButtonElement;
const RefreshSounds = document.getElementById("refreshsounds") as HTMLButtonElement;
const DocCloseBtn = document.getElementById("doc_close_btn") as HTMLButtonElement;
const StartBtn = document.getElementById("start") as HTMLButtonElement; 
const ReZeroBtn = document.getElementById("rezero") as HTMLButtonElement;

const PlaySoundImg = document.getElementById("playimg") as HTMLImageElement;
const StartStopImg = document.getElementById("start_stop_img") as HTMLImageElement;

const CpuRateLimit = document.getElementById("cpuratelimit") as HTMLInputElement;
const RamRateLimit = document.getElementById("ramratelimit") as HTMLInputElement;
const HeatRateLimit = document.getElementById("heatratelimit") as HTMLInputElement;
const SoundLevel = document.getElementById("soundlevel") as HTMLInputElement;
const CheckInterval = document.getElementById("checkinterval") as HTMLInputElement;
const Label1 = document.getElementById("label1") as HTMLLabelElement;
const Label2 = document.getElementById("label2") as HTMLLabelElement;
const Label3 = document.getElementById("label3") as HTMLLabelElement;
const Label4 = document.getElementById("label4") as HTMLLabelElement;
const Label7 = document.getElementById("label7") as HTMLLabelElement;
const Label8 = document.getElementById("label8") as HTMLLabelElement;
const Label9 = document.getElementById("label9") as HTMLLabelElement;


var SoundLevelValue = SoundLevel.value;
var PlayingSound = true;
var PressedCreateBtn : boolean = false;

enum SoundFx{
    NONE = "NONE",
    SPEED125X = "SPEED125X",
    SPEED150X = "SPEED150X",
    SPEED175X = "SPEED175X",
    SPEED200X = "SPEED200X",
    SPEED250X = "SPEED250X",
    SPEED300X = "SPEED300X",
    BASSBOOST1 = "BASSBOOST1",
    BASSBOOST2 = "BASSBOOST2",
    BASSBOOST3 = "BASSBOOST3",
    SLOWDOWN75X = "SLOWDOWN75X",
    SLOWDOWN50X = "SLOWDOWN50X",
    SLOWDOWN25X = "SLOWDOWN25X",
    SLOWDOWN15X = "SLOWDOWN15X",
}

interface TheSound{
  SoundPath : string,
  SoundName : string, 
}




type Limits = 
 | { Limit: "Cpu", LimitValue: { Rate : Number } }
 | { Limit: "Memory", LimitValue: { Rate : Number } }
 | { Limit: "Heat", LimitValue: { Rate : Number } }

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


async function Start() {

  const selectedLimits : Limits[] = [];

  if(+CpuRateLimit.value > 0){
    selectedLimits.push({
      Limit : "Cpu",
      LimitValue : {Rate : +CpuRateLimit.value},
    })
  }
  if(+RamRateLimit.value > 0){
    selectedLimits.push({
      Limit : "Memory",
      LimitValue : {Rate : +RamRateLimit.value},
    })
  }
  if(+HeatRateLimit.value > 0){
    selectedLimits.push({
      Limit : "Heat",
      LimitValue : {Rate : +HeatRateLimit.value},
    })
  }

  if(SoundList.value == "None"){
    await Notify("Error", "You have to select one sound!");
    return
  }

  if(selectedLimits.length == 0){
    await Notify("Error", "You have to select one or more than one limit!");
    return;
  }

  if(PressedCreateBtn == true){
    PressedCreateBtn = false;
    StartStopImg.src = "src/assets/play.png";
    PlaySound.disabled = false;
  }

  else if(PressedCreateBtn == false){
    PressedCreateBtn = true;
    StartStopImg.src = "src/assets/pause.png"
    PlaySound.disabled = true;
  }


  try{
    await invoke("create", {
      test : false,
      structState : {
        SoundPath : SoundList.value,
        Limits : selectedLimits,
        Adjustments : {
          SOUNDLEVEL : +SoundLevel.value,
          CHECKINTERVAL : +CheckInterval.value || 0.1,
          SOUNDFX : await get_soundfx(SoundFXList.value)
        },

      }
    })
  } catch (e) {
    await Notify("Error!", "" + e);
  }



}

async function ToZero() {
  CpuRateLimit.value = "0";
  RamRateLimit.value = "0";
  HeatRateLimit.value = "0";
  SoundLevel.value = "0";
  CheckInterval.value = "0";
  SoundList.selectedIndex = 0;
  SoundFXList.selectedIndex = 0;
  
}


async function Notify(Title : String, Message : String) {
  await invoke("notifysend", {
    title : Title,
    msg : Message
  })
}

async function get_soundfx(Soundfx:String) {
  if (Soundfx == "1.25x") {
    return SoundFx.SPEED125X
  }
  if (Soundfx == "1.5x") {
    return SoundFx.SPEED150X
  }
  if (Soundfx == "1.75x") {
    return SoundFx.SPEED175X
  }
  if (Soundfx == "2.0x") {
    return SoundFx.SPEED200X
  }
  if (Soundfx == "2.5x") {
    return SoundFx.SPEED250X
  }
  if (Soundfx == "3.0x") {
    return SoundFx.SPEED300X
  }
  if (Soundfx == "B1") {
    return SoundFx.BASSBOOST1
  }
  if (Soundfx == "B2") {
    return SoundFx.BASSBOOST2
  }
  if (Soundfx == "B3") {
    return SoundFx.BASSBOOST3
  }
  if (Soundfx == "0.75x") {
    return SoundFx.SLOWDOWN75X
  }
  if (Soundfx == "0.50x") {
    return SoundFx.SLOWDOWN50X
  }
  if (Soundfx == "0.25x") {
    return SoundFx.SLOWDOWN25X
  }
  if (Soundfx == "0.15x" || Soundfx == "0.15X") {
    return SoundFx.SLOWDOWN15X
  }
  else{
    return SoundFx.NONE
  }
}

async function LoadAnimations() {
  const staticElements = [MainTitle, AdjDivTitle, Label1, Label2, Label3, Label4, Label7, Label8, Label9];
  const dynamicElements = Array.from(document.querySelectorAll(".applist-value"));

  const allElements = [...staticElements, ...dynamicElements];

  allElements.forEach(el => {
    if (el) (el as HTMLElement).style.animation = "none";
  });

  void document.body.offsetHeight; 
  const animString = "FontColorer 3s alternate infinite, UpSide 1s ease-out forwards";
  allElements.forEach(el => {
    if (el) (el as HTMLElement).style.animation = animString;
  });
}

//(window as any).GrowMenu = GrowMenu;

window.addEventListener("DOMContentLoaded", async () => {
  SoundList.length = 0;
  SoundList.add(new Option("Select a sound", "None"));
  
  try{
    const SoundsList = await invoke<TheSound[]>("get_sounds");
    for (const process of SoundsList) {
      SoundList.add(new Option(process.SoundName, process.SoundPath));
    }
  } catch (e) {
    console.error(e);
  }



  LoadAnimations()

});


CpuRateLimit.addEventListener("input", () => {
  Label2.textContent = "Cpu Limit : " + CpuRateLimit.value + "%";
});



RamRateLimit.addEventListener("input", () => {
  Label4.textContent = "Ram Limit : " + RamRateLimit.value + "%";
});

HeatRateLimit.addEventListener("input", () => {
  Label3.textContent = "Heat : " + HeatRateLimit.value + "℃";
});

SoundLevel.addEventListener("input", () => {
  SoundLevelValue = SoundLevel.value;
  if (+SoundLevelValue > 100) {
    Label7.textContent = "Sound Level : " + SoundLevel.value + "% (DAYYUM 🥀)";
    return;
  }

  Label7.textContent = "Sound Level : " + SoundLevel.value + "%";
});

CheckInterval.addEventListener("input", () => {
  Label8.textContent = "Check Interval : " + CheckInterval.value + "S";
});

PlaySound.addEventListener("click", async () => {
  if (PlayingSound == true){
    if(SoundList.value == "None"){
      await Notify("Error", "You have to select one sound!");
      return
    }
    PlaySoundImg.src = "src/assets/pause.png";
    PlayingSound = false;
    StartBtn.disabled = true;
    
    try{
      const selectedLimits : Limits[] = [];
      selectedLimits.push({
        Limit : "Memory",
        LimitValue : {Rate : +RamRateLimit.value},
      })
      await invoke("create", {
        test : true,
        structState : {
          SoundPath : SoundList.value,
          Limits : selectedLimits,
          Adjustments : {
            SOUNDLEVEL : +SoundLevel.value,
            CHECKINTERVAL : +CheckInterval.value || 0.1,
            SOUNDFX : await get_soundfx(SoundFXList.value)
          }
        }
      })
    } catch (e) {
      await Notify("Error!", "" + e);
    }
  } else if (PlayingSound == false){
    PlaySoundImg.src = "src/assets/play.png";
    PlayingSound = true;
    StartBtn.disabled = false;
    try{
      const selectedLimits : Limits[] = [];
      selectedLimits.push({
        Limit : "Memory",
        LimitValue : {Rate : +RamRateLimit.value},
      })
      await invoke("create", {
        test : true,
        structState : {
          SoundPath : SoundList.value,
          Limits : selectedLimits,
          Adjustments : {
            SOUNDLEVEL : +SoundLevel.value,
            CHECKINTERVAL : +CheckInterval.value || 0.1,
            SOUNDFX : await get_soundfx(SoundFXList.value)
          }
        }
      })
    } catch (e) {
      await Notify("Error!", "" + e);
    }
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

DocCloseBtn.addEventListener("click", () => {
  MainDiv.style.display = "block";
  DocDiv.style.display = "none";
  LoadAnimations();
});



DocBtn.addEventListener("click", () => {
  MainDiv.style.display = "none";
  DocDiv.style.display = "block";
});

ReZeroBtn.addEventListener("click", async () => {
  await ToZero();
});

StartBtn.addEventListener("click", async () => {
  await Start();
});