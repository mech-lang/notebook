import "setimmediate";
import {Program, Library, createId, RawValue, RawChange, RawMap, handleTuples, handleRecords} from "../../src";

const EMPTY:never[] = [];

export interface Instance extends HTMLElement {
  __element: RawValue,
  __source: HTML,
  //__styles?: RawValue[],
  //__sort?:RawValue,
  //__autoSort?:RawValue,
  //__listeners?: {[event:string]: boolean},
  //__capturedKeys?: {[code:number]: boolean}
}

export class HTML extends Library {
  static id = "html";

  /** Topmost element containing root elements. */
    _container:HTMLElement;
    _canvas: HTMLCanvasElement;
    _editor: HTMLTextAreaElement;
  /** Instances are the physical DOM elements representing table elements. */
  _instances: Instance[][] = [];
  _paths: any[][] = []; 
  control: Boolean;
  enter: Boolean;

  setup() {
    // If we're not in a browser environment, this library does nothing
    if(typeof document === "undefined") {
      this.handlers = {} as any;
      return;
    }

    this.control = false;
    this.enter = false;

    let spacer = document.createElement("div");
    spacer.setAttribute("class","spacer");

    let header = document.createElement("div");
    header.setAttribute("class","header");

    let code = document.createElement("textarea");
    this._editor = code;
    code.setAttribute("class","code");
    document.addEventListener('dblclick', function(e){ e.preventDefault(); }, false);

    this._container = document.createElement("div");
    this._container.setAttribute("program", this.program.name);
    this._container.setAttribute("class","container");
    document.body.appendChild(this._container);

    let controls = document.createElement("div");
    controls.setAttribute("class", "controls");

    let editor = document.createElement("div");
    editor.setAttribute("class", "editor");

    let reset = document.createElement("button");
    reset.setAttribute("id", "controls-reset");
    reset.innerHTML =  "Runtime";

    let stop = document.createElement("button");
    stop.setAttribute("id", "controls-stop");
    stop.innerHTML =  "Stop";
    controls.appendChild(stop);

    let step_back = document.createElement("button");
    step_back.setAttribute("id", "controls-step-back");
    step_back.innerHTML =  "Step Back";
    controls.appendChild(step_back);    

    let step_forward = document.createElement("button");
    step_forward.setAttribute("id", "controls-step-forward");
    step_forward.innerHTML =  "Step Forward";
    controls.appendChild(step_forward);    

    let pause = document.createElement("button");
    pause.setAttribute("id", "controls-pause");
    pause.innerHTML =  "Pause";
    controls.appendChild(pause);

    let resume = document.createElement("button");
    resume.setAttribute("id", "controls-resume");
    resume.innerHTML =  "Resume";
    controls.appendChild(resume);

    let clean = document.createElement("button");
    clean.setAttribute("id", "controls-clean");
    clean.innerHTML =  "Clean";
    controls.appendChild(clean);

    let database = document.createElement("button");
    database.setAttribute("id", "controls-database");
    database.innerHTML =  "Data";
    controls.appendChild(database);

    let history = document.createElement("button");
    history.setAttribute("id", "controls-history");
    history.innerHTML =  "History";
    controls.appendChild(history);

    controls.appendChild(reset);

    let link0 = new Image();
    link0.src = '/images/robotarm/link0.png';
    link0.style.transform = "translate(-250px, 100px) scale(0.5, 0.5)";
    link0.style.zIndex = "3";
    
    let link1 = new Image();
    link1.src = '/images/robotarm/link1.png';
    link1.style.transform = "translate(-30px, -100px) scale(0.5, 0.5) rotate(0deg)";
    link1.style.transformOrigin = "center bottom";
    link1.style.zIndex = "2";
    
    let link2 = new Image();
    link2.src = '/images/robotarm/link2.png';
    link2.style.transform = "translate(107px, -250px) scale(0.5, 0.5) rotate(120deg)";
    link2.style.transformOrigin = "center bottom";
    link2.style.zIndex = "1";

    //editor.appendChild(link2);
    //editor.appendChild(link1);
    //editor.appendChild(link0);
    
    let logo = new Image();
    logo.src = '/images/logo.png';
    logo.setAttribute("class", "logo");
    header.appendChild(logo);
    
    let canvas = this._canvas = document.createElement("canvas");
    canvas.setAttribute("class", "canvas");
    canvas.setAttribute("width", "500");
    canvas.setAttribute("height", "900");
    canvas.style.backgroundColor = 'rgb(226, 79, 94)';
    editor.appendChild(code);
    editor.appendChild(canvas);

    this._container.appendChild(logo);
    this._container.appendChild(controls);
    this._container.appendChild(editor);


    canvas.addEventListener("click", this._mouseEventHandler("click"));
    controls.addEventListener("click", this._mouseEventHandler("click"));
    //window.addEventListener("change", this._changeEventHandler("change"));
    //window.addEventListener("input", this._inputEventHandler("change"));
    window.addEventListener("keydown", this._keyEventHandler("key-down"));
    window.addEventListener("keyup", this._keyEventHandler("key-up"));
  }

