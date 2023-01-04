<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let command = '';
	let commandResult = '';

	async function execCommand() {
		commandResult = JSON.parse(await invoke('execute_command', { command }));
	}
</script>

<div>
	<input
		id="greet-input"
		class="text-black font-mono"
		placeholder="Enter a command"
		bind:value={command}
	/>
	<button class="bg-blue-500 px-4 rounded-lg" on:click={execCommand}>Execute</button>
	<div class="flex flex-col space-y-6 m-10">
		<div class="bg-slate-300 text-black p-2">
			<span class="font-extrabold">Return Code</span>
			<pre class="font-mono">{commandResult.rc}</pre>
		</div>
		<div class="bg-slate-300 text-black p-2">
			<span class="font-extrabold">Output</span>
			<pre class="font-mono">{commandResult.stdout}</pre>
		</div>
		<div class="bg-slate-300 text-black p-2">
			<span class="font-extrabold">Error</span>
			<pre class="font-mono">{commandResult.stderr}</pre>
		</div>
	</div>
</div>
