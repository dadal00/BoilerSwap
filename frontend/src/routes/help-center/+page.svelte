<script lang="ts">
</script>

<svelte:head>
	<title>Help Center - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-16 max-w-4xl">
	<div class="text-center mb-12">
		<h1 class="text-4xl font-bold text-gray-900 mb-4">🏠 Help Center</h1>
		<p class="text-xl text-gray-600">
			Find answers to common questions and get help with BoilerSwap
		</p>
	</div>

	<div class="bg-white rounded-lg shadow-sm border p-6 mb-8">
		<div class="flex gap-4">
			<input
				type="text"
				placeholder="Search for help articles..."
				class="flex-1 px-4 py-3 border rounded-lg"
			/>
			<button class="bg-yellow-400 text-gray-800 hover:bg-yellow-500 px-6 py-3 rounded-lg">
				🔍 Search
			</button>
		</div>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-12">
		<a
			href="faq"
			class="bg-white rounded-lg shadow-sm border p-6 hover:shadow-md transition-shadow"
		>
			<div class="text-4xl mb-4">?</div>
			<h3 class="text-xl font-semibold mb-2">FAQ</h3>
			<p class="text-gray-600">Frequently asked questions about using BoilerSwap</p>
		</a>

		<a
			href="how-it-works"
			class="bg-white rounded-lg shadow-sm border p-6 hover:shadow-md transition-shadow"
		>
			<div class="text-4xl mb-4">📖</div>
			<h3 class="text-xl font-semibold mb-2">How It Works</h3>
			<p class="text-gray-600">Learn how to use BoilerSwap step by step</p>
		</a>

		<a
			href="safety-guidelines"
			class="bg-white rounded-lg shadow-sm border p-6 hover:shadow-md transition-shadow"
		>
			<div class="text-4xl mb-4">🛡️</div>
			<h3 class="text-xl font-semibold mb-2">Safety Guidelines</h3>
			<p class="text-gray-600">Stay safe while using BoilerSwap</p>
		</a>

		<a
			href="report-issue"
			class="bg-white rounded-lg shadow-sm border p-6 hover:shadow-md transition-shadow"
		>
			<div class="text-4xl mb-4">🚨</div>
			<h3 class="text-xl font-semibold mb-2">Report Issue</h3>
			<p class="text-gray-600">Report problems or inappropriate behavior</p>
		</a>

		<a
			href="contact"
			class="bg-white rounded-lg shadow-sm border p-6 hover:shadow-md transition-shadow"
		>
			<div class="text-4xl mb-4">📧</div>
			<h3 class="text-xl font-semibold mb-2">Contact Support</h3>
			<p class="text-gray-600">Get in touch with our support team</p>
		</a>

		<a
			href="disclaimer"
			class="bg-white rounded-lg shadow-sm border p-6 hover:shadow-md transition-shadow"
		>
			<div class="text-4xl mb-4">📋</div>
			<h3 class="text-xl font-semibold mb-2">Terms & Policies</h3>
			<p class="text-gray-600">Read our terms of service and policies</p>
		</a>
	</div>

	<div class="bg-white rounded-lg shadow-sm border p-8">
		<h2 class="text-2xl font-bold mb-6">📚 Popular Help Articles</h2>
		<div class="space-y-6">
			<div class="border-b pb-4">
				<h3 class="text-lg font-semibold mb-2">How do I post an item?</h3>
				<p class="text-gray-600 mb-2">Learn how to create your first item listing on BoilerSwap.</p>
				<a href="how-it-works" class="text-yellow-600 hover:underline">Read more →</a>
			</div>

			<div class="border-b pb-4">
				<h3 class="text-lg font-semibold mb-2">What should I do if someone doesn't show up?</h3>
				<p class="text-gray-600 mb-2">Steps to take when a pickup doesn't go as planned.</p>
				<a href="faq" class="text-yellow-600 hover:underline">Read more →</a>
			</div>

			<div class="border-b pb-4">
				<h3 class="text-lg font-semibold mb-2">How do I verify my Purdue email?</h3>
				<p class="text-gray-600 mb-2">Instructions for verifying your @purdue.edu email address.</p>
				<a href="faq" class="text-yellow-600 hover:underline">Read more →</a>
			</div>

			<div class="border-b pb-4">
				<h3 class="text-lg font-semibold mb-2">Is BoilerSwap safe to use?</h3>
				<p class="text-gray-600 mb-2">Learn about our safety measures and best practices.</p>
				<a href="safety-guidelines" class="text-yellow-600 hover:underline">Read more →</a>
			</div>

			<div class="pb-4">
				<h3 class="text-lg font-semibold mb-2">Can I edit my post after submitting?</h3>
				<p class="text-gray-600 mb-2">Information about editing and managing your item listings.</p>
				<a href="faq" class="text-yellow-600 hover:underline">Read more →</a>
			</div>
		</div>
	</div>

	<div class="mt-12 bg-yellow-50 border border-yellow-200 rounded-lg p-8 text-center">
		<h2 class="text-2xl font-bold mb-4">Still Need Help?</h2>
		<p class="text-gray-600 mb-6">
			Can't find what you're looking for? Our support team is here to help!
		</p>
		<div class="flex flex-col sm:flex-row gap-4 justify-center">
			<a
				href="contact"
				class="bg-yellow-400 text-gray-800 hover:bg-yellow-500 px-8 py-3 rounded-lg font-medium"
				>Contact Support</a
			>
			<a
				href="report-issue"
				class="border border-gray-300 text-gray-700 hover:bg-gray-50 px-8 py-3 rounded-lg font-medium"
				>Report an Issue</a
			>
		</div>
	</div>
</div>
