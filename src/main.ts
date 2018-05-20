import {Connection, Message} from "./connection";

export interface DiffMessage extends Message { type: "diff"; adds?:String[]; removes?:String[]; }
export interface LoadBundleMessage extends Message { type: "load-bundle"; bundle: string }
export interface ErrorMessage extends Message { type:"error"; error:string }


class MultiplexedConnection extends Connection {
  handlers = {
    "diff": (diff:DiffMessage) => {
      console.log(diff);
    }
  };
}

let host = location.hostname == "" ? "localhost" : location.hostname;
let connection = new MultiplexedConnection(new WebSocket(`ws://${host}:3012`));

console.log(connection);