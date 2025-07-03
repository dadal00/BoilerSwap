<script lang="ts">
	import { PUBLIC_MAX_CHARS } from '$env/static/public'

	let { title_value = $bindable() } = $props<{ title_value: string }>()
</script>

Title *
<input
	type="text"
	required
	placeholder="e.g., IKEA Desk Chair - Great Condition"
	class="w-full px-4 py-2 border rounded-lg"
	bind:value={title_value}
	maxlength={Number(PUBLIC_MAX_CHARS)}
/>
