import { Messages } from './proto/hello';

export async function readData(receiveStream) {
	const reader = receiveStream.getReader();
	let recv = '';
	while (true) {
		const { done, value } = await reader.read();
		if (done) {
			break;
		}
		// value is a Uint8Array
		recv = Messages.fromBinary(value).messages.join('\n');
		// recv = new TextDecoder().decode(value);

		console.log('got data from unidirectional stream: ' + recv);
	}
	return recv;
}

export const run = async (msg: string, wt: WebTransport) => {
	await wt.ready;

	console.log('connected');

	// Create a bidirectional stream
	const stream = await wt.createBidirectionalStream();
	console.log('created stream');

	const writer = stream.writable.getWriter();
	const reader = stream.readable.getReader();

	// Create a message
	const encoded = new TextEncoder().encode(msg);

	await writer.write(encoded);
	await writer.close();
	writer.releaseLock();

	console.log('send: ' + msg);

	// Read a message from it
	// TODO handle partial reads
	const { value } = await reader.read();

	const recv = new TextDecoder().decode(value);
	console.log('recv: ' + recv);

	// document.getElementById("output").innerText = recv;
	console.log('closed');

	writer.releaseLock();
	reader.releaseLock();

	return recv;
};
