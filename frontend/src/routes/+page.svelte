<script lang="ts">
	import ProductCard from '$lib/components/ProductCard.svelte'
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
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<ProductCard
				emoji="chair"
				title="Desk Chair - Great Condition"
				category="Furniture"
				description="Moving out tomorrow! Comfortable office chair, barely used. Perfect for studying."
				location="CaryQuadEast"
				expiration="8 hours"
				condition="Excellent"
				expirationColor="red"
			></ProductCard>

			<ProductCard
				emoji="snowflake"
				title="Mini Fridge - Works Perfect"
				category="Electronics"
				description="Compact fridge, great for dorm rooms. Clean and in excellent working condition."
				location="WileyHall"
				expiration="3 days"
				condition="Fair"
				expirationColor="green"
			></ProductCard>

			<ProductCard
				emoji="books"
				title="CS 180 Textbook + Notes"
				category="Books"
				description="Java programming textbook with my handwritten notes. Great for future CS students!"
				location="HarrisonHall"
				expiration="1 day"
				condition="Good"
				expirationColor="yellow"
			></ProductCard>
		</div>

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

<a
	href="post"
	class="fixed bottom-6 right-6 w-14 h-14 rounded-full bg-gradient-to-r from-yellow-400 to-amber-500 hover:from-yellow-500 hover:to-amber-600 text-white shadow-lg transition-all hover:scale-110 flex items-center justify-center"
>
	<span class="text-2xl">+</span>
</a>

<style lang="postcss">
	@reference "tailwindcss";
</style>
