<script lang="ts">
	import { onMount } from 'svelte';
	import { readData, run } from '../lib/utils';
	import { connection } from '../lib/stores/connection';

	let output = $state('');
	let input = $state('');

	const handleKey = async (event: KeyboardEvent) => {
		if (event.key === 'Enter') {
			const msg = input;
			input = '';

			run(msg, $connection);
		}
	};

	async function receiveUnidirectional() {
		console.log('waiting for unidirectional stream');
		const uds = $connection.incomingUnidirectionalStreams;
		const reader = uds.getReader();
		console.log('got unidirectional stream');
		while (true) {
			const { done, value: stream } = await reader.read();
			if (done) {
				console.log('no more unidirectional streams');
				break;
			}

			console.log('reading data from unidirectional stream');

			output = await readData(stream);
		}
	}

	onMount(async () => {
		await $connection.ready;
		console.log('checking for unidirectional streams...');
		await receiveUnidirectional();
	});
</script>

<svelte:head>
	<title>Web Transport Web Client</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1>Web Transport Chat App</h1>
	<input
		type="text"
		id="input"
		placeholder="Hello, world!"
		bind:value={input}
		on:keypress={handleKey}
	/>
	<p>{output}</p>
</section>
