<script lang="ts">
	import { PUBLIC_CODE_LENGTH } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
	import { verify, verify_forget, resend } from '$lib/helpers/auth'
	import type { VerifcationType } from '$lib/models'
	import { onDestroy, onMount } from 'svelte'

	let {
		auth_code = $bindable(),
		verification_type,
		timer = $bindable()
	} = $props<{
		auth_code: string
		verification_type: VerifcationType
		timer: number | null
	}>()

	let resendSeconds: number = $state(30)
	let resendTimer: number | undefined = $state(undefined)
	let error: string = $derived(appState.getAuthError())

	function startResendCooldown() {
		clearInterval(resendTimer)
		resendSeconds = 30
		resendTimer = setInterval(() => {
			resendSeconds--
			if (resendSeconds <= 0) {
				clearInterval(resendTimer)
			}
		}, 1000)
	}

	onMount(() => {
		appState.setAuthError('')
		startResendCooldown()
	})

	onDestroy(() => {
		appState.setAuthError('')
	})
</script>

<div class="container mx-auto px-6 py-16 max-w-md">
	<div class="bg-white rounded-lg shadow-sm border p-6">
		<form
			onsubmit={() => {
				verification_type == 'verify' ? verify(auth_code) : verify_forget(auth_code)
			}}
			class="space-y-4"
		>
			<p class="text-gray-600 text-sm">Enter the verification code sent to your email.</p>
			{#if error != ''}
				<p class="text-red-600 text-sm font-medium text-center">{error}</p>
			{/if}
			<div>
				<label class="block text-sm font-medium mb-2">
					Verification Code
					<input
						type="text"
						bind:value={auth_code}
						placeholder="123456"
						class="w-full px-4 py-2 border rounded-lg"
						minlength={Number(PUBLIC_CODE_LENGTH)}
						maxlength={Number(PUBLIC_CODE_LENGTH)}
						required
					/>
				</label>
				<button
					type="button"
					class="{resendSeconds != 0
						? 'cursor-not-allowed'
						: 'hover:underline'} text-yellow-600 text-sm mt-2 block"
					onclick={() => {
						appState.setAuthError('')
						resend(resendSeconds)
						startResendCooldown()
					}}
				>
					Resend code {resendSeconds != 0 ? 'in ' + resendSeconds + ' seconds' : ''}
				</button>
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
