<script lang="ts">
	import { PUBLIC_CODE_LENGTH } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
	import { verify, verify_forget } from '$lib/helpers/auth'
	import type { VerifcationType } from '$lib/models'

	let { auth_code = $bindable(), verification_type } = $props<{
		auth_code: string
		verification_type: VerifcationType
	}>()
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
				<p class="text-xs text-gray-500 mt-1">Must be {Number(PUBLIC_CODE_LENGTH)} numbers</p>
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
