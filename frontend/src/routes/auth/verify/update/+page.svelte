<script lang="ts">
	import { goto } from '$app/navigation'
	import {
		PUBLIC_TEMP_SESSION_DURATION_SECS,
		PUBLIC_MAX_CHARS,
		PUBLIC_MIN_PASSWORD_LENGTH
	} from '$env/static/public'
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

<div class="container mx-auto px-6 py-16 max-w-md">
	<div class="bg-white rounded-lg shadow-sm border p-6">
		<form onsubmit={() => update(new_password)} class="space-y-4">
			<p class="text-gray-600 text-sm">Enter your new password.</p>
			<div>
				<label class="block text-sm font-medium mb-2">
					New Password
					<input
						type="text"
						bind:value={new_password}
						placeholder="abcdefghijklmnop"
						class="w-full px-4 py-2 border rounded-lg"
						maxlength={Number(PUBLIC_MAX_CHARS)}
						minlength={Number(PUBLIC_MIN_PASSWORD_LENGTH)}
						required
					/>
				</label>
				<p class="text-xs text-gray-500 mt-1">Must be {PUBLIC_MIN_PASSWORD_LENGTH}+ characters</p>
			</div>
			<button
				type="submit"
				class="w-full {appState.getLimited()
					? 'bg-gray-300 cursor-not-allowed'
					: 'bg-yellow-400 hover:bg-yellow-500'} text-gray-800 py-2 rounded-lg transition-colors"
			>
				Submit
			</button>
		</form>
	</div>
</div>
