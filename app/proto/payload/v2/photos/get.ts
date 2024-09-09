/**
 * Generated by the protoc-gen-ts.  DO NOT EDIT!
 * compiler version: 3.12.4
 * source: payload/v2/photos/get.proto
 * git: https://github.com/thesayyn/protoc-gen-ts */
import * as dependency_1 from "./../../../entity/photo";
import * as pb_1 from "google-protobuf";

export class GetPhotoResponse extends pb_1.Message {
  #one_of_decls: number[][] = [];

  constructor(
    data?:
      | any[]
      | {
          photo?: dependency_1.Photo;
        },
  ) {
    super();
    pb_1.Message.initialize(
      this,
      Array.isArray(data) ? data : [],
      0,
      -1,
      [],
      this.#one_of_decls,
    );
    if (!Array.isArray(data) && typeof data == "object") {
      if ("photo" in data && data.photo != undefined) {
        this.photo = data.photo;
      }
    }
  }

  get photo() {
    return pb_1.Message.getWrapperField(
      this,
      dependency_1.Photo,
      1,
    ) as dependency_1.Photo;
  }

  set photo(value: dependency_1.Photo) {
    pb_1.Message.setWrapperField(this, 1, value);
  }

  get hasPhoto() {
    return pb_1.Message.getField(this, 1) != null;
  }

  static fromObject(data: {
    photo?: ReturnType<typeof dependency_1.Photo.prototype.toObject>;
  }): GetPhotoResponse {
    const message = new GetPhotoResponse({});
    if (data.photo != null) {
      message.photo = dependency_1.Photo.fromObject(data.photo);
    }
    return message;
  }

  toObject() {
    const data: {
      photo?: ReturnType<typeof dependency_1.Photo.prototype.toObject>;
    } = {};
    if (this.photo != null) {
      data.photo = this.photo.toObject();
    }
    return data;
  }

  serialize(): Uint8Array;
  serialize(w: pb_1.BinaryWriter): void;
  serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
    const writer = w || new pb_1.BinaryWriter();
    if (this.hasPhoto)
      writer.writeMessage(1, this.photo, () => this.photo.serialize(writer));
    if (!w) return writer.getResultBuffer();
  }

  static deserialize(bytes: Uint8Array | pb_1.BinaryReader): GetPhotoResponse {
    const reader =
        bytes instanceof pb_1.BinaryReader
          ? bytes
          : new pb_1.BinaryReader(bytes),
      message = new GetPhotoResponse();
    while (reader.nextField()) {
      if (reader.isEndGroup()) break;
      switch (reader.getFieldNumber()) {
        case 1:
          reader.readMessage(
            message.photo,
            () => (message.photo = dependency_1.Photo.deserialize(reader)),
          );
          break;
        default:
          reader.skipField();
      }
    }
    return message;
  }

  serializeBinary(): Uint8Array {
    return this.serialize();
  }

  static deserializeBinary(bytes: Uint8Array): GetPhotoResponse {
    return GetPhotoResponse.deserialize(bytes);
  }
}
