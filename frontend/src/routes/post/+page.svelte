<script lang="ts">
	import { goto } from '$app/navigation'

	function submitItem(event: SubmitEvent) {
		event.preventDefault()
		alert('Item posted successfully! (This is a demo - no actual posting occurred)')
		goto('/')
	}
</script>

<svelte:head>
	<title>Post Item - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-8 max-w-2xl">
	<div class="text-center mb-8">
		<h1 class="text-3xl font-bold text-gray-900 mb-4">Post a New Item</h1>
		<p class="text-gray-600">Help a fellow Boilermaker and keep usable items out of the trash!</p>
	</div>

	<div class="bg-white rounded-lg shadow-sm border p-6">
		<form onsubmit={submitItem}>
			<div class="space-y-6">
				<div>
					<label class="block text-sm font-medium mb-2">
						Item Type *
						<select required class="w-full px-4 py-2 border rounded-lg">
							<option value="">Select item type</option>
							<option value="furniture-chair">Chair</option>
							<option value="furniture-desk">Desk</option>
							<option value="electronics-fridge">Mini Fridge</option>
							<option value="electronics-microwave">Microwave</option>
							<option value="books-textbook">Textbook</option>
							<option value="kitchen-cookware">Cookware</option>
							<option value="other">Other</option>
						</select>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Title *
						<input
							type="text"
							required
							placeholder="e.g., IKEA Desk Chair - Great Condition"
							class="w-full px-4 py-2 border rounded-lg"
						/>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Condition *
						<div class="space-y-2">
							<label class="flex items-center">
								<input type="radio" name="condition" value="excellent" required class="mr-2" />
								Excellent - Like new, minimal wear
							</label>
							<label class="flex items-center">
								<input type="radio" name="condition" value="good" class="mr-2" />
								Good - Some wear but fully functional
							</label>
							<label class="flex items-center">
								<input type="radio" name="condition" value="fair" class="mr-2" />
								Fair - Noticeable wear but still usable
							</label>
						</div>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Description
						<textarea
							placeholder="Add any additional details, flaws, or special instructions..."
							class="w-full px-4 py-2 border rounded-lg h-24"
						></textarea>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Pickup Location *
						<select required class="w-full px-4 py-2 border rounded-lg">
							<option value="">Select pickup location</option>
							<option value="cary-east">Cary Quad - East</option>
							<option value="wiley">Wiley Hall</option>
							<option value="harrison">Harrison Hall</option>
							<option value="earhart">Earhart Hall</option>
							<option value="hillenbrand">Hillenbrand Hall</option>
							<option value="third-street">Third Street Suites</option>
						</select>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Urgency Level *
						<div class="space-y-2">
							<label class="flex items-center">
								<input type="radio" name="urgency" value="low" required class="mr-2" />
								<div>
									<div class="font-medium">Flexible (3+ days)</div>
									<div class="text-sm text-gray-500">No rush, available for a week or more</div>
								</div>
							</label>
							<label class="flex items-center">
								<input type="radio" name="urgency" value="medium" class="mr-2" />
								<div>
									<div class="font-medium">Soon (1-2 days)</div>
									<div class="text-sm text-gray-500">
										Would like it gone in the next couple days
									</div>
								</div>
							</label>
							<label class="flex items-center">
								<input type="radio" name="urgency" value="high" class="mr-2" />
								<div>
									<div class="font-medium text-red-600">URGENT (&lt; 24 hours)</div>
									<div class="text-sm text-gray-500">
										Moving out today/tomorrow, needs to go ASAP!
									</div>
								</div>
							</label>
						</div>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Contact Information *
						<input
							type="email"
							required
							placeholder="Your Purdue email"
							class="w-full px-4 py-2 border rounded-lg"
						/>
					</label>
				</div>

				<button
					type="submit"
					class="w-full bg-gradient-to-r from-yellow-400 to-amber-500 hover:from-yellow-500 hover:to-amber-600 text-white py-3 text-lg font-medium rounded-lg transition-colors"
				>
					Post My Free Item
				</button>
			</div>
		</form>
	</div>

	<div class="mt-8 bg-yellow-50 border border-yellow-200 rounded-lg p-6">
		<h3 class="text-lg font-semibold mb-4">💡 Tips for a Great Post</h3>
		<ul class="space-y-2 text-sm">
			<li>• Take clear, well-lit photos from multiple angles</li>
			<li>• Be honest about any flaws or damage</li>
			<li>• Include dimensions for furniture items</li>
			<li>• Respond quickly to interested students</li>
			<li>• Be flexible with pickup times when possible</li>
		</ul>
	</div>
</div>
