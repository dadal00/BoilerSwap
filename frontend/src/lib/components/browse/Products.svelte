<script lang="ts">
	import ProductCard from '$lib/components/ProductCard.svelte'
</script>

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
	/>

	<ProductCard
		emoji="snowflake"
		title="Mini Fridge - Works Perfect"
		category="Electronics"
		description="Compact fridge, great for dorm rooms. Clean condition."
		location="WileyHall"
		expiration="2 days"
		condition="Good"
		expirationColor="green"
	/>

	<ProductCard
		emoji="books"
		title="CS 180 Textbook + Notes"
		category="Books"
		description="Java programming textbook with handwritten notes."
		location="HarrisonHall"
		expiration="2 days"
		condition="Good"
		expirationColor="green"
	/>

	<ProductCard
		emoji="pan"
		title="Kitchen Supplies Set"
		category="Kitchen"
		description="Pots, pans, utensils - everything you need!"
		location="ThirdStreetSuites"
		expiration="3 hours left"
		condition="Good"
		expirationColor="red"
	/>

	<ProductCard
		emoji="monitor"
		title="Gaming Monitor 24"
		category="Electronics"
		description="1080p, 144Hz, perfect for gaming or studying."
		location="HillenbrandHall"
		expiration="2 days"
		condition="Good"
		expirationColor="green"
	/>

	<ProductCard
		emoji="decor"
		title="Dorm Room Decorations"
		category="Decor"
		description="Posters, fairy lights, plants, and more."
		location="EarhartHall"
		expiration="2 days"
		condition="Good"
		expirationColor="green"
	/>
</div>
