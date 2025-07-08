<script lang="ts">
	import { PUBLIC_MAX_CHARS, PUBLIC_MIN_PASSWORD_LENGTH } from '$env/static/public'

	let { password = $bindable(), displayName } = $props<{
		password: string
		displayName: string
	}>()
</script>

<div>
	<label class="block text-sm font-medium mb-2">
		{displayName}
		<input
			type="password"
			class="w-full px-4 py-2 border rounded-lg"
			maxlength={Number(PUBLIC_MAX_CHARS)}
			minlength={Number(PUBLIC_MIN_PASSWORD_LENGTH)}
			required
			bind:value={password}
		/>
	</label>
	<p class="text-xs text-gray-500 mt-1">Must be {PUBLIC_MIN_PASSWORD_LENGTH}+ characters</p>
</div>
