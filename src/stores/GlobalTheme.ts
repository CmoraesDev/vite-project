import { writable } from "svelte/store";

export let isDarkMode = writable(true);
export const appTitle = "vite-project"