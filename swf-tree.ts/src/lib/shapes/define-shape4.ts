import {BooleanType, CaseStyle, DocumentType, Int32Type, LiteralType} from "kryo";
import {Uint16} from "semantic-types";
import {Rect} from "../basic-types/rect";
import {SwfTagType} from "../swf-tag-type";
import {SwfTagBase} from "../swf-tags/_base";
import {ShapeWithStyle} from "./shape-with-style";

export interface DefineShape4 extends SwfTagBase {
  type: SwfTagType.DefineShape4;
  shapeId: Uint16;
  shapeBounds: Rect;
  edgeBounds: Rect;
  usesFillWindingRule: boolean;
  usesNonScalingStrokes: boolean;
  usesScalingStrokes: boolean;
  shapes: ShapeWithStyle;
}

export namespace DefineShape4 {
  export interface Json {
    type: "define-shape4";
    shape_id: number;
    shape_bounds: Rect.Json;
    edge_bounds: Rect.Json;
    uses_fill_winding_rule: boolean;
    uses_non_scaling_strokes: boolean;
    uses_scaling_strokes: boolean;
    shapes: ShapeWithStyle.Json;
  }

  export const type: DocumentType<DefineShape4> = new DocumentType<DefineShape4>({
    properties: {
      type: {type: new LiteralType({type: SwfTagType.type, value: SwfTagType.DefineShape4})},
      shapeId: {type: new Int32Type()},
      shapeBounds: {type: Rect.type},
      edgeBounds: {type: Rect.type},
      usesFillWindingRule: {type: new BooleanType()},
      usesNonScalingStrokes: {type: new BooleanType()},
      usesScalingStrokes: {type: new BooleanType()},
      shapes: {type: ShapeWithStyle.type},
    },
    rename: CaseStyle.SnakeCase,
  });
}
