<script lang="ts">
	import { appState } from '$lib/AppState.svelte'
	import ItemCard from '$lib/components/ItemCard.svelte'
	import { search } from '$lib/meiliClient'
	import { ItemFields } from '$lib/models'

	$effect(() => {
		const fullQuery = appState.getFullQuery()
		search(
			fullQuery.query,
			fullQuery[ItemFields.ITEM_TYPE],
			fullQuery[ItemFields.LOCATION],
			fullQuery[ItemFields.CONDITION]
		)
	})
</script>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 products-display">
	{#each appState.getHits() as hit}
		<ItemCard item={hit} />
	{/each}
</div>