  protected decorate(elem:Element, value: RawValue): Instance {
    let e = elem as Instance;
    e.__element = value;
    e.__source = this;
    e.textContent = `${value}`;
    this._container.appendChild(e);
    return e;
  }

  protected addInstance(row: any, column: any, value: RawValue) {
    row = row - 1;
    column = column - 1; 
    //if(id === null || id === "null") throw new Error(`Cannot create instance with null id for element '${elemId}'.`);
    if (this._instances[row] === undefined) {
      this._instances[row] = [];
    }
    
    /*
    let instance = this._instances[row][column];
    if (instance == undefined) {
      this._instances[row][column] = this.decorate(document.createElement("div"), value);
    } else {
      instance.textContent = `${value}`;
    }*/




    if (this._paths[row] === undefined) {
      this._paths[row] = [];
    }
    this._paths[row][column] = value;
    
    //let n = new Node();
    //this._container.appendChild(n);
    //if(instance) throw new Error(`Recreating existing instance '${id}'`);
    //if(ns) instance = this.decorate(document.createElementNS(""+ns, ""+tagname), elemId);
    //else instance = this.decorate(document.createElement(""+tagname), elemId);
    //if(!this._elementToInstances[elemId]) this._elementToInstances[elemId] = [id];
    //else this._elementToInstances[elemId].push(id);
    //return this._instances[id] = instance;
    this.changing();
  }

  rerender() {
    
    let canvas = this._canvas;
    let context = canvas.getContext("2d")!;
    context.clearRect(0, 0, canvas.width, canvas.height);

    let radius = 10;
    for (let path of this._paths) {
      let centerX = path[0] / 10;
      let centerY = path[1] / 10;
      context.beginPath();
      context.arc(centerX, centerY, radius, 0, 2 * Math.PI, false);
      context.fillStyle = 'black';
      context.fill();
      context.lineWidth = 1;
      context.strokeStyle = '#000000';
      context.stroke();
    }
  }

  _isChanging = false;
  changed = () => {
    this.rerender();
    this._isChanging = false;
  }

  changing() {
    if(!this._isChanging) {
      this._isChanging = true;
      setImmediate(this.changed);
    }
  }

  handlers = {
    "export instances": handleTuples(({adds, removes}) => {
      for(let remove of removes || EMPTY) {
        //this.removeInstance(instanceId);
      }
      for(let [table, row, column, value] of adds || EMPTY) {
        let v: any = value;
        if (table == 1819042146 ) {
          if (column == 120) {
            column = 1;
          }
          if (column == 121) {
            column = 2;
          }
          this.addInstance(row, column, v.Number);
        } else if (table == 3436366081) {
          this._editor.value = v.String;
        }
      }
    }),
  };

  protected _sendEvent(change:RawChange[]) {
    //console.log(this.program.history);
    this.program.send_transaction(change);
  }

  // ## Browser Event Handlers 

  _mouseEventHandler(tagname:string) {
    let table_id = 0x1a076b771;
    return (event:MouseEvent) => {
      if (event.target !== null) {
        let target: any = event.target;
        switch (target.id) {
          case "controls-reset": this.program.send_control(1); break;
          case "controls-stop": this.program.send_control(2); break;
          case "controls-step-back": this.program.send_control(3); break;
          case "controls-step-forward": this.program.send_control(4); break;
          case "controls-pause": this.program.send_control(5); break;
          case "controls-resume": this.program.send_control(6); break;
          case "controls-database": this.program.send_control(8); break;
          case "controls-history": this.program.send_control(9); break;
          case "controls-clean": {
            this._paths = [];
            this.rerender();
            this.program.send_control(7); 
            break;
          };
          default: this._sendEvent([[table_id,1,120,event.x],
                                    [table_id,1,121,event.y]]);
        }
      }
    };
  }

  

  _keyEventHandler(tagname:string) {
    return (event:KeyboardEvent) => {
      if(event.repeat) return;
      switch (tagname) {
        case "key-down": {
          if (event.keyCode == 17) {
            this.control = true;
          }
          if (event.keyCode == 13 && this.control) {
            let target:any|null = event.target as Element;
            let value = target.value;
            this.program.send_code(value);
          }
         break; 
        }
        case "key-up": {
          if (event.keyCode == 17) {
            this.control = false;
          }
          break;
        }
        default: {}
      }
      let code = event.keyCode;
      let key = this._keyMap[code];
    };
  }

  _keyMap:{[key:number]: string|undefined} = { // Overrides to provide sane names for common control codes.
    9: "tab",
    13: "enter",
    16: "shift",
    17: "control",
    18: "alt",
    27: "escape",
    32: "space",
    37: "left",
    38: "up",
    39: "right",
    40: "down",
    91: "meta"
  }

};

Library.register(HTML.id, HTML);
(window as any)["lib"] = Library;
