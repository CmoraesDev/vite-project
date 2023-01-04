<script>
	import ToggleTheme from '$lib/ToggleTheme.svelte';
	import { isDarkMode, appTitle } from '../../src/stores/GlobalTheme';

	import '../app.css';
</script>

<div class={$isDarkMode ? 'dark' : 'light'}>
	<div class="h-screen bg-slate-50 text-black dark:bg-slate-600 dark:text-slate-50">
		<header>
			<!-- Lorem ipsum dolor sit. -->
			<div class="p-2 shadow-xl bg-slate-800 text-slate-100">
				<div class="flex justify-between p-2">
					<h1 class="text-3xl">{appTitle}</h1>
					<nav class="flex space-x-10">
						<a href="/">Index</a>
						<a href="/components">Components</a>
					</nav>
					<ToggleTheme />
				</div>
			</div>
		</header>

		<main>
			<slot />
		</main>

		<footer>
			<!-- Lorem ipsum dolor sit. -->
		</footer>
	</div>
</div>
