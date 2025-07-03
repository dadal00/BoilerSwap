<script lang="ts">
	let activeCategory = $state('login')

	function showCategory(category: string) {
		activeCategory = category
	}
</script>

<svelte:head>
	<title>FAQ - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-16 max-w-4xl">
	<div class="text-center mb-12">
		<h1 class="text-4xl font-bold text-gray-900 mb-4">? Frequently Asked Questions</h1>
		<p class="text-xl text-gray-600">Find quick answers to common questions about BoilerSwap</p>
	</div>

	<div class="bg-white rounded-lg shadow-sm border p-6 mb-8">
		<div class="flex gap-4">
			<input type="text" placeholder="Search FAQ..." class="flex-1 px-4 py-3 border rounded-lg" />
			<button class="bg-yellow-400 text-gray-800 hover:bg-yellow-500 px-6 py-3 rounded-lg">
				🔍 Search
			</button>
		</div>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
		<button
			onclick={() => showCategory('login')}
			class="bg-white rounded-lg shadow-sm border p-6 text-left hover:shadow-md transition-shadow"
		>
			<div class="text-3xl mb-3">🚀</div>
			<h3 class="text-lg font-semibold">Getting Started</h3>
			<p class="text-gray-600 text-sm">Account setup and first steps</p>
		</button>

		<button
			onclick={() => showCategory('posting')}
			class="bg-white rounded-lg shadow-sm border p-6 text-left hover:shadow-md transition-shadow"
		>
			<div class="text-3xl mb-3">📝</div>
			<h3 class="text-lg font-semibold">Posting Items</h3>
			<p class="text-gray-600 text-sm">How to create great listings</p>
		</button>

		<button
			onclick={() => showCategory('safety')}
			class="bg-white rounded-lg shadow-sm border p-6 text-left hover:shadow-md transition-shadow"
		>
			<div class="text-3xl mb-3">🛡️</div>
			<h3 class="text-lg font-semibold">Safety & Security</h3>
			<p class="text-gray-600 text-sm">Staying safe on BoilerSwap</p>
		</button>
	</div>

	<div class="bg-white rounded-lg shadow-sm border p-8">
		{#if activeCategory === 'login'}
			<h2 class="text-2xl font-bold mb-6">🚀 Getting Started</h2>
			<div class="space-y-6">
				<div class="border-b pb-4">
					<h3 class="text-lg font-semibold mb-2">How do I create an account?</h3>
					<p class="text-gray-600">
						You need a valid @purdue.edu email address to sign up. Click "Sign In" in the header,
						then "Sign Up" and follow the verification process.
					</p>
				</div>

				<div class="border-b pb-4">
					<h3 class="text-lg font-semibold mb-2">Is BoilerSwap really free?</h3>
					<p class="text-gray-600">
						Yes! BoilerSwap is completely free to use. All items shared on the platform are given
						away for free by fellow students.
					</p>
				</div>

				<div class="border-b pb-4">
					<h3 class="text-lg font-semibold mb-2">Who can use BoilerSwap?</h3>
					<p class="text-gray-600">
						BoilerSwap is exclusively for Purdue University students, faculty, and staff with valid
						@purdue.edu email addresses.
					</p>
				</div>

				<div class="pb-4">
					<h3 class="text-lg font-semibold mb-2">How do I verify my Purdue email?</h3>
					<p class="text-gray-600">
						After signing up, we'll send a verification link to your @purdue.edu email. Click the
						link to activate your account.
					</p>
				</div>
			</div>
		{:else if activeCategory === 'posting'}
			<h2 class="text-2xl font-bold mb-6">📝 Posting Items</h2>
			<div class="space-y-6">
				<div class="border-b pb-4">
					<h3 class="text-lg font-semibold mb-2">How do I post an item?</h3>
					<p class="text-gray-600">
						Click the "Post Item" button, fill out the form with item details, photos, and pickup
						information. Your item will be live immediately!
					</p>
				</div>

				<div class="border-b pb-4">
					<h3 class="text-lg font-semibold mb-2">Can I edit my post after submitting?</h3>
					<p class="text-gray-600">
						Currently, you'll need to delete and repost your item to make changes. We're working on
						an edit feature!
					</p>
				</div>

				<div class="border-b pb-4">
					<h3 class="text-lg font-semibold mb-2">What items can I post?</h3>
					<p class="text-gray-600">
						Most items are allowed except weapons, illegal substances, live animals, and hazardous
						materials. Check our terms for the full list.
					</p>
				</div>

				<div class="border-b pb-4">
					<h3 class="text-lg font-semibold mb-2">How long do posts stay active?</h3>
					<p class="text-gray-600">
						Posts expire based on urgency: Urgent (24 hours), Soon (3 days), Flexible (7 days). You
						can mark items as picked up anytime.
					</p>
				</div>

				<div class="pb-4">
					<h3 class="text-lg font-semibold mb-2">Should I include my phone number?</h3>
					<p class="text-gray-600">
						You can, but we recommend using your Purdue email for initial contact. Only share your
						phone number with people you plan to meet.
					</p>
				</div>
			</div>
		{:else if activeCategory === 'safety'}
			<h2 class="text-2xl font-bold mb-6">🛡️ Safety & Security</h2>
			<div class="space-y-6">
				<div class="border-b pb-4">
					<h3 class="text-lg font-semibold mb-2">Is BoilerSwap safe?</h3>
					<p class="text-gray-600">
						BoilerSwap is designed for the Purdue community, but you should always follow safety
						guidelines when meeting strangers.
					</p>
				</div>

				<div class="border-b pb-4">
					<h3 class="text-lg font-semibold mb-2">Where should I meet for pickups?</h3>
					<p class="text-gray-600">
						Always meet in public, well-lit areas. Campus locations like residence hall lobbies or
						the Union are ideal.
					</p>
				</div>

				<div class="border-b pb-4">
					<h3 class="text-lg font-semibold mb-2">What if someone doesn't show up?</h3>
					<p class="text-gray-600">
						Contact us to report no-shows. Repeated offenders may have their accounts restricted.
						Always have backup plans.
					</p>
				</div>

				<div class="border-b pb-4">
					<h3 class="text-lg font-semibold mb-2">How do I report inappropriate behavior?</h3>
					<p class="text-gray-600">
						Use our "Report Issue" page to report any safety concerns, harassment, or inappropriate
						behavior immediately.
					</p>
				</div>

				<div class="pb-4">
					<h3 class="text-lg font-semibold mb-2">Can I bring a friend to pickups?</h3>
					<p class="text-gray-600">
						Bringing a friend is encouraged, especially for evening pickups or when meeting someone
						new.
					</p>
				</div>
			</div>
		{/if}
	</div>
</div>
