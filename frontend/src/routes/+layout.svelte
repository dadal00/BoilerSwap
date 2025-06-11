<script lang="ts">
	import '../app.css';
	import NavBar from '$lib/components/NavBar.svelte';
	import { afterNavigate } from '$app/navigation';

	let { children } = $props();

	let currentRoute = $state('/');
	afterNavigate((navigation) => {
		currentRoute = navigation.to?.url.pathname as string;
	});
</script>

{#if !currentRoute.includes('/auth')}
	<NavBar></NavBar>
{/if}

{@render children()}
