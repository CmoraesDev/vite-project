<script>
	import { isDarkMode } from "$lib/../stores/GlobalTheme";

    function toggle(){
        isDarkMode.set(!$isDarkMode)
    }
</script>

<button on:click={toggle} class="bg-indigo-300 text-indigo-900 p-2 rounded-full font-extrabold dark:bg-indigo-900 dark:text-indigo-100">Toggle DarkMode</button>