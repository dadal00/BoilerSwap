<script lang="ts">
	import { LocationIterable, LocationLabels, type Location } from '$lib/models'

	let { locationValue = $bindable() } = $props<{ locationValue: Location }>()
</script>

Pickup Location *
<select required class="w-full px-4 py-2 border rounded-lg" bind:value={locationValue}>
	{#each LocationIterable as category}
		<option value={category}>{LocationLabels[category]}</option>
	{/each}
</select>
