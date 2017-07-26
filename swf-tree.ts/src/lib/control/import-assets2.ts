import {ArrayType, CaseStyle, DocumentType, LiteralType, Ucs2StringType} from "kryo";
import {NamedId} from "../named-id";
import {_Tag} from "../tags/_tag";
import {TagType} from "../tags/_type";

export interface ImportAssets2 extends _Tag {
  type: TagType.ImportAssets2;
  url: string;
  assets: NamedId[];
}

export namespace ImportAssets2 {
  export interface Json {
    type: "import-assets2";
    url: string;
    assets: NamedId.Json[];
  }

  export const type: DocumentType<ImportAssets2> = new DocumentType<ImportAssets2>({
    properties: {
      type: {type: new LiteralType({type: TagType.type, value: TagType.ImportAssets2})},
      url: {type: new Ucs2StringType({maxLength: Infinity})},
      assets: {type: new ArrayType({itemType: NamedId.type, maxLength: Infinity})},
    },
    rename: CaseStyle.SnakeCase,
  });
}
