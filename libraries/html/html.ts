import {Program, Library, createId, RawValue, RawEAV, RawMap, handleTuples} from "../../src";

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
  /** Instances are the physical DOM elements representing table elements. */
  _instances: Instance[][] = [];

  setup() {
    // If we're not in a browser environment, this library does nothing
    if(typeof document === "undefined") {
      this.handlers = {} as any;
      return;
    }

    this._container = document.createElement("div");
    this._container.setAttribute("program", this.program.name);
    document.body.appendChild(this._container);

    console.log("HELLO HTML!!!!!");
  }

  protected decorate(elem:Element, value: RawValue): Instance {
    let e = elem as Instance;
    e.__element = value;
    e.__source = this;
    e.textContent = `${value}`;
    this._container.appendChild(e);
    return e;
  }

  protected addInstance(table: number, row: number, column: number, value: number) {
    row = row - 1;
    column = column - 1; 
    //if(id === null || id === "null") throw new Error(`Cannot create instance with null id for element '${elemId}'.`);
    if (this._instances[row] === undefined) {
      this._instances[row] = [];
    }
    
    let instance = this._instances[row][column];
    if (instance == undefined) {
      this._instances[row][column] = this.decorate(document.createElement("div"), value);
    } else {
      instance.textContent = `${value}`;
    }

    //let n = new Node();
    //this._container.appendChild(n);
    //if(instance) throw new Error(`Recreating existing instance '${id}'`);
    //if(ns) instance = this.decorate(document.createElementNS(""+ns, ""+tagname), elemId);
    //else instance = this.decorate(document.createElement(""+tagname), elemId);
    //if(!this._elementToInstances[elemId]) this._elementToInstances[elemId] = [id];
    //else this._elementToInstances[elemId].push(id);
    //return this._instances[id] = instance;
    
  }

  handlers = {
    "export instances": handleTuples(({adds, removes}) => {
      for(let remove of removes || EMPTY) {
        console.log(remove);
        //this.removeInstance(instanceId);
      }
      for(let [handler, table, row, column, value] of adds || EMPTY) {
        console.log(handler, table, row, column, value);
        this.addInstance(table, row, column, value);
      }
    })
  };

};

Library.register(HTML.id, HTML);
(window as any)["lib"] = Library;
