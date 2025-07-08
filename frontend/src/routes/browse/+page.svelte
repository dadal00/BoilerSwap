<script lang="ts">
	import BrowseSearch from '$lib/components/browse/BrowseSearch.svelte'
	import Items from '$lib/components/browse/Items.svelte'
	import PageButtons from '$lib/components/browse/PageButtons.svelte'
</script>

<svelte:head>
	<title>Browse Items - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-8">
	<h1 class="text-3xl font-bold mb-8">Browse All Items</h1>
	<BrowseSearch />
	<Items />
	<PageButtons />
</div>
