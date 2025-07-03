<script lang="ts">
	import '../app.css'
	import NavBar from '$lib/components/layout/NavBar.svelte'
	import Footer from '$lib/components/layout/Footer.svelte'
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

{#if !currentRoute.includes('/auth') && !currentRoute.includes('/post')}
	<a
		href="post"
		class="fixed bottom-6 right-6 w-14 h-14 rounded-full bg-gradient-to-r from-yellow-400 to-amber-500 hover:from-yellow-500 hover:to-amber-600 text-white shadow-lg transition-all hover:scale-110 flex items-center justify-center"
	>
		<span class="text-2xl">+</span>
	</a>
{/if}
