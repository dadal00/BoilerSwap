<script lang="ts">
	import { goto } from '$app/navigation'

	function submitReport(event: SubmitEvent) {
		event.preventDefault()
		alert(
			'Report submitted successfully. Our team will review it and take appropriate action. Thank you for helping keep BoilerSwap safe.'
		)
		goto('/')
	}
</script>

<svelte:head>
	<title>Report Issue - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-16 max-w-2xl">
	<div class="text-center mb-8">
		<h1 class="text-4xl font-bold text-gray-900 mb-4">🚨 Report an Issue</h1>
		<p class="text-xl text-gray-600">Help us keep BoilerSwap safe and welcoming for everyone</p>
	</div>

	<div class="bg-red-50 border border-red-200 rounded-lg p-6 mb-8">
		<h2 class="text-xl font-bold text-red-800 mb-4">🚨 Emergency Situations</h2>
		<p class="text-red-700 mb-4">If you are in immediate danger or experiencing an emergency:</p>
		<div class="space-y-2 text-red-700 font-medium">
			<p>• Call 911 for emergencies</p>
			<p>• Call Purdue Police: (765) 494-8221</p>
			<p>• Use Purdue's SafeWalk: (765) 494-7233</p>
		</div>
	</div>

	<div class="bg-white rounded-lg shadow-sm border p-6">
		<h2 class="text-2xl font-bold mb-6">Report Details</h2>
		<form onsubmit={submitReport}>
			<div class="space-y-6">
				<div>
					<label class="block text-sm font-medium mb-2">
						Type of Issue *
						<select required class="w-full px-4 py-2 border rounded-lg">
							<option value="">Select issue type</option>
							<option value="safety">Safety Concern</option>
							<option value="harassment">Harassment or Inappropriate Behavior</option>
							<option value="scam">Suspected Scam or Fraud</option>
							<option value="no-show">User Didn't Show Up</option>
							<option value="fake-listing">Fake or Misleading Listing</option>
							<option value="spam">Spam or Inappropriate Content</option>
							<option value="technical">Technical Problem</option>
							<option value="other">Other</option>
						</select>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Your Email *
						<input
							type="email"
							required
							placeholder="your.email@purdue.edu"
							class="w-full px-4 py-2 border rounded-lg"
						/>
					</label>
					<p class="text-sm text-gray-500 mt-1">We may need to contact you for follow-up</p>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						User or Item Involved
						<input
							type="text"
							placeholder="Username, email, or item title (if applicable)"
							class="w-full px-4 py-2 border rounded-lg"
						/>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						When did this occur?
						<input type="datetime-local" class="w-full px-4 py-2 border rounded-lg" />
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Location (if applicable)
						<input
							type="text"
							placeholder="Where did this incident occur?"
							class="w-full px-4 py-2 border rounded-lg"
						/>
					</label>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">
						Detailed Description *
						<textarea
							required
							placeholder="Please provide as much detail as possible about what happened..."
							class="w-full px-4 py-2 border rounded-lg h-32"
						></textarea>
					</label>
					<p class="text-sm text-gray-500 mt-1">
						Include any relevant screenshots, messages, or evidence
					</p>
				</div>

				<!-- <div>
					<label class="block text-sm font-medium mb-2">
						Evidence/Screenshots
						<div class="border-2 border-dashed border-gray-300 rounded-lg p-6 text-center">
							<div class="text-4xl mb-2">📎</div>
							<p class="text-gray-600">Upload any relevant files or screenshots</p>
							<p class="text-sm text-gray-400 mt-1">PNG, JPG, PDF up to 10MB</p>
						</div>
					</label>
				</div> -->

				<div class="bg-gray-50 rounded-lg p-4">
					<h3 class="font-semibold mb-2">Privacy Notice</h3>
					<p class="text-sm text-gray-600">
						Your report will be reviewed by our moderation team. We take all reports seriously and
						will investigate appropriately. Personal information will only be used for investigation
						purposes and will not be shared unnecessarily.
					</p>
				</div>

				<button
					type="submit"
					class="w-full bg-red-600 text-white hover:bg-red-700 py-3 rounded-lg transition-colors"
				>
					🚨 Submit Report
				</button>
			</div>
		</form>
	</div>

	<div class="mt-8 bg-blue-50 border border-blue-200 rounded-lg p-6">
		<h3 class="text-xl font-bold mb-4">Additional Resources</h3>
		<div class="space-y-3">
			<a
				href="safety-guidelines"
				class="flex items-center space-x-3 text-blue-700 hover:text-blue-900"
			>
				<span class="text-xl">🛡️</span>
				<span>Safety Guidelines</span>
			</a>
			<a
				href="https://www.purdue.edu/advocacy/"
				class="flex items-center space-x-3 text-blue-700 hover:text-blue-900"
				target="_blank"
			>
				<span class="text-xl">🏫</span>
				<span>Purdue Advocacy & Support</span>
			</a>
			<a
				href="https://www.purdue.edu/caps/"
				class="flex items-center space-x-3 text-blue-700 hover:text-blue-900"
				target="_blank"
			>
				<span class="text-xl">💚</span>
				<span>Counseling & Psychological Services</span>
			</a>
			<a href="contact" class="flex items-center space-x-3 text-blue-700 hover:text-blue-900">
				<span class="text-xl">📧</span>
				<span>Contact BoilerSwap Support</span>
			</a>
		</div>
	</div>
</div>
