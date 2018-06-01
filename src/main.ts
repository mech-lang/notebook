import {Program, Library, Diff, RawEAV, RawTuple, libraries} from ".";
import {Connection, Message} from "./connection";

export interface DiffMessage extends Message { type: "diff"; adds?:number[]; removes?:number[]; }
export interface LoadBundleMessage extends Message { type: "load-bundle"; bundle: string }
export interface ErrorMessage extends Message { type:"error"; error:string }


class RemoteProgram implements Program {
  libraries = {};
  handlers:{[id:string]: (diff:Diff<RawTuple[]>) => void} = {};

  attach(libraryId:string):Library {
    return Library.attach(this, libraryId);
  }

  attached(libraryId:string, library:Library) {
    for(let handlerName in library.handlers) {
      this.handlers[`${libraryId}/${handlerName}`] = library.handlers[handlerName];
    }
  }

  constructor(public name = "Remote Client", public send:(type: string, diff: any) => void) {}

  send_transaction(transaction: RawEAV[]) {
    this.send("Transaction", {adds: transaction, removes: []});
    return this;
  }

  handleDiff(diff: any) {
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
      if(!program) throw new Error(`Unable to handle diff for unitialized program: '${diff.client}'.`);
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