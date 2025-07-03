<script lang="ts">
	import { ItemTypeIterable, type ItemType } from '$lib/models'

	let { item_type_value = $bindable() } = $props<{ item_type_value: ItemType }>()
</script>

Item Type *
<select required class="w-full px-4 py-2 border rounded-lg" bind:value={item_type_value}>
	{#each ItemTypeIterable as category}
		<option value={category}>{category}</option>
	{/each}
</select>
