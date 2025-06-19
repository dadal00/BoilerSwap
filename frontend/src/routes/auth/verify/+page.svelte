<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_BACKEND_URL } from '$env/static/public'

	let auth_code: string = $state('')

	async function verify() {
		if (!/^\d+$/.test(auth_code)) {
			console.log('Verification failed: only numbers')
			return
		}

		try {
			const response = await fetch(PUBLIC_BACKEND_URL + '/verify', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				credentials: 'include',
				body: JSON.stringify({ token: auth_code })
			})

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`)
			}

			goto('/browse')
		} catch (err) {
			console.log('verification failed: ', err)
		}
	}
</script>

<form onsubmit={verify} class="space-y-2">
	<input
		type="text"
		bind:value={auth_code}
		placeholder="Enter your code"
		class="border p-2 rounded w-full"
	/>
	<button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded"> Submit </button>
</form>
