<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_TEMP_SESSION_DURATION_SECS } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
	import { Status } from '$lib/models'
	import { onDestroy, onMount } from 'svelte'
	import { verify_forget } from '$lib/auth'

	let auth_code: string = $state('')
	let timer: number | null = null

	onMount(() => {
		if (!appState.getStatus(Status.isVerifyingForgot)) {
			goto('/browse')
		}

		timer = setTimeout(() => {
			appState.setStatus(Status.isVerifyingForgot, false)
		}, PUBLIC_TEMP_SESSION_DURATION_SECS * 1000)
	})

	$effect(() => {
		if (!appState.getStatus(Status.isVerifyingForgot)) {
			goto('/browse')
		}
	})

	onDestroy(() => {
		appState.setStatus(Status.isVerifyingForgot, false)
		clearTimeout(timer!)
	})
</script>

<form onsubmit={() => verify_forget(auth_code)} class="space-y-2">
	<input
		type="text"
		bind:value={auth_code}
		placeholder="Enter your code"
		class="border p-2 rounded w-full"
	/>
	<button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded"> Submit </button>
</form>
