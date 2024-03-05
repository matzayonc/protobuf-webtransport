import { writable } from 'svelte/store';

const [url, fingerprintHex] = [
	'https://localhost:4443',
	'b3d1839927ee4321f97fc1d44b76e3699bfce31be2f52471f385a8a4d6ef9e6f'
];

const fingerprint: number[] = [];
for (let c = 0; c < fingerprintHex.length - 1; c += 2) {
	fingerprint.push(parseInt(fingerprintHex.substring(c, c + 2), 16));
}

export const connection = writable(
	new WebTransport(url, {
		serverCertificateHashes: [
			{
				algorithm: 'sha-256',
				value: new Uint8Array(fingerprint)
			}
		]
	})
);
