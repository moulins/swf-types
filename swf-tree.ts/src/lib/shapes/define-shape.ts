import {CaseStyle, DocumentType, Int32Type, LiteralType} from "kryo";
import {Uint16} from "semantic-types";
import {Rect} from "../basic-types/rect";
import {SwfTagType} from "../swf-tag-type";
import {SwfTagBase} from "../swf-tags/_base";
import {ShapeWithStyle} from "./shape-with-style";

export interface DefineShape extends SwfTagBase {
  type: SwfTagType.DefineShape;
  shapeId: Uint16;
  shapeBounds: Rect;
  shapes: ShapeWithStyle;
}

export namespace DefineShape {
  export interface Json {
    type: "define-shape";
    shape_id: number;
    shape_bounds: Rect.Json;
    shapes: ShapeWithStyle.Json;
  }

  export const type: DocumentType<DefineShape> = new DocumentType<DefineShape>({
    properties: {
      type: {type: new LiteralType({type: SwfTagType.type, value: SwfTagType.DefineShape})},
      shapeId: {type: new Int32Type()},
      shapeBounds: {type: Rect.type},
      shapes: {type: ShapeWithStyle.type},
    },
    rename: CaseStyle.SnakeCase,
  });
}
