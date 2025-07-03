<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_TEMP_SESSION_DURATION_SECS, PUBLIC_MAX_CHARS } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
	import { Status } from '$lib/models'
	import { onDestroy, onMount } from 'svelte'
	import { update } from '$lib/helpers/auth'

	let new_password: string = $state('')
	let timer: number | null = null

	onMount(() => {
		if (!appState.getStatus(Status.isVerifyingUpdate)) {
			goto('/browse')
		}

		timer = setTimeout(
			() => {
				appState.setStatus(Status.isVerifyingUpdate, false)
			},
			Number(PUBLIC_TEMP_SESSION_DURATION_SECS) * 1000
		)
	})

	$effect(() => {
		if (!appState.getStatus(Status.isVerifyingUpdate)) {
			goto('/browse')
		}
	})

	onDestroy(() => {
		appState.setStatus(Status.isVerifyingUpdate, false)
		clearTimeout(timer!)
	})
</script>

<form onsubmit={() => update(new_password)} class="space-y-2">
	<input
		type="text"
		bind:value={new_password}
		placeholder="Enter your new password"
		class="border p-2 rounded w-full"
		maxlength={Number(PUBLIC_MAX_CHARS)}
		required
	/>
	<button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded"> Submit </button>
</form>
