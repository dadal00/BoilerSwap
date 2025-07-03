<script lang="ts">
	import { ItemTypeIterable, type ItemType } from '$lib/models'

	let { itemTypeValue = $bindable() } = $props<{ itemTypeValue: ItemType }>()
</script>

Item Type *
<select required class="w-full px-4 py-2 border rounded-lg" bind:value={itemTypeValue}>
	{#each ItemTypeIterable as category}
		<option value={category}>{category}</option>
	{/each}
</select>
