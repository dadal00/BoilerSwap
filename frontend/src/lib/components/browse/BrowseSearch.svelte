<script lang="ts">
	import { ItemTypeIterable, LocationIterable, LocationLabels } from '$lib/models'
</script>

<div class="bg-white rounded-lg shadow-sm border p-6 mb-8">
	<div class="flex flex-col lg:flex-row gap-4">
		<div class="flex-1 relative">
			<input
				type="text"
				placeholder="Search for items..."
				class="w-full pl-10 pr-4 py-3 border rounded-lg"
			/>
			<span class="absolute left-3 top-1/2 transform -translate-y-1/2">🔍</span>
		</div>
		<select class="px-4 py-3 border rounded-lg">
			<option value="*">All Categories</option>
			{#each ItemTypeIterable as category}
				<option value={category}>{category}</option>
			{/each}
		</select>
		<select class="px-4 py-3 border rounded-lg">
			<option value="*">All Locations</option>
			{#each LocationIterable as category}
				<option value={category}>{LocationLabels[category]}</option>
			{/each}
		</select>
	</div>
	<div class="flex items-center justify-between mt-4 pt-4 border-t">
		<p class="text-gray-600">6 items available</p>
	</div>
</div>
