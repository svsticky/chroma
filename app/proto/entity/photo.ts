/**
 * Generated by the protoc-gen-ts.  DO NOT EDIT!
 * compiler version: 3.21.12
 * source: entity/photo.proto
 * git: https://github.com/thesayyn/protoc-gen-ts */
import * as dependency_1 from "./user";
import * as pb_1 from "google-protobuf";
export class Photo extends pb_1.Message {
    #one_of_decls: number[][] = [[1], [2], [3], [4], [5], [6], [7]];
    constructor(data?: any[] | ({} & (({
        id?: string;
    }) | ({
        uploadedAt?: number;
    }) | ({
        uploadedBy?: dependency_1.User;
    }) | ({
        capturedAt?: number;
    }) | ({
        linked?: PhotoLinks;
    }) | ({
        media?: PhotoMedia;
    }) | ({
        metadata?: PhotoMetadata;
    })))) {
        super();
        pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [], this.#one_of_decls);
        if (!Array.isArray(data) && typeof data == "object") {
            if ("id" in data && data.id != undefined) {
                this.id = data.id;
            }
            if ("uploadedAt" in data && data.uploadedAt != undefined) {
                this.uploadedAt = data.uploadedAt;
            }
            if ("uploadedBy" in data && data.uploadedBy != undefined) {
                this.uploadedBy = data.uploadedBy;
            }
            if ("capturedAt" in data && data.capturedAt != undefined) {
                this.capturedAt = data.capturedAt;
            }
            if ("linked" in data && data.linked != undefined) {
                this.linked = data.linked;
            }
            if ("media" in data && data.media != undefined) {
                this.media = data.media;
            }
            if ("metadata" in data && data.metadata != undefined) {
                this.metadata = data.metadata;
            }
        }
    }
    get id() {
        return pb_1.Message.getFieldWithDefault(this, 1, "") as string;
    }
    set id(value: string) {
        pb_1.Message.setOneofField(this, 1, this.#one_of_decls[0], value);
    }
    get hasId() {
        return pb_1.Message.getField(this, 1) != null;
    }
    get uploadedAt() {
        return pb_1.Message.getFieldWithDefault(this, 2, 0) as number;
    }
    set uploadedAt(value: number) {
        pb_1.Message.setOneofField(this, 2, this.#one_of_decls[1], value);
    }
    get hasUploadedAt() {
        return pb_1.Message.getField(this, 2) != null;
    }
    get uploadedBy() {
        return pb_1.Message.getWrapperField(this, dependency_1.User, 3) as dependency_1.User;
    }
    set uploadedBy(value: dependency_1.User) {
        pb_1.Message.setOneofWrapperField(this, 3, this.#one_of_decls[2], value);
    }
    get hasUploadedBy() {
        return pb_1.Message.getField(this, 3) != null;
    }
    get capturedAt() {
        return pb_1.Message.getFieldWithDefault(this, 4, 0) as number;
    }
    set capturedAt(value: number) {
        pb_1.Message.setOneofField(this, 4, this.#one_of_decls[3], value);
    }
    get hasCapturedAt() {
        return pb_1.Message.getField(this, 4) != null;
    }
    get linked() {
        return pb_1.Message.getWrapperField(this, PhotoLinks, 5) as PhotoLinks;
    }
    set linked(value: PhotoLinks) {
        pb_1.Message.setOneofWrapperField(this, 5, this.#one_of_decls[4], value);
    }
    get hasLinked() {
        return pb_1.Message.getField(this, 5) != null;
    }
    get media() {
        return pb_1.Message.getWrapperField(this, PhotoMedia, 6) as PhotoMedia;
    }
    set media(value: PhotoMedia) {
        pb_1.Message.setOneofWrapperField(this, 6, this.#one_of_decls[5], value);
    }
    get hasMedia() {
        return pb_1.Message.getField(this, 6) != null;
    }
    get metadata() {
        return pb_1.Message.getWrapperField(this, PhotoMetadata, 7) as PhotoMetadata;
    }
    set metadata(value: PhotoMetadata) {
        pb_1.Message.setOneofWrapperField(this, 7, this.#one_of_decls[6], value);
    }
    get hasMetadata() {
        return pb_1.Message.getField(this, 7) != null;
    }
    get _id() {
        const cases: {
            [index: number]: "none" | "id";
        } = {
            0: "none",
            1: "id"
        };
        return cases[pb_1.Message.computeOneofCase(this, [1])];
    }
    get _uploadedAt() {
        const cases: {
            [index: number]: "none" | "uploadedAt";
        } = {
            0: "none",
            2: "uploadedAt"
        };
        return cases[pb_1.Message.computeOneofCase(this, [2])];
    }
    get _uploadedBy() {
        const cases: {
            [index: number]: "none" | "uploadedBy";
        } = {
            0: "none",
            3: "uploadedBy"
        };
        return cases[pb_1.Message.computeOneofCase(this, [3])];
    }
    get _capturedAt() {
        const cases: {
            [index: number]: "none" | "capturedAt";
        } = {
            0: "none",
            4: "capturedAt"
        };
        return cases[pb_1.Message.computeOneofCase(this, [4])];
    }
    get _linked() {
        const cases: {
            [index: number]: "none" | "linked";
        } = {
            0: "none",
            5: "linked"
        };
        return cases[pb_1.Message.computeOneofCase(this, [5])];
    }
    get _media() {
        const cases: {
            [index: number]: "none" | "media";
        } = {
            0: "none",
            6: "media"
        };
        return cases[pb_1.Message.computeOneofCase(this, [6])];
    }
    get _metadata() {
        const cases: {
            [index: number]: "none" | "metadata";
        } = {
            0: "none",
            7: "metadata"
        };
        return cases[pb_1.Message.computeOneofCase(this, [7])];
    }
    static fromObject(data: {
        id?: string;
        uploadedAt?: number;
        uploadedBy?: ReturnType<typeof dependency_1.User.prototype.toObject>;
        capturedAt?: number;
        linked?: ReturnType<typeof PhotoLinks.prototype.toObject>;
        media?: ReturnType<typeof PhotoMedia.prototype.toObject>;
        metadata?: ReturnType<typeof PhotoMetadata.prototype.toObject>;
    }): Photo {
        const message = new Photo({});
        if (data.id != null) {
            message.id = data.id;
        }
        if (data.uploadedAt != null) {
            message.uploadedAt = data.uploadedAt;
        }
        if (data.uploadedBy != null) {
            message.uploadedBy = dependency_1.User.fromObject(data.uploadedBy);
        }
        if (data.capturedAt != null) {
            message.capturedAt = data.capturedAt;
        }
        if (data.linked != null) {
            message.linked = PhotoLinks.fromObject(data.linked);
        }
        if (data.media != null) {
            message.media = PhotoMedia.fromObject(data.media);
        }
        if (data.metadata != null) {
            message.metadata = PhotoMetadata.fromObject(data.metadata);
        }
        return message;
    }
    toObject() {
        const data: {
            id?: string;
            uploadedAt?: number;
            uploadedBy?: ReturnType<typeof dependency_1.User.prototype.toObject>;
            capturedAt?: number;
            linked?: ReturnType<typeof PhotoLinks.prototype.toObject>;
            media?: ReturnType<typeof PhotoMedia.prototype.toObject>;
            metadata?: ReturnType<typeof PhotoMetadata.prototype.toObject>;
        } = {};
        if (this.id != null) {
            data.id = this.id;
        }
        if (this.uploadedAt != null) {
            data.uploadedAt = this.uploadedAt;
        }
        if (this.uploadedBy != null) {
            data.uploadedBy = this.uploadedBy.toObject();
        }
        if (this.capturedAt != null) {
            data.capturedAt = this.capturedAt;
        }
        if (this.linked != null) {
            data.linked = this.linked.toObject();
        }
        if (this.media != null) {
            data.media = this.media.toObject();
        }
        if (this.metadata != null) {
            data.metadata = this.metadata.toObject();
        }
        return data;
    }
    serialize(): Uint8Array;
    serialize(w: pb_1.BinaryWriter): void;
    serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
        const writer = w || new pb_1.BinaryWriter();
        if (this.hasId)
            writer.writeString(1, this.id);
        if (this.hasUploadedAt)
            writer.writeInt64(2, this.uploadedAt);
        if (this.hasUploadedBy)
            writer.writeMessage(3, this.uploadedBy, () => this.uploadedBy.serialize(writer));
        if (this.hasCapturedAt)
            writer.writeInt64(4, this.capturedAt);
        if (this.hasLinked)
            writer.writeMessage(5, this.linked, () => this.linked.serialize(writer));
        if (this.hasMedia)
            writer.writeMessage(6, this.media, () => this.media.serialize(writer));
        if (this.hasMetadata)
            writer.writeMessage(7, this.metadata, () => this.metadata.serialize(writer));
        if (!w)
            return writer.getResultBuffer();
    }
    static deserialize(bytes: Uint8Array | pb_1.BinaryReader): Photo {
        const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new Photo();
        while (reader.nextField()) {
            if (reader.isEndGroup())
                break;
            switch (reader.getFieldNumber()) {
                case 1:
                    message.id = reader.readString();
                    break;
                case 2:
                    message.uploadedAt = reader.readInt64();
                    break;
                case 3:
                    reader.readMessage(message.uploadedBy, () => message.uploadedBy = dependency_1.User.deserialize(reader));
                    break;
                case 4:
                    message.capturedAt = reader.readInt64();
                    break;
                case 5:
                    reader.readMessage(message.linked, () => message.linked = PhotoLinks.deserialize(reader));
                    break;
                case 6:
                    reader.readMessage(message.media, () => message.media = PhotoMedia.deserialize(reader));
                    break;
                case 7:
                    reader.readMessage(message.metadata, () => message.metadata = PhotoMetadata.deserialize(reader));
                    break;
                default: reader.skipField();
            }
        }
        return message;
    }
    serializeBinary(): Uint8Array {
        return this.serialize();
    }
    static deserializeBinary(bytes: Uint8Array): Photo {
        return Photo.deserialize(bytes);
    }
}
export class PhotoLinks extends pb_1.Message {
    #one_of_decls: number[][] = [];
    constructor(data?: any[] | {
        albums?: PhotoAlbum[];
    }) {
        super();
        pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [1], this.#one_of_decls);
        if (!Array.isArray(data) && typeof data == "object") {
            if ("albums" in data && data.albums != undefined) {
                this.albums = data.albums;
            }
        }
    }
    get albums() {
        return pb_1.Message.getRepeatedWrapperField(this, PhotoAlbum, 1) as PhotoAlbum[];
    }
    set albums(value: PhotoAlbum[]) {
        pb_1.Message.setRepeatedWrapperField(this, 1, value);
    }
    static fromObject(data: {
        albums?: ReturnType<typeof PhotoAlbum.prototype.toObject>[];
    }): PhotoLinks {
        const message = new PhotoLinks({});
        if (data.albums != null) {
            message.albums = data.albums.map(item => PhotoAlbum.fromObject(item));
        }
        return message;
    }
    toObject() {
        const data: {
            albums?: ReturnType<typeof PhotoAlbum.prototype.toObject>[];
        } = {};
        if (this.albums != null) {
            data.albums = this.albums.map((item: PhotoAlbum) => item.toObject());
        }
        return data;
    }
    serialize(): Uint8Array;
    serialize(w: pb_1.BinaryWriter): void;
    serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
        const writer = w || new pb_1.BinaryWriter();
        if (this.albums.length)
            writer.writeRepeatedMessage(1, this.albums, (item: PhotoAlbum) => item.serialize(writer));
        if (!w)
            return writer.getResultBuffer();
    }
    static deserialize(bytes: Uint8Array | pb_1.BinaryReader): PhotoLinks {
        const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new PhotoLinks();
        while (reader.nextField()) {
            if (reader.isEndGroup())
                break;
            switch (reader.getFieldNumber()) {
                case 1:
                    reader.readMessage(message.albums, () => pb_1.Message.addToRepeatedWrapperField(message, 1, PhotoAlbum.deserialize(reader), PhotoAlbum));
                    break;
                default: reader.skipField();
            }
        }
        return message;
    }
    serializeBinary(): Uint8Array {
        return this.serialize();
    }
    static deserializeBinary(bytes: Uint8Array): PhotoLinks {
        return PhotoLinks.deserialize(bytes);
    }
}
export class PhotoAlbum extends pb_1.Message {
    #one_of_decls: number[][] = [];
    constructor(data?: any[] | {
        id?: string;
    }) {
        super();
        pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [], this.#one_of_decls);
        if (!Array.isArray(data) && typeof data == "object") {
            if ("id" in data && data.id != undefined) {
                this.id = data.id;
            }
        }
    }
    get id() {
        return pb_1.Message.getFieldWithDefault(this, 1, "") as string;
    }
    set id(value: string) {
        pb_1.Message.setField(this, 1, value);
    }
    static fromObject(data: {
        id?: string;
    }): PhotoAlbum {
        const message = new PhotoAlbum({});
        if (data.id != null) {
            message.id = data.id;
        }
        return message;
    }
    toObject() {
        const data: {
            id?: string;
        } = {};
        if (this.id != null) {
            data.id = this.id;
        }
        return data;
    }
    serialize(): Uint8Array;
    serialize(w: pb_1.BinaryWriter): void;
    serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
        const writer = w || new pb_1.BinaryWriter();
        if (this.id.length)
            writer.writeString(1, this.id);
        if (!w)
            return writer.getResultBuffer();
    }
    static deserialize(bytes: Uint8Array | pb_1.BinaryReader): PhotoAlbum {
        const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new PhotoAlbum();
        while (reader.nextField()) {
            if (reader.isEndGroup())
                break;
            switch (reader.getFieldNumber()) {
                case 1:
                    message.id = reader.readString();
                    break;
                default: reader.skipField();
            }
        }
        return message;
    }
    serializeBinary(): Uint8Array {
        return this.serialize();
    }
    static deserializeBinary(bytes: Uint8Array): PhotoAlbum {
        return PhotoAlbum.deserialize(bytes);
    }
}
export class PhotoMedia extends pb_1.Message {
    #one_of_decls: number[][] = [];
    constructor(data?: any[] | {
        urls?: PhotoUrl[];
    }) {
        super();
        pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [1], this.#one_of_decls);
        if (!Array.isArray(data) && typeof data == "object") {
            if ("urls" in data && data.urls != undefined) {
                this.urls = data.urls;
            }
        }
    }
    get urls() {
        return pb_1.Message.getRepeatedWrapperField(this, PhotoUrl, 1) as PhotoUrl[];
    }
    set urls(value: PhotoUrl[]) {
        pb_1.Message.setRepeatedWrapperField(this, 1, value);
    }
    static fromObject(data: {
        urls?: ReturnType<typeof PhotoUrl.prototype.toObject>[];
    }): PhotoMedia {
        const message = new PhotoMedia({});
        if (data.urls != null) {
            message.urls = data.urls.map(item => PhotoUrl.fromObject(item));
        }
        return message;
    }
    toObject() {
        const data: {
            urls?: ReturnType<typeof PhotoUrl.prototype.toObject>[];
        } = {};
        if (this.urls != null) {
            data.urls = this.urls.map((item: PhotoUrl) => item.toObject());
        }
        return data;
    }
    serialize(): Uint8Array;
    serialize(w: pb_1.BinaryWriter): void;
    serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
        const writer = w || new pb_1.BinaryWriter();
        if (this.urls.length)
            writer.writeRepeatedMessage(1, this.urls, (item: PhotoUrl) => item.serialize(writer));
        if (!w)
            return writer.getResultBuffer();
    }
    static deserialize(bytes: Uint8Array | pb_1.BinaryReader): PhotoMedia {
        const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new PhotoMedia();
        while (reader.nextField()) {
            if (reader.isEndGroup())
                break;
            switch (reader.getFieldNumber()) {
                case 1:
                    reader.readMessage(message.urls, () => pb_1.Message.addToRepeatedWrapperField(message, 1, PhotoUrl.deserialize(reader), PhotoUrl));
                    break;
                default: reader.skipField();
            }
        }
        return message;
    }
    serializeBinary(): Uint8Array {
        return this.serialize();
    }
    static deserializeBinary(bytes: Uint8Array): PhotoMedia {
        return PhotoMedia.deserialize(bytes);
    }
}
export class PhotoUrl extends pb_1.Message {
    #one_of_decls: number[][] = [];
    constructor(data?: any[] | {
        url?: string;
        size?: string;
        dimensions?: PhotoDimensions;
    }) {
        super();
        pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [], this.#one_of_decls);
        if (!Array.isArray(data) && typeof data == "object") {
            if ("url" in data && data.url != undefined) {
                this.url = data.url;
            }
            if ("size" in data && data.size != undefined) {
                this.size = data.size;
            }
            if ("dimensions" in data && data.dimensions != undefined) {
                this.dimensions = data.dimensions;
            }
        }
    }
    get url() {
        return pb_1.Message.getFieldWithDefault(this, 1, "") as string;
    }
    set url(value: string) {
        pb_1.Message.setField(this, 1, value);
    }
    get size() {
        return pb_1.Message.getFieldWithDefault(this, 2, "") as string;
    }
    set size(value: string) {
        pb_1.Message.setField(this, 2, value);
    }
    get dimensions() {
        return pb_1.Message.getWrapperField(this, PhotoDimensions, 3) as PhotoDimensions;
    }
    set dimensions(value: PhotoDimensions) {
        pb_1.Message.setWrapperField(this, 3, value);
    }
    get hasDimensions() {
        return pb_1.Message.getField(this, 3) != null;
    }
    static fromObject(data: {
        url?: string;
        size?: string;
        dimensions?: ReturnType<typeof PhotoDimensions.prototype.toObject>;
    }): PhotoUrl {
        const message = new PhotoUrl({});
        if (data.url != null) {
            message.url = data.url;
        }
        if (data.size != null) {
            message.size = data.size;
        }
        if (data.dimensions != null) {
            message.dimensions = PhotoDimensions.fromObject(data.dimensions);
        }
        return message;
    }
    toObject() {
        const data: {
            url?: string;
            size?: string;
            dimensions?: ReturnType<typeof PhotoDimensions.prototype.toObject>;
        } = {};
        if (this.url != null) {
            data.url = this.url;
        }
        if (this.size != null) {
            data.size = this.size;
        }
        if (this.dimensions != null) {
            data.dimensions = this.dimensions.toObject();
        }
        return data;
    }
    serialize(): Uint8Array;
    serialize(w: pb_1.BinaryWriter): void;
    serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
        const writer = w || new pb_1.BinaryWriter();
        if (this.url.length)
            writer.writeString(1, this.url);
        if (this.size.length)
            writer.writeString(2, this.size);
        if (this.hasDimensions)
            writer.writeMessage(3, this.dimensions, () => this.dimensions.serialize(writer));
        if (!w)
            return writer.getResultBuffer();
    }
    static deserialize(bytes: Uint8Array | pb_1.BinaryReader): PhotoUrl {
        const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new PhotoUrl();
        while (reader.nextField()) {
            if (reader.isEndGroup())
                break;
            switch (reader.getFieldNumber()) {
                case 1:
                    message.url = reader.readString();
                    break;
                case 2:
                    message.size = reader.readString();
                    break;
                case 3:
                    reader.readMessage(message.dimensions, () => message.dimensions = PhotoDimensions.deserialize(reader));
                    break;
                default: reader.skipField();
            }
        }
        return message;
    }
    serializeBinary(): Uint8Array {
        return this.serialize();
    }
    static deserializeBinary(bytes: Uint8Array): PhotoUrl {
        return PhotoUrl.deserialize(bytes);
    }
}
export class PhotoDimensions extends pb_1.Message {
    #one_of_decls: number[][] = [];
    constructor(data?: any[] | {
        width?: number;
        height?: number;
    }) {
        super();
        pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [], this.#one_of_decls);
        if (!Array.isArray(data) && typeof data == "object") {
            if ("width" in data && data.width != undefined) {
                this.width = data.width;
            }
            if ("height" in data && data.height != undefined) {
                this.height = data.height;
            }
        }
    }
    get width() {
        return pb_1.Message.getFieldWithDefault(this, 1, 0) as number;
    }
    set width(value: number) {
        pb_1.Message.setField(this, 1, value);
    }
    get height() {
        return pb_1.Message.getFieldWithDefault(this, 2, 0) as number;
    }
    set height(value: number) {
        pb_1.Message.setField(this, 2, value);
    }
    static fromObject(data: {
        width?: number;
        height?: number;
    }): PhotoDimensions {
        const message = new PhotoDimensions({});
        if (data.width != null) {
            message.width = data.width;
        }
        if (data.height != null) {
            message.height = data.height;
        }
        return message;
    }
    toObject() {
        const data: {
            width?: number;
            height?: number;
        } = {};
        if (this.width != null) {
            data.width = this.width;
        }
        if (this.height != null) {
            data.height = this.height;
        }
        return data;
    }
    serialize(): Uint8Array;
    serialize(w: pb_1.BinaryWriter): void;
    serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
        const writer = w || new pb_1.BinaryWriter();
        if (this.width != 0)
            writer.writeInt64(1, this.width);
        if (this.height != 0)
            writer.writeInt64(2, this.height);
        if (!w)
            return writer.getResultBuffer();
    }
    static deserialize(bytes: Uint8Array | pb_1.BinaryReader): PhotoDimensions {
        const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new PhotoDimensions();
        while (reader.nextField()) {
            if (reader.isEndGroup())
                break;
            switch (reader.getFieldNumber()) {
                case 1:
                    message.width = reader.readInt64();
                    break;
                case 2:
                    message.height = reader.readInt64();
                    break;
                default: reader.skipField();
            }
        }
        return message;
    }
    serializeBinary(): Uint8Array {
        return this.serialize();
    }
    static deserializeBinary(bytes: Uint8Array): PhotoDimensions {
        return PhotoDimensions.deserialize(bytes);
    }
}
export class PhotoMetadata extends pb_1.Message {
    #one_of_decls: number[][] = [];
    constructor(data?: any[] | {
        exif?: PhotoExif[];
    }) {
        super();
        pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [1], this.#one_of_decls);
        if (!Array.isArray(data) && typeof data == "object") {
            if ("exif" in data && data.exif != undefined) {
                this.exif = data.exif;
            }
        }
    }
    get exif() {
        return pb_1.Message.getRepeatedWrapperField(this, PhotoExif, 1) as PhotoExif[];
    }
    set exif(value: PhotoExif[]) {
        pb_1.Message.setRepeatedWrapperField(this, 1, value);
    }
    static fromObject(data: {
        exif?: ReturnType<typeof PhotoExif.prototype.toObject>[];
    }): PhotoMetadata {
        const message = new PhotoMetadata({});
        if (data.exif != null) {
            message.exif = data.exif.map(item => PhotoExif.fromObject(item));
        }
        return message;
    }
    toObject() {
        const data: {
            exif?: ReturnType<typeof PhotoExif.prototype.toObject>[];
        } = {};
        if (this.exif != null) {
            data.exif = this.exif.map((item: PhotoExif) => item.toObject());
        }
        return data;
    }
    serialize(): Uint8Array;
    serialize(w: pb_1.BinaryWriter): void;
    serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
        const writer = w || new pb_1.BinaryWriter();
        if (this.exif.length)
            writer.writeRepeatedMessage(1, this.exif, (item: PhotoExif) => item.serialize(writer));
        if (!w)
            return writer.getResultBuffer();
    }
    static deserialize(bytes: Uint8Array | pb_1.BinaryReader): PhotoMetadata {
        const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new PhotoMetadata();
        while (reader.nextField()) {
            if (reader.isEndGroup())
                break;
            switch (reader.getFieldNumber()) {
                case 1:
                    reader.readMessage(message.exif, () => pb_1.Message.addToRepeatedWrapperField(message, 1, PhotoExif.deserialize(reader), PhotoExif));
                    break;
                default: reader.skipField();
            }
        }
        return message;
    }
    serializeBinary(): Uint8Array {
        return this.serialize();
    }
    static deserializeBinary(bytes: Uint8Array): PhotoMetadata {
        return PhotoMetadata.deserialize(bytes);
    }
}
export class PhotoExif extends pb_1.Message {
    #one_of_decls: number[][] = [];
    constructor(data?: any[] | {
        key?: string;
        value?: string;
    }) {
        super();
        pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [], this.#one_of_decls);
        if (!Array.isArray(data) && typeof data == "object") {
            if ("key" in data && data.key != undefined) {
                this.key = data.key;
            }
            if ("value" in data && data.value != undefined) {
                this.value = data.value;
            }
        }
    }
    get key() {
        return pb_1.Message.getFieldWithDefault(this, 1, "") as string;
    }
    set key(value: string) {
        pb_1.Message.setField(this, 1, value);
    }
    get value() {
        return pb_1.Message.getFieldWithDefault(this, 2, "") as string;
    }
    set value(value: string) {
        pb_1.Message.setField(this, 2, value);
    }
    static fromObject(data: {
        key?: string;
        value?: string;
    }): PhotoExif {
        const message = new PhotoExif({});
        if (data.key != null) {
            message.key = data.key;
        }
        if (data.value != null) {
            message.value = data.value;
        }
        return message;
    }
    toObject() {
        const data: {
            key?: string;
            value?: string;
        } = {};
        if (this.key != null) {
            data.key = this.key;
        }
        if (this.value != null) {
            data.value = this.value;
        }
        return data;
    }
    serialize(): Uint8Array;
    serialize(w: pb_1.BinaryWriter): void;
    serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
        const writer = w || new pb_1.BinaryWriter();
        if (this.key.length)
            writer.writeString(1, this.key);
        if (this.value.length)
            writer.writeString(2, this.value);
        if (!w)
            return writer.getResultBuffer();
    }
    static deserialize(bytes: Uint8Array | pb_1.BinaryReader): PhotoExif {
        const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new PhotoExif();
        while (reader.nextField()) {
            if (reader.isEndGroup())
                break;
            switch (reader.getFieldNumber()) {
                case 1:
                    message.key = reader.readString();
                    break;
                case 2:
                    message.value = reader.readString();
                    break;
                default: reader.skipField();
            }
        }
        return message;
    }
    serializeBinary(): Uint8Array {
        return this.serialize();
    }
    static deserializeBinary(bytes: Uint8Array): PhotoExif {
        return PhotoExif.deserialize(bytes);
    }
}
