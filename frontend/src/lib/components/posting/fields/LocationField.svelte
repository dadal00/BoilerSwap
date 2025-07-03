<script lang="ts">
	import { LocationIterable, LocationLabels, type Location } from '$lib/models'

	let { location_value = $bindable() } = $props<{ location_value: Location }>()
</script>

Pickup Location *
<select required class="w-full px-4 py-2 border rounded-lg" bind:value={location_value}>
	{#each LocationIterable as category}
		<option value={category}>{LocationLabels[category]}</option>
	{/each}
</select>
