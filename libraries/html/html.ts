import "setimmediate";
import {Program, Library, createId, RawValue, RawChange, RawMap, handleTuples} from "../../src";

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
  /** Instances are the physical DOM elements representing table elements. */
  _instances: Instance[][] = [];
  _paths: number[][] = []; 

  setup() {
    // If we're not in a browser environment, this library does nothing
    if(typeof document === "undefined") {
      this.handlers = {} as any;
      return;
    }

    this._container = document.createElement("div");
    this._container.setAttribute("program", this.program.name);
    document.body.appendChild(this._container);


    let editor = document.createElement("div");
    editor.setAttribute("class", "editor");
    this._container.appendChild(editor);

    let reset = document.createElement("button");
    reset.setAttribute("id", "editor-reset");
    reset.innerHTML =  "Reset";
    editor.appendChild(reset);

    let stop = document.createElement("button");
    stop.setAttribute("id", "editor-stop");
    stop.innerHTML =  "Stop";
    editor.appendChild(stop);

    let step_back = document.createElement("button");
    step_back.setAttribute("id", "editor-step-back");
    step_back.innerHTML =  "Step Back";
    editor.appendChild(step_back);    

    let step_forward = document.createElement("button");
    step_forward.setAttribute("id", "editor-step-forward");
    step_forward.innerHTML =  "Step Forward";
    editor.appendChild(step_forward);    

    let pause = document.createElement("button");
    pause.setAttribute("id", "editor-pause");
    pause.innerHTML =  "Pause";
    editor.appendChild(pause);

    let resume = document.createElement("button");
    resume.setAttribute("id", "editor-resume");
    resume.innerHTML =  "Resume";
    editor.appendChild(resume);

    let clean = document.createElement("button");
    clean.setAttribute("id", "editor-clean");
    clean.innerHTML =  "Clean";
    editor.appendChild(clean);

    let canvas = this._canvas = document.createElement("canvas");
    canvas.setAttribute("width", "500");
    canvas.setAttribute("height", "500");
    canvas.style.backgroundColor = 'rgb(226, 79, 94)';
    this._container.appendChild(canvas);

    window.addEventListener("click", this._mouseEventHandler("click"));
    //window.addEventListener("change", this._changeEventHandler("change"));
    //window.addEventListener("input", this._inputEventHandler("change"));
    window.addEventListener("keyup", this._keyEventHandler("key-up"));
    //window.addEventListener("keyup", this._keyEventHandler("key-up"));

    var context = canvas.getContext('2d');
    if (context !== null) {
      var centerX = canvas.width / 2;
      var centerY = canvas.height / 2;
      var radius = 5;

      context.beginPath();
      context.arc(centerX, centerY, radius, 0, 2 * Math.PI, false);
      context.fillStyle = 'black';
      context.fill();
      context.lineWidth = 0;
      context.strokeStyle = '#000000';
      context.stroke();
    }


  }

  protected decorate(elem:Element, value: RawValue): Instance {
    let e = elem as Instance;
    e.__element = value;
    e.__source = this;
    e.textContent = `${value}`;
    this._container.appendChild(e);
    return e;
  }

  protected addInstance(row: number, column: number, value: number) {
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
    let radius = 5;
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
        if (table == 1819042146 ) {
          if (column == 120) {
            column = 1;
          }
          if (column == 121) {
            column = 2;
          }
          this.addInstance(row, column, value);
        }
      }
    })
  };

  protected _sendEvent(change:RawChange[]) {
    //console.log(this.program.history);
    this.program.send_transaction(change);
  }
  // ----------------------
  // BROWSER EVENT HANDLERS
  // ----------------------

  _mouseEventHandler(tagname:string) {
    let table_id = 0x1a076b771;
    return (event:MouseEvent) => {
      if (event.target !== null) {
        let target: any = event.target;
        switch (target.id) {
          case "editor-reset": this.program.send_control(1); break;
          case "editor-stop": this.program.send_control(2); break;
          case "editor-step-back": this.program.send_control(3); break;
          case "editor-step-forward": this.program.send_control(4); break;
          case "editor-pause": this.program.send_control(5); break;
          case "editor-resume": this.program.send_control(6); break;
          case "editor-clean": this.program.send_control(7); break;
          default: this._sendEvent([[table_id,1,120,event.x],
                                    [table_id,1,121,event.y]]);
        }
      }
    };
  }

  _keyEventHandler(tagname:string) {
    return (event:KeyboardEvent) => {
      if(event.repeat) return;
      let target:any|null = event.target as Element;

      let code = event.keyCode;
      let key = this._keyMap[code];
      let value = target.value;
      if (value != undefined) {
        this._sendEvent([[1, 1, 1, value]]);
      }
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
