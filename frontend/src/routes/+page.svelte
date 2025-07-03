<script lang="ts">
	import Products from '$lib/components/browse/Products.svelte'
	import { ItemTypeIterable, LocationIterable, LocationLabels } from '$lib/models'
</script>

<section class="py-16 px-6 text-center bg-white">
	<div class="container mx-auto max-w-4xl">
		<h1 class="text-5xl font-bold text-gray-900 mb-4 fade-in">Give Away, Grab, Save Money!</h1>
		<p class="text-xl text-gray-600 mb-12 fade-in">
			The sustainable way for Purdue students to share, reuse, and reduce waste
		</p>

		<div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-12">
			<div class="text-center fade-in">
				<div class="text-3xl font-bold text-yellow-500 mb-2">1,247</div>
				<div class="text-gray-600">Items Saved from Trash</div>
			</div>
			<div class="text-center fade-in">
				<div class="text-3xl font-bold text-yellow-500 mb-2">892</div>
				<div class="text-gray-600">Active Students</div>
			</div>
			<div class="text-center fade-in">
				<div class="text-3xl font-bold text-yellow-500 mb-2">$15,430</div>
				<div class="text-gray-600">Money Saved</div>
			</div>
		</div>

		<div class="bg-gray-50 p-6 rounded-lg shadow-sm">
			<div class="flex flex-col md:flex-row gap-4 mb-4">
				<input
					type="text"
					placeholder="Search for items (e.g., desk, microwave, textbooks...)"
					class="flex-1 px-4 py-2 border rounded-lg"
				/>
				<button
					class="bg-yellow-400 text-gray-800 hover:bg-yellow-500 px-6 py-2 rounded-lg transition-colors"
				>
					🔍 Search
				</button>
			</div>

			<div class="flex flex-col md:flex-row gap-4">
				<select class="px-4 py-2 border rounded-lg">
					<option value="*">All Categories</option>
					{#each ItemTypeIterable as category}
						<option value={category}>{category}</option>
					{/each}
				</select>
				<select class="px-4 py-2 border rounded-lg">
					<option value="*">All Locations</option>
					{#each LocationIterable as category}
						<option value={category}>{LocationLabels[category]}</option>
					{/each}
				</select>
			</div>
		</div>
	</div>
</section>

<section class="py-16 px-6">
	<div class="container mx-auto">
		<h2 class="text-2xl font-bold text-center mb-8">Available Items</h2>
		<Products></Products>

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
