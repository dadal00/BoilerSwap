<script lang="ts"></script>

<svelte:head>
	<title>How It Works - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-16 max-w-4xl">
	<div class="text-center mb-16">
		<h1 class="text-4xl font-bold text-gray-900 mb-4">How BoilerSwap Works</h1>
		<p class="text-xl text-gray-600">
			Three simple steps to reduce waste and help fellow Boilermakers
		</p>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-16">
		<div class="text-center bg-white p-6 rounded-lg shadow-sm">
			<div
				class="w-16 h-16 bg-yellow-100 rounded-full flex items-center justify-center mx-auto mb-4"
			>
				<span class="text-2xl">📤</span>
			</div>
			<h3 class="text-xl font-semibold mb-2">1. Post Your Item</h3>
			<p class="text-gray-600">
				Take a quick photo, add a description, and set your pickup location. It takes less than 2
				minutes!
			</p>
		</div>

		<div class="text-center bg-white p-6 rounded-lg shadow-sm">
			<div
				class="w-16 h-16 bg-yellow-100 rounded-full flex items-center justify-center mx-auto mb-4"
			>
				<span class="text-2xl">🔍</span>
			</div>
			<h3 class="text-xl font-semibold mb-2">2. Students Find & Claim</h3>
			<p class="text-gray-600">
				Other students browse, search, and claim items they need. They'll contact you directly to
				arrange pickup.
			</p>
		</div>

		<div class="text-center bg-white p-6 rounded-lg shadow-sm">
			<div
				class="w-16 h-16 bg-yellow-100 rounded-full flex items-center justify-center mx-auto mb-4"
			>
				<span class="text-2xl">🤝</span>
			</div>
			<h3 class="text-xl font-semibold mb-2">3. Make the Exchange</h3>
			<p class="text-gray-600">
				Meet up, hand over the item, and feel good about helping a fellow student while saving the
				environment!
			</p>
		</div>
	</div>

	<div class="space-y-12">
		<section>
			<h2 class="text-2xl font-bold text-gray-900 mb-6">For Item Givers</h2>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="bg-white rounded-lg shadow-sm border p-6">
					<h3 class="text-xl font-semibold mb-4 flex items-center">
						<span class="text-2xl mr-3">📤</span>
						Posting Items
					</h3>
					<ul class="space-y-2 text-gray-600">
						<li>• Choose from predefined item categories</li>
						<li>• Rate the condition honestly</li>
						<li>• Add photos and descriptions</li>
						<li>• Set pickup location and urgency</li>
						<li>• Items auto-expire based on urgency</li>
					</ul>
				</div>

				<div class="bg-white rounded-lg shadow-sm border p-6">
					<h3 class="text-xl font-semibold mb-4 flex items-center">
						<span class="text-2xl mr-3">💬</span>
						Managing Requests
					</h3>
					<ul class="space-y-2 text-gray-600">
						<li>• Receive notifications when items are claimed</li>
						<li>• Communicate directly with interested students</li>
						<li>• Arrange convenient pickup times</li>
						<li>• Mark items as picked up when complete</li>
						<li>• Build your community reputation</li>
					</ul>
				</div>
			</div>
		</section>

		<section>
			<h2 class="text-2xl font-bold text-gray-900 mb-6">For Item Seekers</h2>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="bg-white rounded-lg shadow-sm border p-6">
					<h3 class="text-xl font-semibold mb-4 flex items-center">
						<span class="text-2xl mr-3">🔍</span>
						Finding Items
					</h3>
					<ul class="space-y-2 text-gray-600">
						<li>• Browse by category, location, or urgency</li>
						<li>• Use search to find specific items</li>
						<li>• See item condition and photos</li>
						<li>• Check pickup locations and times</li>
						<li>• Get notified about urgent items</li>
					</ul>
				</div>

				<div class="bg-white rounded-lg shadow-sm border p-6">
					<h3 class="text-xl font-semibold mb-4 flex items-center">
						<span class="text-2xl mr-3">🤝</span>
						Claiming Items
					</h3>
					<ul class="space-y-2 text-gray-600">
						<li>• Click "Claim It!" to express interest</li>
						<li>• Message the poster to arrange pickup</li>
						<li>• Coordinate meeting times and locations</li>
						<li>• Be respectful and punctual</li>
						<li>• Leave feedback for the community</li>
					</ul>
				</div>
			</div>
		</section>

		<section>
			<h2 class="text-2xl font-bold text-gray-900 mb-6">Safety & Guidelines</h2>
			<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
				<div class="bg-white rounded-lg shadow-sm border p-6">
					<h3 class="text-xl font-semibold mb-4 flex items-center">
						<span class="text-2xl mr-3">🛡️</span>
						Safety First
					</h3>
					<ul class="space-y-2 text-gray-600 text-sm">
						<li>• Meet in public, well-lit areas</li>
						<li>• Bring a friend if possible</li>
						<li>• Trust your instincts</li>
						<li>• Use Purdue email for verification</li>
					</ul>
				</div>

				<div class="bg-white rounded-lg shadow-sm border p-6">
					<h3 class="text-xl font-semibold mb-4 flex items-center">
						<span class="text-2xl mr-3">⏰</span>
						Be Timely
					</h3>
					<ul class="space-y-2 text-gray-600 text-sm">
						<li>• Respond to messages quickly</li>
						<li>• Show up on time for pickups</li>
						<li>• Cancel if plans change</li>
						<li>• Respect urgency levels</li>
					</ul>
				</div>

				<div class="bg-white rounded-lg shadow-sm border p-6">
					<h3 class="text-xl font-semibold mb-4 flex items-center">
						<span class="text-2xl mr-3">👥</span>
						Community Spirit
					</h3>
					<ul class="space-y-2 text-gray-600 text-sm">
						<li>• Be honest about item conditions</li>
						<li>• Help fellow Boilermakers</li>
						<li>• Keep the platform positive</li>
						<li>• Report any issues</li>
					</ul>
				</div>
			</div>
		</section>
	</div>

	<div class="text-center mt-16 p-8 bg-yellow-50 rounded-lg">
		<h2 class="text-2xl font-bold text-gray-900 mb-4">Ready to Start Swapping?</h2>
		<p class="text-gray-600 mb-6">
			Join hundreds of Purdue students making campus more sustainable
		</p>
		<div class="flex flex-col sm:flex-row gap-4 justify-center">
			<a
				href="post"
				class="bg-yellow-400 text-gray-800 hover:bg-yellow-500 px-8 py-3 rounded-lg font-medium"
				>Post Your First Item</a
			>
			<a
				href="browse"
				class="border border-gray-300 text-gray-700 hover:bg-gray-50 px-8 py-3 rounded-lg font-medium"
				>Browse Available Items</a
			>
		</div>
	</div>
</div>
