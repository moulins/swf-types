import {CaseStyle, DocumentType, LiteralType} from "kryo";
import {Rgba} from "../basic-types/rgba";
import {FillStyleType} from "./fill-style-type";

export interface SolidFill {
  fillStyleType: FillStyleType.Solid;
  color: Rgba;
}

export namespace SolidFill {
  export interface Json {
    fill_style_type: "solid";
    color: Rgba.Json;
  }

  export const type: DocumentType<SolidFill> = new DocumentType<SolidFill>({
    properties: {
      fillStyleType: {type: new LiteralType({type: FillStyleType.type, value: FillStyleType.Solid})},
      color: {type: Rgba.type},
    },
    rename: CaseStyle.SnakeCase,
  });
}
