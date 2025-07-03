<script lang="ts">
</script>

<svelte:head>
	<title>About - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-16 max-w-4xl">
	<div class="text-center mb-16">
		<h1 class="text-4xl font-bold text-gray-900 mb-4">About BoilerSwap</h1>
		<p class="text-xl text-gray-600">
			Building a sustainable community at Purdue, one swap at a time
		</p>
	</div>

	<section class="mb-16">
		<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-8">
			<h2 class="text-2xl font-bold text-center mb-4 flex items-center justify-center">
				<span class="text-3xl mr-3">🎯</span>
				Our Mission
			</h2>
			<p class="text-lg text-gray-700 text-center leading-relaxed">
				BoilerSwap exists to reduce waste, save money, and build community among Purdue students. We
				believe that one person's unwanted item can be another's treasure, and by facilitating these
				exchanges, we're making our campus more sustainable and our community stronger.
			</p>
		</div>
	</section>

	<section class="mb-16">
		<h2 class="text-2xl font-bold text-gray-900 mb-6">Our Story</h2>
		<div class="bg-white rounded-lg shadow-sm border p-8">
			<div class="prose prose-lg text-gray-600">
				<p class="mb-4">
					BoilerSwap was born from a simple observation: every semester, especially during move-out
					periods, tons of perfectly good items end up in dumpsters around campus. Meanwhile, other
					students are spending money on the same items at stores.
				</p>
				<p class="mb-4">
					Created by Purdue students for Purdue students, BoilerSwap bridges this gap by providing a
					platform where students can easily give away items they no longer need and find items they
					do need - all for free.
				</p>
				<p>
					What started as a solution to reduce waste has grown into a community-building platform
					that embodies the Boilermaker spirit of helping one another and making a positive impact.
				</p>
			</div>
		</div>
	</section>

	<section class="mb-16">
		<h2 class="text-2xl font-bold text-gray-900 mb-8">Our Values</h2>
		<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
			<div class="bg-white rounded-lg shadow-sm border p-6 text-center">
				<span class="text-4xl mb-4 block">🌱</span>
				<h3 class="text-xl font-semibold mb-2">Sustainability</h3>
				<p class="text-gray-600">
					We're committed to reducing waste and promoting reuse. Every item shared through
					BoilerSwap is one less item in a landfill and one less new item that needs to be
					manufactured.
				</p>
			</div>

			<div class="bg-white rounded-lg shadow-sm border p-6 text-center">
				<span class="text-4xl mb-4 block">👥</span>
				<h3 class="text-xl font-semibold mb-2">Community</h3>
				<p class="text-gray-600">
					We believe in the power of community. BoilerSwap connects students, builds relationships,
					and fosters the spirit of mutual aid that makes Purdue special.
				</p>
			</div>

			<div class="bg-white rounded-lg shadow-sm border p-6 text-center">
				<span class="text-4xl mb-4 block">❤️</span>
				<h3 class="text-xl font-semibold mb-2">Accessibility</h3>
				<p class="text-gray-600">
					Everyone deserves access to the items they need. BoilerSwap makes essential items
					available to all students, regardless of their financial situation.
				</p>
			</div>
		</div>
	</section>

	<section class="mb-16">
		<h2 class="text-2xl font-bold text-gray-900 mb-8">Our Impact</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-8">
			<div class="bg-white rounded-lg shadow-sm border p-6">
				<h3 class="text-xl font-semibold mb-4 flex items-center">
					<span class="text-2xl mr-3">♻️</span>
					Environmental Impact
				</h3>
				<div class="space-y-4">
					<div class="flex justify-between items-center">
						<span class="text-gray-600">Items Saved from Trash</span>
						<span class="font-bold text-2xl text-yellow-600">1,247</span>
					</div>
					<div class="flex justify-between items-center">
						<span class="text-gray-600">Estimated CO₂ Saved</span>
						<span class="font-bold text-2xl text-green-600">2.3 tons</span>
					</div>
					<div class="flex justify-between items-center">
						<span class="text-gray-600">Landfill Waste Prevented</span>
						<span class="font-bold text-2xl text-blue-600">5.8 tons</span>
					</div>
				</div>
			</div>

			<div class="bg-white rounded-lg shadow-sm border p-6">
				<h3 class="text-xl font-semibold mb-4 flex items-center">
					<span class="text-2xl mr-3">👥</span>
					Community Impact
				</h3>
				<div class="space-y-4">
					<div class="flex justify-between items-center">
						<span class="text-gray-600">Active Students</span>
						<span class="font-bold text-2xl text-blue-600">892</span>
					</div>
					<div class="flex justify-between items-center">
						<span class="text-gray-600">Money Saved</span>
						<span class="font-bold text-2xl text-green-600">$15,430</span>
					</div>
					<div class="flex justify-between items-center">
						<span class="text-gray-600">Successful Exchanges</span>
						<span class="font-bold text-2xl text-yellow-600">1,089</span>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>
