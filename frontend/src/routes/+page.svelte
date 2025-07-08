<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_MAX_CHARS } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
	import Items from '$lib/components/browse/Items.svelte'
	import SearchFilters from '$lib/components/browse/SearchFilters.svelte'
	import { type Condition, type ItemType, type Location } from '$lib/models'

	let query: string = $state('')
	let itemTypeFilter: ItemType | '' = $state('')
	let locationFilter: Location | '' = $state('')
	let conditionFilter: Condition | '' = $state('')

	function transitionSearch(_: MouseEvent) {
		appState.setQuery(query)
		appState.setItemTypeFilter(itemTypeFilter)
		appState.setLocationFilter(locationFilter)
		appState.setConditionFilter(conditionFilter)
		goto('/browse')
	}
</script>

<section class="py-16 px-6 text-center bg-white">
	<div class="container mx-auto max-w-4xl">
		<h1 class="text-5xl font-bold text-gray-900 mb-4 fade-in">Give Away, Grab, Save Money!</h1>
		<p class="text-xl text-gray-600 mb-12 fade-in">
			The sustainable way for Purdue students to share, reuse, and reduce waste
		</p>

		<div class="bg-gray-50 p-6 rounded-lg shadow-sm">
			<div class="flex flex-col md:flex-row gap-4 mb-4">
				<input
					type="text"
					placeholder="Search for items (e.g., desk, microwave, textbooks...)"
					class="flex-1 px-4 py-2 border rounded-lg"
					maxlength={Number(PUBLIC_MAX_CHARS)}
					bind:value={query}
				/>
				<button
					onclick={transitionSearch}
					class="bg-yellow-400 text-gray-800 hover:bg-yellow-500 px-6 py-2 rounded-lg transition-colors"
				>
					🔍 Search
				</button>
			</div>

			<div class="flex flex-col md:flex-row gap-4">
				<SearchFilters bind:itemTypeFilter bind:locationFilter bind:conditionFilter />
			</div>
		</div>
	</div>
</section>

<section class="py-16 px-6">
	<div class="container mx-auto">
		<h2 class="text-2xl font-bold text-center mb-8">Available Items</h2>
		<Items />

		<div class="text-center mt-8">
			<a
				href="browse"
				class="bg-gradient-to-r from-yellow-400 to-amber-500 hover:from-yellow-500 hover:to-amber-600 text-white px-8 py-3 rounded-lg text-lg font-medium"
			>
				View All Items
			</a>
		</div>
	</div>
</section>

<style lang="postcss">
	@reference "tailwindcss";
</style>
