import {Core} from "mech-wasm";

// ## Websocket 

/*
let host = location.hostname == "" ? "localhost" : location.hostname;
let ws = new WebSocket(`ws://${host}:3012`);

ws.addEventListener("open", () => opened());
ws.addEventListener("close", (event) => closed(event.code, event.reason));
ws.addEventListener("message", (event) => messaged(event.data));

function opened() {
  console.log(ws);
  // Get code
  ws.send("{\"Table\":3436366081}");
}

function closed(code, reason) {
  console.log(code, reason);
}

function messaged(data) {
  var obj = JSON.parse(data);
  let code = obj[0][0].String;
  let code_editor = document.getElementById("code");
  code_editor.innerHTML = code;

  // Compile the code
  mech_core.compile_code(code);
  mech_core.add_application();

  // Start the timer if there is one
  let column = mech_core.get_column("system/timer", 1);
  interval = setInterval(system_timer, column[0]);
}*/

let mech_core = Core.new(100000, 100);
var interval;

let time = 1;

// ## Controls

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
txn.innerHTML =  "Add Txn";

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

// ## Editor

let editor = document.createElement("div");
editor.setAttribute("class", "editor");

let code = document.createElement("textarea");
code.setAttribute("class", "code");
code.setAttribute("id", "code");
code.setAttribute("spellcheck", "false");

let drawing_area = document.createElement("div")
drawing_area.setAttribute("id", "drawing");
drawing_area.setAttribute("class", "drawing-area");

editor.appendChild(drawing_area)

// ## Editor Container

let editor_container = document.createElement("div");
editor_container.setAttribute("id","editor container");
editor_container.setAttribute("class","editor-container");

editor_container.appendChild(controls);
editor_container.appendChild(editor);
editor_container.appendChild(time_travel);

// ## Navigation

let nav = document.createElement("div");
nav.setAttribute("id","navigation");
nav.setAttribute("class","navigation");


// ## Bring it all together

/*let app = document.createElement("div");
app.setAttribute("id","app");
app.setAttribute("class","app");
app.appendChild(nav);
app.appendChild(code);
app.appendChild(editor_container);*/

let app = document.createElement("div");
app.setAttribute("id","mech-app");
app.onclick = function(event) {
  console.log(event);
}
app.innerHTML = `
<h1># Documentation</h1>
<h2>Introduction</h2>
<ul>
  <li><a href="/#/docs/tutorial.mec">tutorial</a></li>
</ul>
<h2>Math</h2>
<ul>
  <li><a href="/#/docs/math/sin.mec">math/sin</a></li>
  <li><a href="/#/docs/math/cos.mec">math/cos</a></li>
</ul>
<h2>Examples</h2>
<ul>
  <li><a href="/#/examples/breakout.mec">breakout.mec</a></li>
  <li><a href="/#/examples/clock.mec">clock.mec</a></li>
  <li><a href="/#/examples/robot-drawing.mec">robot-drawing.mec</a></li>
</ul>
`;

// ## Event handlers
function system_timer() {
  var d = new Date();
  mech_core.queue_change("system/timer",1,2,time);
  mech_core.queue_change("system/timer",1,3,d.getHours() % 12);
  mech_core.queue_change("system/timer",1,4,d.getMinutes());
  mech_core.queue_change("system/timer",1,5,d.getSeconds());
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
  let column = mech_core.get_column("system/timer", 1);
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

window.onhashchange = function(event) {
  document.body.innerHTML = "";
  console.log();
  let extension = location.hash.substring(location.hash.length - 3);
  var url = location.hash.substring(1);
  if (extension == "mec") {
    var xhr = new XMLHttpRequest();
    console.log(url);
    xhr.open('GET', url, true);
    xhr.send();
  
    xhr.onreadystatechange = processRequest;
  } else if (url == "") {
    document.body.appendChild(app);
  } else {
    window.location = event.newURL;
  }

  function processRequest(e) {
    if (xhr.readyState == 4 && xhr.status == 200) {
      clearInterval(interval);
      let program = xhr.responseText;
      console.log(program);
      //code.innerHTML = program;
      mech_core.compile_code(program);
      mech_core.add_application();
      // Start the timer if there is one
      let column = mech_core.get_column("system/timer", 1);
      if (column[0] != undefined) {
        interval = setInterval(system_timer, column[0]);
      }
    }
  }

}

let extension = location.hash.substring(location.hash.length - 3);
var url = location.hash.substring(1);
if (extension == "mec") {
  var xhr = new XMLHttpRequest();
  var url = location.hash.substring(1);
  xhr.open('GET', url, false);
  xhr.send();
  let program = xhr.responseText;
  mech_core.compile_code(program);
  mech_core.add_application();
  // Start the timer if there is one
  let column = mech_core.get_column("system/timer", 1);
  if (column[0] != undefined) {
    interval = setInterval(system_timer, column[0]);
  }
} else if (url == "") {
  window.location = "/#/docs/index.mec";
} else {
  window.location = event.newURL;
}