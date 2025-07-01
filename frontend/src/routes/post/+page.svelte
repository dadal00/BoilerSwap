<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_BACKEND_URL, PUBLIC_MAX_CHARS } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
	import { type ItemType, type Condition, type Location } from '$lib/models'

	let item_type: ItemType = $state('Furniture')
	let condition: Condition = $state('Fair')
	let title: string = $state('')
	let description: string = $state('')
	let location: Location = $state('CaryQuadEast')

	async function submitItem(event: SubmitEvent) {
		event.preventDefault()

		if (appState.isProductLimited()) {
			return
		}

		appState.setLastAttempt(Date.now())
		const response = await fetch(PUBLIC_BACKEND_URL + '/post-item', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			credentials: 'include',
			body: JSON.stringify({ item_type, condition, title, description, location })
		})

		if (!response.ok) {
			throw new Error(`HTTP error! status: ${response.status}`)
		}

		alert('Item posted successfully!')
		goto('/browse')
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
						<select required class="w-full px-4 py-2 border rounded-lg" bind:value={item_type}>
							<option value="" disabled selected hidden>Select item type</option>
							<option value="Furniture">Furniture</option>
							<option value="Electronics">Electronics</option>
							<option value="Books">Books</option>
							<option value="Kitchen">Kitchen</option>
							<option value="Clothing">Clothing</option>
							<option value="Other">Other</option>
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
							bind:value={title}
							maxlength={Number(PUBLIC_MAX_CHARS)}
						/>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Condition *
						<div class="space-y-2">
							<label class="flex items-center">
								<input
									type="radio"
									bind:group={condition}
									name="condition"
									value="Excellent"
									required
									class="mr-2"
								/>
								Excellent - Like new, minimal wear
							</label>
							<label class="flex items-center">
								<input
									type="radio"
									bind:group={condition}
									name="condition"
									value="Good"
									class="mr-2"
								/>
								Good - Some wear but fully functional
							</label>
							<label class="flex items-center">
								<input
									type="radio"
									bind:group={condition}
									name="condition"
									value="Fair"
									class="mr-2"
								/>
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
							bind:value={description}
							maxlength={Number(PUBLIC_MAX_CHARS)}
						></textarea>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Pickup Location *
						<select required class="w-full px-4 py-2 border rounded-lg" bind:value={location}>
							<option value="" disabled selected hidden>Select pickup location</option>
							<option value="CaryQuadEast">Cary Quad - East</option>
							<option value="WileyHall">Wiley Hall</option>
							<option value="HarrisonHall">Harrison Hall</option>
							<option value="EarhartHall">Earhart Hall</option>
							<option value="HillenbrandHall">Hillenbrand Hall</option>
							<option value="ThirdStreetSuites">Third Street Suites</option>
						</select>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Urgency Level *
						<div class="space-y-2">
							<label class="flex items-center">
								<input type="radio" name="urgency" value="low" class="mr-2" />
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
