<script lang="ts">
	import '../app.css'
	import NavBar from '$lib/components/NavBar.svelte'
	import Footer from '$lib/components/Footer.svelte'
	import { afterNavigate } from '$app/navigation'
	import { appState } from '$lib/AppState.svelte'
	import { Status } from '$lib/models'
	import { onDestroy, onMount } from 'svelte'

	let { children, data } = $props()
	let interval: ReturnType<typeof setInterval>
	let currentRoute = $state('/')

	appState.setStatus(Status.isSignedIn, data.signedIn)

	afterNavigate((navigation) => {
		currentRoute = navigation.to?.url.pathname ?? '/'
	})

	onMount(() => {
		refreshToken()

		interval = setInterval(refreshToken, 270000)
	})

	onDestroy(() => {
		clearInterval(interval)
	})

	async function refreshToken() {
		try {
			await fetch('/', {
				method: 'HEAD',
				headers: {
					'x-refresh': 'true'
				}
			})
		} catch (e) {}
	}
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
