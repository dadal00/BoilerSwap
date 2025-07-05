<script lang="ts">
	import {
		ConditionIterable,
		ItemTypeIterable,
		LocationIterable,
		LocationLabels,
		type Condition,
		type ItemType,
		type Location
	} from '$lib/models'

	let {
		itemTypeFilter = $bindable(),
		locationFilter = $bindable(),
		conditionFilter = $bindable()
	} = $props<{
		itemTypeFilter: ItemType | ''
		locationFilter: Location | ''
		conditionFilter: Condition | ''
	}>()
</script>

<select class="px-4 py-3 border rounded-lg" bind:value={itemTypeFilter}>
	<option value="">All Categories</option>
	{#each ItemTypeIterable as category}
		<option value={category}>{category}</option>
	{/each}
</select>
<select class="px-4 py-3 border rounded-lg" bind:value={locationFilter}>
	<option value="">All Locations</option>
	{#each LocationIterable as category}
		<option value={category}>{LocationLabels[category]}</option>
	{/each}
</select>
<select class="px-4 py-3 border rounded-lg" bind:value={conditionFilter}>
	<option value="">All Conditions</option>
	{#each ConditionIterable as category}
		<option value={category}>{category}</option>
	{/each}
</select>
