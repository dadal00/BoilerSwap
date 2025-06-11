<script lang="ts">
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
				<option>All Categories</option>
				<option>Furniture</option>
				<option>Electronics</option>
				<option>Books</option>
				<option>Kitchen</option>
				<option>Clothing</option>
			</select>
			<select class="px-4 py-3 border rounded-lg">
				<option>All Locations</option>
				<option>Wiley Hall</option>
				<option>Cary Quad</option>
				<option>Harrison Hall</option>
				<option>Earhart</option>
			</select>
			<select class="px-4 py-3 border rounded-lg">
				<option>Sort by</option>
				<option>Newest First</option>
				<option>Most Urgent</option>
				<option>By Location</option>
			</select>
		</div>
		<div class="flex items-center justify-between mt-4 pt-4 border-t">
			<p class="text-gray-600">6 items available</p>
		</div>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
		<div
			class="bg-white rounded-lg shadow-sm border-l-4 border-l-red-500 overflow-hidden card-hover urgent-pulse"
		>
			<div class="h-48 bg-gray-200 flex items-center justify-center">
				<div class="text-6xl">🪑</div>
			</div>
			<div class="p-6">
				<div class="flex justify-between items-start mb-2">
					<h3 class="font-semibold text-lg">Desk Chair - Great Condition</h3>
					<span class="bg-yellow-100 text-yellow-800 px-2 py-1 rounded text-sm">Furniture</span>
				</div>
				<p class="text-gray-600 text-sm mb-4">
					Moving out tomorrow! Comfortable office chair, barely used.
				</p>
				<div class="flex items-center text-sm text-gray-500 mb-2">
					<span class="mr-1">📍</span>Cary Quad - East
				</div>
				<div class="flex items-center text-sm text-red-600 font-medium">
					<span class="mr-1">⏰</span>Expires in 8 hours
				</div>
			</div>
		</div>

		<div
			class="bg-white rounded-lg shadow-sm border-l-4 border-l-yellow-500 overflow-hidden card-hover"
		>
			<div class="h-48 bg-gray-200 flex items-center justify-center">
				<div class="text-6xl">❄️</div>
			</div>
			<div class="p-6">
				<div class="flex justify-between items-start mb-2">
					<h3 class="font-semibold text-lg">Mini Fridge - Works Perfect</h3>
					<span class="bg-yellow-100 text-yellow-800 px-2 py-1 rounded text-sm">Electronics</span>
				</div>
				<p class="text-gray-600 text-sm mb-4">
					Compact fridge, great for dorm rooms. Clean condition.
				</p>
				<div class="flex items-center text-sm text-gray-500 mb-2">
					<span class="mr-1">📍</span>Wiley Hall
				</div>
			</div>
		</div>

		<div
			class="bg-white rounded-lg shadow-sm border-l-4 border-l-green-500 overflow-hidden card-hover"
		>
			<div class="h-48 bg-gray-200 flex items-center justify-center">
				<div class="text-6xl">📚</div>
			</div>
			<div class="p-6">
				<div class="flex justify-between items-start mb-2">
					<h3 class="font-semibold text-lg">CS 180 Textbook + Notes</h3>
					<span class="bg-yellow-100 text-yellow-800 px-2 py-1 rounded text-sm">Books</span>
				</div>
				<p class="text-gray-600 text-sm mb-4">Java programming textbook with handwritten notes.</p>
				<div class="flex items-center text-sm text-gray-500 mb-2">
					<span class="mr-1">📍</span>Harrison Hall
				</div>
			</div>
		</div>

		<div
			class="bg-white rounded-lg shadow-sm border-l-4 border-l-red-500 overflow-hidden card-hover urgent-pulse"
		>
			<div class="h-48 bg-gray-200 flex items-center justify-center">
				<div class="text-6xl">🍳</div>
			</div>
			<div class="p-6">
				<div class="flex justify-between items-start mb-2">
					<h3 class="font-semibold text-lg">Kitchen Supplies Set</h3>
					<span class="bg-yellow-100 text-yellow-800 px-2 py-1 rounded text-sm">Kitchen</span>
				</div>
				<p class="text-gray-600 text-sm mb-4">Pots, pans, utensils - everything you need!</p>
				<div class="flex items-center text-sm text-gray-500 mb-2">
					<span class="mr-1">📍</span>Third Street Suites
				</div>
				<div class="flex items-center text-sm text-red-600 font-medium">
					<span class="mr-1">⏰</span>3 hours left
				</div>
			</div>
		</div>

		<div
			class="bg-white rounded-lg shadow-sm border-l-4 border-l-yellow-500 overflow-hidden card-hover"
		>
			<div class="h-48 bg-gray-200 flex items-center justify-center">
				<div class="text-6xl">🖥️</div>
			</div>
			<div class="p-6">
				<div class="flex justify-between items-start mb-2">
					<h3 class="font-semibold text-lg">Gaming Monitor 24"</h3>
					<span class="bg-yellow-100 text-yellow-800 px-2 py-1 rounded text-sm">Electronics</span>
				</div>
				<p class="text-gray-600 text-sm mb-4">1080p, 144Hz, perfect for gaming or studying.</p>
				<div class="flex items-center text-sm text-gray-500 mb-2">
					<span class="mr-1">📍</span>Hillenbrand Hall
				</div>
			</div>
		</div>

		<div
			class="bg-white rounded-lg shadow-sm border-l-4 border-l-green-500 overflow-hidden card-hover"
		>
			<div class="h-48 bg-gray-200 flex items-center justify-center">
				<div class="text-6xl">🎨</div>
			</div>
			<div class="p-6">
				<div class="flex justify-between items-start mb-2">
					<h3 class="font-semibold text-lg">Dorm Room Decorations</h3>
					<span class="bg-yellow-100 text-yellow-800 px-2 py-1 rounded text-sm">Decor</span>
				</div>
				<p class="text-gray-600 text-sm mb-4">Posters, fairy lights, plants, and more.</p>
				<div class="flex items-center text-sm text-gray-500 mb-2">
					<span class="mr-1">📍</span>Earhart Hall
				</div>
			</div>
		</div>
	</div>
</div>

<a
	href="post"
	class="fixed bottom-6 right-6 w-14 h-14 rounded-full bg-gradient-to-r from-yellow-400 to-amber-500 hover:from-yellow-500 hover:to-amber-600 text-white shadow-lg transition-all hover:scale-110 flex items-center justify-center"
>
	<span class="text-2xl">+</span>
</a>
