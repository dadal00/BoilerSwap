<script lang="ts">
	import { PUBLIC_MAX_CHARS } from '$env/static/public'

	let { descriptionValue = $bindable() } = $props<{ descriptionValue: string }>()
</script>

Description
<textarea
	placeholder="Add any additional details, flaws, or special instructions..."
	class="w-full px-4 py-2 border rounded-lg h-24"
	bind:value={descriptionValue}
	maxlength={Number(PUBLIC_MAX_CHARS)}
></textarea>
