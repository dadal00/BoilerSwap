<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_BACKEND_URL } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'

	let new_password: string = $state('')

	async function update() {
		if (new_password === '' || new_password.length > 100) {
			console.log('Invalid password')
			return
		}

		try {
			const response = await fetch(PUBLIC_BACKEND_URL + '/verify', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				credentials: 'include',
				body: JSON.stringify({ token: new_password })
			})

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`)
			}

			appState.setSignedIn(true)
			goto('/browse')
		} catch (err) {
			console.log('verification failed: ', err)
		}
	}
</script>

<form onsubmit={update} class="space-y-2">
	<input
		type="text"
		bind:value={new_password}
		placeholder="Enter your new password"
		class="border p-2 rounded w-full"
	/>
	<button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded"> Submit </button>
</form>
