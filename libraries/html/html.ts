import {Program, Library, createId, RawValue, RawEAV, RawMap, handleTuples} from "../../src";

const EMPTY:never[] = [];

export interface Instance extends HTMLElement {
  /*
  __element: RawValue,
  __source: HTML,
  __styles?: RawValue[],
  __sort?:RawValue,
  __autoSort?:RawValue,
  __listeners?: {[event:string]: boolean},
  __capturedKeys?: {[code:number]: boolean}
  */
}

export class HTML extends Library {
  static id = "html";

  /** Instances are the physical DOM elements representing table elements. */
  _instances: number[][] = [];

  setup() {
    console.log("HELLO HTML!!!!!");
  }

  protected addInstance(table: number, row: number, column: number, value: number) {
    row = row - 1;
    column = column - 1; 
    //if(id === null || id === "null") throw new Error(`Cannot create instance with null id for element '${elemId}'.`);
    if (this._instances[row] === undefined) {
      this._instances[row] = [];
    }
    this._instances[row][column] = value;
    console.log(this._instances);
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
