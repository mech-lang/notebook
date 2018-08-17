import {Program, Library, Diff, RawChange, RawTuple, libraries} from ".";
import {Connection, Message} from "./connection";

export interface DiffMessage extends Message { type: "diff"; adds: number[]; removes: number[]; }
export interface LoadBundleMessage extends Message { type: "load-bundle"; bundle: string }
export interface ErrorMessage extends Message { type:"error"; error:string }


class Table {
  public data: Number[][];

  constructor() {
      this.data = [];
      for(var i: number = 0; i < 10; i++) {
          this.data[i] = [];
          for(var j: number = 0; j< 10; j++) {
              this.data[i][j] = 0;
          }
      }
  }
}

class RemoteProgram implements Program {
  libraries = {};
  database: any = {};
  handlers:{[id:string]: (diff:Diff<RawTuple[]>) => void} = {};
  history: Number[][] = [];

  attach(libraryId:string):Library {
    return Library.attach(this, libraryId);
  }

  attached(libraryId:string, library:Library) {
    for(let handlerName in library.handlers) {
      this.handlers[`${libraryId}/${handlerName}`] = library.handlers[handlerName];
    }
  }

  constructor(public name = "Remote Client", public send:(type: string, diff: any) => void) {}

  send_transaction(transaction: RawChange[]) {
    this.send("Transaction", {adds: transaction, removes: []});
    return this;
  }

  send_control(kind: number) {
    this.send("Control", {kind});
    return this;
  }

  handleDiff(diff: any) {
    // Populate the database
    for(let add of diff.adds) {
      
      let table_id = add[0];
      let row = add[1];
      let column = add[2];
      let value = add[3];
      this.history.push([table_id, row, column, value]);
      let table = this.database[`${table_id}`];
      if (this.database[`${table_id}`] === undefined) {
        this.database[`${table_id}`] = new Table();
      } else {
        if (table.data[row - 1] === undefined) {
          table.data[row - 1] = [];
          for(var j: number = 0; j< 10; j++) {
            table.data[row - 1][j] = 0;
          }
        }
        table.data[row - 1][column - 1] = value;
      }      
    }
    for(let type in this.handlers) {
      this.handlers[type](diff);
    }
    
  }
}

class MultiplexedConnection extends Connection {
  programs:{[client:string]: RemoteProgram} = {};
  panes:{[client:string]: HTMLElement} = {};

  handlers = {
    "init": ({client}:Message) => {
      if(this.programs[client]) throw new Error(`Unable to initialize existing program: '${client}'.`);
      let program = this.programs[client] = new RemoteProgram(client, (type: string, diff: any) => this.send(type, diff, client));
      let html = program.attach("html") as libraries.HTML;
    },
    "diff": (diff:DiffMessage) => {
      let program = this.programs[diff.client];
      if(!program) throw new Error(`Unable to handle diff for program: '${diff.client}'.`);
      program.handleDiff(diff);
    }
  };

  addPane(name:string, container:HTMLElement) {
    if(this.panes[name] && this.panes[name] !== container) {
      console.warn(`Overwriting container for existing pane '${name}'`);
    }
    this.panes[name] = container;
    container.classList.add("program-pane");
  }
}

let host = location.hostname == "" ? "localhost" : location.hostname;
let connection = new MultiplexedConnection(new WebSocket(`ws://${host}:3012`));

console.log(connection);