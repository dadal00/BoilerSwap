<script lang="ts">
	import '../app.css'
	import NavBar from '$lib/components/NavBar.svelte'
	import Footer from '$lib/components/Footer.svelte'
	import { afterNavigate } from '$app/navigation'

	let { children } = $props()

	let currentRoute = $state('/')
	afterNavigate((navigation) => {
		currentRoute = navigation.to?.url.pathname ?? '/'
	})
</script>

{#if !currentRoute.includes('/auth')}
	<NavBar />
{/if}

<main>
	{@render children()}
</main>

{#if !currentRoute.includes('/auth')}
	<Footer />
{/if}
