import {Core} from "mech-wasm";

let mech_core = Core.new(100000, 100);
mech_core.connect_remote_core("ws://localhost:3012");
var interval;

let time = 1;

// ## Controls
/*
let controls = document.createElement("div");
controls.setAttribute("class", "controls");

let compile = document.createElement("button");
compile.setAttribute("id", "compile");
compile.innerHTML =  "Compile";
controls.appendChild(compile);

let view_core = document.createElement("button");
view_core.setAttribute("id", "view core");
view_core.innerHTML =  "View Core";
controls.appendChild(view_core);

let view_runtime = document.createElement("button");
view_runtime.setAttribute("id", "view runtime");
view_runtime.innerHTML =  "View Runtime";
controls.appendChild(view_runtime);

let clear_core = document.createElement("button");
clear_core.setAttribute("id", "clear core");
clear_core.innerHTML =  "Clear Core";
controls.appendChild(clear_core);

let txn = document.createElement("button");
txn.setAttribute("id", "txn");
txn.innerHTML =  "Add Txn";*/

// ## Time Travel

function resume() {
  let toggle_core = document.getElementById("toggle core");
  let time_slider = document.getElementById("time slider");
  mech_core.resume();
  toggle_core.innerHTML = "Pause";
  time_slider.value = time_slider.max;
  render();
}

function pause() {
  let toggle_core = document.getElementById("toggle core");
  mech_core.pause();
  toggle_core.innerHTML = "Resume";
  render();
}

let time_travel = document.createElement("div");
time_travel.setAttribute("class", "time-travel");

let time_slider = document.createElement("input");
time_slider.setAttribute("id", "time slider");
time_slider.setAttribute("class", "slider");
time_slider.setAttribute("min", "1");
time_slider.setAttribute("max", "100");
time_slider.setAttribute("value", "100");
time_slider.setAttribute("type", "range");
time_travel.appendChild(time_slider);

let last_slider_value = 100;
time_slider.oninput = function() {
  pause();
  let current_value = this.value;
  mech_core.set_time(100 - current_value);
  render();
}

let step_back = document.createElement("button");
step_back.setAttribute("id", "step back");
step_back.innerHTML =  "<";
step_back.onclick = function() {
  pause();
  mech_core.step_back_one();
  time_slider.value = time_slider.value - 1;
  render();
}
time_travel.appendChild(step_back);

let toggle_core = document.createElement("button");
toggle_core.setAttribute("id", "toggle core");
toggle_core.innerHTML =  "Pause";
toggle_core.onclick = function() {
  let toggle_core = document.getElementById("toggle core");
  let state = toggle_core.innerHTML;
  if (state == "Resume") {
    resume();
  } else {
    pause();
  }
  render();
};
time_travel.appendChild(toggle_core);

let step_forward = document.createElement("button");
step_forward.setAttribute("id", "step forward");
step_forward.innerHTML =  ">";
step_forward.onclick = function() {
  pause();
  mech_core.step_forward_one();
  time_slider.value = time_slider.value*1 + 1;
  render();
}
time_travel.appendChild(step_forward);

// ## Editor Container

let container = document.createElement("div");
container.setAttribute("id","mech-container");
container.setAttribute("class","mech-container");

let editor_container = document.createElement("div");
editor_container.setAttribute("id","mech-editor-container");
editor_container.setAttribute("class","mech-editor-container");

container.appendChild(editor_container);
container.appendChild(time_travel);

// ## Navigation

let controls = document.createElement("div");
controls.setAttribute("id","mech-controls");
controls.setAttribute("class","mech-controls");

// Code
let code_button = document.createElement("a");
code_button.setAttribute("class", "mech-control ion-pound");
code_button.setAttribute("href", "/#/docs/index.mec");
controls.appendChild(code_button);

// Tables
let tables_button = document.createElement("a");
tables_button.setAttribute("class", "mech-control ion-grid");
tables_button.setAttribute("href", "/#/tables/index.mec");
controls.appendChild(tables_button);

