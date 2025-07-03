<script lang="ts">
	import ProductCard from '$lib/components/ProductCard.svelte'
	import { ItemTypeIterable, LocationIterable, LocationLabels } from '$lib/models'
</script>

<svelte:head>
	<title>Browse Items - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-8">
	<h1 class="text-3xl font-bold mb-8">Browse All Items</h1>

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

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
		<ProductCard
			emoji="chair"
			title="Desk Chair - Great Condition"
			category="Furniture"
			description="Moving out tomorrow! Comfortable office chair, barely used."
			location="CaryQuadEast"
			expiration="8 hours"
			condition="Good"
			expirationColor="red"
		></ProductCard>

		<ProductCard
			emoji="snowflake"
			title="Mini Fridge - Works Perfect"
			category="Electronics"
			description="Compact fridge, great for dorm rooms. Clean condition."
			location="WileyHall"
			expiration="2 days"
			condition="Good"
			expirationColor="green"
		></ProductCard>

		<ProductCard
			emoji="books"
			title="CS 180 Textbook + Notes"
			category="Books"
			description="Java programming textbook with handwritten notes."
			location="HarrisonHall"
			expiration="2 days"
			condition="Good"
			expirationColor="green"
		></ProductCard>

		<ProductCard
			emoji="pan"
			title="Kitchen Supplies Set"
			category="Kitchen"
			description="Pots, pans, utensils - everything you need!"
			location="ThirdStreetSuites"
			expiration="3 hours left"
			condition="Good"
			expirationColor="red"
		></ProductCard>

		<ProductCard
			emoji="monitor"
			title="Gaming Monitor 24"
			category="Electronics"
			description="1080p, 144Hz, perfect for gaming or studying."
			location="HillenbrandHall"
			expiration="2 days"
			condition="Good"
			expirationColor="green"
		></ProductCard>

		<ProductCard
			emoji="decor"
			title="Dorm Room Decorations"
			category="Decor"
			description="Posters, fairy lights, plants, and more."
			location="EarhartHall"
			expiration="2 days"
			condition="Good"
			expirationColor="green"
		></ProductCard>
	</div>
</div>

<a
	href="post"
	class="fixed bottom-6 right-6 w-14 h-14 rounded-full bg-gradient-to-r from-yellow-400 to-amber-500 hover:from-yellow-500 hover:to-amber-600 text-white shadow-lg transition-all hover:scale-110 flex items-center justify-center"
>
	<span class="text-2xl">+</span>
</a>
