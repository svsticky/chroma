/**
 * Generated by the protoc-gen-ts.  DO NOT EDIT!
 * compiler version: 3.19.4
 * source: payload/v1/user/list.proto
 * git: https://github.com/thesayyn/protoc-gen-ts */
import * as dependency_1 from "./../../../entity/user";
import * as pb_1 from "google-protobuf";
export class ListUserResponse extends pb_1.Message {
    #one_of_decls: number[][] = [];
    constructor(data?: any[] | {
        users?: dependency_1.User[];
    }) {
        super();
        pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [1], this.#one_of_decls);
        if (!Array.isArray(data) && typeof data == "object") {
            if ("users" in data && data.users != undefined) {
                this.users = data.users;
            }
        }
    }
    get users() {
        return pb_1.Message.getRepeatedWrapperField(this, dependency_1.User, 1) as dependency_1.User[];
    }
    set users(value: dependency_1.User[]) {
        pb_1.Message.setRepeatedWrapperField(this, 1, value);
    }
    static fromObject(data: {
        users?: ReturnType<typeof dependency_1.User.prototype.toObject>[];
    }): ListUserResponse {
        const message = new ListUserResponse({});
        if (data.users != null) {
            message.users = data.users.map(item => dependency_1.User.fromObject(item));
        }
        return message;
    }
    toObject() {
        const data: {
            users?: ReturnType<typeof dependency_1.User.prototype.toObject>[];
        } = {};
        if (this.users != null) {
            data.users = this.users.map((item: dependency_1.User) => item.toObject());
        }
        return data;
    }
    serialize(): Uint8Array;
    serialize(w: pb_1.BinaryWriter): void;
    serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
        const writer = w || new pb_1.BinaryWriter();
        if (this.users.length)
            writer.writeRepeatedMessage(1, this.users, (item: dependency_1.User) => item.serialize(writer));
        if (!w)
            return writer.getResultBuffer();
    }
    static deserialize(bytes: Uint8Array | pb_1.BinaryReader): ListUserResponse {
        const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new ListUserResponse();
        while (reader.nextField()) {
            if (reader.isEndGroup())
                break;
            switch (reader.getFieldNumber()) {
                case 1:
                    reader.readMessage(message.users, () => pb_1.Message.addToRepeatedWrapperField(message, 1, dependency_1.User.deserialize(reader), dependency_1.User));
                    break;
                default: reader.skipField();
            }
        }
        return message;
    }
    serializeBinary(): Uint8Array {
        return this.serialize();
    }
    static deserializeBinary(bytes: Uint8Array): ListUserResponse {
        return ListUserResponse.deserialize(bytes);
    }
}