// Documentation
let docs_button = document.createElement("a");
docs_button.setAttribute("class", "mech-control ion-ios-bookmarks");
docs_button.setAttribute("href", "/#/docs/index.mec");
controls.appendChild(docs_button);

// ## Bring it all together

let app = document.createElement("div");
app.setAttribute("id","mech-app");
app.setAttribute("class","mech-app");
app.appendChild(controls);
app.appendChild(container);

document.body.appendChild(app);

// ## Event handlers
function system_timer() {
  var d = new Date();
  mech_core.queue_change("time/timer",1,2,time);
  mech_core.queue_change("time/timer",1,3,d.getHours() % 12);
  mech_core.queue_change("time/timer",1,4,d.getMinutes());
  mech_core.queue_change("time/timer",1,5,d.getSeconds());
  mech_core.process_transaction();
  time = time + 1;
  render();
}

function render() {
  mech_core.render();
}
/*
document.getElementById("compile").addEventListener("click", function(click) {
  mech_core.clear();
  clearInterval(interval);

  let code = document.getElementById("code");
  mech_core.compile_code(code.value);
  mech_core.add_application();

  // Start the timer if there is one
  let column = mech_core.get_column("time/timer", 1);
  if (column[0] != undefined) {
    interval = setInterval(system_timer, column[0]);
  }
});

document.getElementById("view core").addEventListener("click", function() {
  mech_core.display_core();
  mech_core.list_global_tables();
});

document.getElementById("view runtime").addEventListener("click", function() {
  mech_core.display_runtime();
});

document.getElementById("clear core").addEventListener("click", function() {
  mech_core.clear();
  clearInterval(interval);
  //render();
});*/

document.onkeyup = function (event) {
  if (event.altKey) {
    switch (event.key) {
      case "1": window.location = "/#/examples/clock.mec"; break;
      case "2": window.location = "/#/examples/breakout.mec"; break;
      case "3": window.location = "/#/examples/bouncing-balls.mec"; break;
      case "4": window.location = "/#/examples/robot-drawing.mec"; break;
      case "5": window.location = "/#/examples/error.mec"; break;
      case "6": window.location = "/#/examples/units.mec"; break;
      case "7": window.location = "/#/docs/math/sin.mec"; break;
    }
    
  }
}

window.onhashchange = function(event) {
  let extension = location.hash.substring(location.hash.length - 3);
  var url = location.hash.substring(1);
  if (extension == "mec") {
    var xhr = new XMLHttpRequest();
    console.log(url);
    xhr.open('GET', url, true);
    xhr.send();
  
    xhr.onreadystatechange = processRequest;
  } else if (url == "") {
    clearInterval(interval);
    window.location = "/#/docs/index.mec";
  } else {
    window.location = event.newURL;
  }

  function processRequest(e) {
    if (xhr.readyState == 4 && xhr.status == 200) {
      let editor = document.getElementById("mech-editor-container");  
      editor.innerHTML = "";
      clearInterval(interval);
      let program = xhr.responseText;
      console.log(program);
      //code.innerHTML = program;
      mech_core.clear();
      mech_core.compile_code(program);
      mech_core.add_application();
      // Start the timer if there is one
      let column = mech_core.get_column("time/timer", 1);
      if (column[0] != undefined) {
        interval = setInterval(system_timer, column[0]);
      }
    }
  }
  
}

let extension = location.hash.substring(location.hash.length - 3);
var url = location.hash.substring(1);
if (extension == "mec") {
  clearInterval(interval);
  var xhr = new XMLHttpRequest();
  var url = location.hash.substring(1);
  xhr.open('GET', url, false);
  xhr.send();
  let program = xhr.responseText;
  console.log(program);
  
  mech_core.clear();
  mech_core.compile_code(program);
  mech_core.add_application();
  // Start the timer if there is one
  let column = mech_core.get_column("time/timer", 1);
  if (column[0] != undefined) {
    interval = setInterval(system_timer, column[0]);
  }
} else if (url == "") {
  clearInterval(interval);
  window.location = "/#/docs/index.mec";
} else {
  window.location = event.newURL;
}