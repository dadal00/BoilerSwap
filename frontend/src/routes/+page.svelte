<script lang="ts">
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
					<option>All Categories</option>
					<option>Furniture</option>
					<option>Electronics</option>
					<option>Books</option>
					<option>Kitchen</option>
					<option>Clothing</option>
				</select>
				<select class="px-4 py-2 border rounded-lg">
					<option>All Locations</option>
					<option>Cary Quad</option>
					<option>Wiley Hall</option>
					<option>Harrison Hall</option>
					<option>Earhart</option>
				</select>
				<select class="px-4 py-2 border rounded-lg">
					<option>All Urgency</option>
					<option>Urgent (&lt; 24h)</option>
					<option>Soon (1-2 days)</option>
					<option>Flexible</option>
				</select>
			</div>
		</div>
	</div>
</section>

<section class="py-16 px-6">
	<div class="container mx-auto">
		<h2 class="text-2xl font-bold text-center mb-8">Available Items</h2>
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
						Moving out tomorrow! Comfortable office chair, barely used. Perfect for studying.
					</p>
					<div class="flex items-center text-sm text-gray-500 mb-2">
						<span class="mr-1">📍</span>
						Cary Quad - East
					</div>
					<div class="flex items-center text-sm text-red-600 font-medium">
						<span class="mr-1">⏰</span>
						Expires in 8 hours
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
						Compact fridge, great for dorm rooms. Clean and in excellent working condition.
					</p>
					<div class="flex items-center text-sm text-gray-500 mb-2">
						<span class="mr-1">📍</span>
						Wiley Hall
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
					<p class="text-gray-600 text-sm mb-4">
						Java programming textbook with my handwritten notes. Great for future CS students!
					</p>
					<div class="flex items-center text-sm text-gray-500 mb-2">
						<span class="mr-1">📍</span>
						Harrison Hall
					</div>
				</div>
			</div>
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

<footer class="bg-gray-800 text-white py-12 px-6">
	<div class="container mx-auto">
		<div class="grid grid-cols-1 md:grid-cols-4 gap-8">
			<div>
				<div class="flex items-center space-x-2 mb-4">
					<div
						class="w-6 h-6 bg-gradient-to-br from-yellow-400 to-amber-500 rounded-lg flex items-center justify-center"
					>
						<span class="text-white font-bold text-xs">BS</span>
					</div>
					<span class="text-xl font-bold">BoilerSwap</span>
				</div>
				<p class="text-gray-400">Making Purdue more sustainable, one swap at a time.</p>
			</div>

			<div>
				<h4 class="font-semibold mb-4">Quick Links</h4>
				<ul class="space-y-2 text-gray-400">
					<li><a href="how-it-works" class="hover:text-white">How It Works</a></li>
					<li><a href="about" class="hover:text-white">About</a></li>
					<li><a href="contact" class="hover:text-white">Contact Us</a></li>
					<li><a href="browse" class="hover:text-white">Browse Items</a></li>
				</ul>
			</div>

			<div>
				<h4 class="font-semibold mb-4">Resources</h4>
				<ul class="space-y-2 text-gray-400">
					<li><a href="safety-guidelines" class="hover:text-white">Safety Guidelines</a></li>
					<li><a href="faq" class="hover:text-white">FAQ</a></li>
					<li><a href="disclaimer" class="hover:text-white">Terms of Service</a></li>
					<li><a href="help-center" class="hover:text-white">Help Center</a></li>
				</ul>
			</div>

			<div>
				<h4 class="font-semibold mb-4">Support</h4>
				<ul class="space-y-2 text-gray-400">
					<li><a href="auth" class="hover:text-white">Login</a></li>
					<li><a href="auth" class="hover:text-white">Create Account</a></li>
					<li><a href="help-center" class="hover:text-white">Help Center</a></li>
					<li><a href="report-issue" class="hover:text-white">Report Issue</a></li>
				</ul>
			</div>
		</div>

		<div class="border-t border-gray-700 mt-8 pt-8 text-center text-gray-400">
			<p>&copy; 2024 BoilerSwap. Made with ❤️ by Purdue students, for Purdue students.</p>
		</div>
	</div>
</footer>

<a
	href="post"
	class="fixed bottom-6 right-6 w-14 h-14 rounded-full bg-gradient-to-r from-yellow-400 to-amber-500 hover:from-yellow-500 hover:to-amber-600 text-white shadow-lg transition-all hover:scale-110 flex items-center justify-center"
>
	<span class="text-2xl">+</span>
</a>

<style lang="postcss">
	@reference "tailwindcss";
</style>
