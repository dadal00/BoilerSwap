<script lang="ts">
	import { PUBLIC_MAX_CHARS } from '$env/static/public'

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
			required
			bind:value={password}
		/>
	</label>
</div>
