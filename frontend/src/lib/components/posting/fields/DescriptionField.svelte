<script lang="ts">
	import { PUBLIC_MAX_CHARS } from '$env/static/public'

	let { description_value = $bindable() } = $props<{ description_value: string }>()
</script>

Description
<textarea
	placeholder="Add any additional details, flaws, or special instructions..."
	class="w-full px-4 py-2 border rounded-lg h-24"
	bind:value={description_value}
	maxlength={Number(PUBLIC_MAX_CHARS)}
></textarea>
