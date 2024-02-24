// package: chat
// file: hello.proto

import * as jspb from 'google-protobuf';

export class PublishRequest extends jspb.Message {
	getMessage(): string;
	setMessage(value: string): void;

	serializeBinary(): Uint8Array;
	toObject(includeInstance?: boolean): PublishRequest.AsObject;
	static toObject(includeInstance: boolean, msg: PublishRequest): PublishRequest.AsObject;
	static extensions: { [key: number]: jspb.ExtensionFieldInfo<jspb.Message> };
	static extensionsBinary: { [key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message> };
	static serializeBinaryToWriter(message: PublishRequest, writer: jspb.BinaryWriter): void;
	static deserializeBinary(bytes: Uint8Array): PublishRequest;
	static deserializeBinaryFromReader(
		message: PublishRequest,
		reader: jspb.BinaryReader
	): PublishRequest;
}

export namespace PublishRequest {
	export type AsObject = {
		message: string;
	};
}

export class PublishReply extends jspb.Message {
	getMessage(): string;
	setMessage(value: string): void;

	serializeBinary(): Uint8Array;
	toObject(includeInstance?: boolean): PublishReply.AsObject;
	static toObject(includeInstance: boolean, msg: PublishReply): PublishReply.AsObject;
	static extensions: { [key: number]: jspb.ExtensionFieldInfo<jspb.Message> };
	static extensionsBinary: { [key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message> };
	static serializeBinaryToWriter(message: PublishReply, writer: jspb.BinaryWriter): void;
	static deserializeBinary(bytes: Uint8Array): PublishReply;
	static deserializeBinaryFromReader(
		message: PublishReply,
		reader: jspb.BinaryReader
	): PublishReply;
}

export namespace PublishReply {
	export type AsObject = {
		message: string;
	};
}

export class UpdateRequest extends jspb.Message {
	clearMessagesList(): void;
	getMessagesList(): Array<string>;
	setMessagesList(value: Array<string>): void;
	addMessages(value: string, index?: number): string;

	serializeBinary(): Uint8Array;
	toObject(includeInstance?: boolean): UpdateRequest.AsObject;
	static toObject(includeInstance: boolean, msg: UpdateRequest): UpdateRequest.AsObject;
	static extensions: { [key: number]: jspb.ExtensionFieldInfo<jspb.Message> };
	static extensionsBinary: { [key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message> };
	static serializeBinaryToWriter(message: UpdateRequest, writer: jspb.BinaryWriter): void;
	static deserializeBinary(bytes: Uint8Array): UpdateRequest;
	static deserializeBinaryFromReader(
		message: UpdateRequest,
		reader: jspb.BinaryReader
	): UpdateRequest;
}

export namespace UpdateRequest {
	export type AsObject = {
		messagesList: Array<string>;
	};
}
