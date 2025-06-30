<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_TEMP_SESSION_DURATION_SECS } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
	import { Status } from '$lib/models'
	import { onDestroy, onMount } from 'svelte'
	import { verify } from '$lib/auth'

	let auth_code: string = $state('')
	let timer: number | null = null

	onMount(() => {
		if (!appState.getStatus(Status.isVerifying)) {
			goto('/browse')
		}

		timer = setTimeout(
			() => {
				appState.setStatus(Status.isVerifying, false)
			},
			Number(PUBLIC_TEMP_SESSION_DURATION_SECS) * 1000
		)
	})

	$effect(() => {
		if (!appState.getStatus(Status.isVerifying)) {
			goto('/browse')
		}
	})

	onDestroy(() => {
		appState.setStatus(Status.isVerifying, false)
		clearTimeout(timer!)
	})
</script>

<form onsubmit={() => verify(auth_code)} class="space-y-2">
	<input
		type="text"
		bind:value={auth_code}
		placeholder="Enter your code"
		class="border p-2 rounded w-full"
	/>
	<button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded"> Submit </button>
</form>
