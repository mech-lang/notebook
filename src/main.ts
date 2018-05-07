import {Connection, Message} from "./connection";

export interface DiffMessage extends Message { type: "diff"; adds?:String[]; removes?:String[]; }
export interface LoadBundleMessage extends Message { type: "load-bundle"; bundle: string }
export interface ErrorMessage extends Message { type:"error"; error:string }

let connection = new Connection(new WebSocket(`ws://localhost:3012`));

console.log(connection);