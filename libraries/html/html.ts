import {Program, Library, createId, RawValue, RawEAV, RawMap, handleTuples} from "../../src";

const EMPTY:never[] = [];

export class HTML extends Library {
  static id = "html";

  setup() {
    console.log("HELLO HTML!!!!!");
  }

  handlers = {
    "export instances": handleTuples(({adds, removes}) => {
      for(let remove of removes || EMPTY) {
        console.log(remove);
        //this.removeInstance(instanceId);
      }
      for(let [handler, tag, row, column, value] of adds || EMPTY) {
        console.log(handler, tag, row, column, value);
        //this.addInstance(instanceId, elemId, tagname, ns);
      }
    })
  };

};

Library.register(HTML.id, HTML);
(window as any)["lib"] = Library;
