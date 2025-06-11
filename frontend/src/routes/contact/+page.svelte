<script lang="ts">
	function submitContact(event: SubmitEvent) {
		event.preventDefault();
		alert('Message sent successfully! (This is a demo - no actual email was sent)');
	}
</script>

<svelte:head>
	<title>Contact Us - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-16 max-w-4xl">
	<div class="text-center mb-12">
		<h1 class="text-4xl font-bold text-gray-900 mb-4">Contact Us</h1>
		<p class="text-xl text-gray-600">
			We're here to help! Reach out with any questions or feedback.
		</p>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
		<div>
			<div class="bg-white rounded-lg shadow-sm border p-6">
				<h2 class="text-2xl font-bold mb-6 flex items-center">
					<span class="text-2xl mr-3">💬</span>
					Send us a Message
				</h2>
				<form onsubmit={submitContact}>
					<div class="space-y-6">
						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div>
								<label class="block text-sm font-medium mb-2">
									First Name *
									<input
										type="text"
										required
										placeholder="John"
										class="w-full px-4 py-2 border rounded-lg"
									/>
								</label>
							</div>
							<div>
								<label class="block text-sm font-medium mb-2">
									Last Name *
									<input
										type="text"
										required
										placeholder="Doe"
										class="w-full px-4 py-2 border rounded-lg"
									/>
								</label>
							</div>
						</div>

						<div>
							<label class="block text-sm font-medium mb-2">
								Email Address *
								<input
									type="email"
									required
									placeholder="john.doe@purdue.edu"
									class="w-full px-4 py-2 border rounded-lg"
								/>
							</label>
						</div>

						<div>
							<label class="block text-sm font-medium mb-2">
								Type of Inquiry *
								<select required class="w-full px-4 py-2 border rounded-lg">
									<option value="">Select inquiry type</option>
									<option value="general">General Question</option>
									<option value="technical">Technical Issue</option>
									<option value="bug">Bug Report</option>
									<option value="feature">Feature Request</option>
									<option value="safety">Safety Concern</option>
									<option value="partnership">Partnership Inquiry</option>
									<option value="feedback">Feedback</option>
								</select>
							</label>
						</div>

						<div>
							<label class="block text-sm font-medium mb-2">
								Subject *
								<input
									type="text"
									required
									placeholder="Brief description of your inquiry"
									class="w-full px-4 py-2 border rounded-lg"
								/>
							</label>
						</div>

						<div>
							<label class="block text-sm font-medium mb-2">
								Message *
								<textarea
									required
									placeholder="Please provide details about your inquiry..."
									class="w-full px-4 py-2 border rounded-lg h-32"
								></textarea>
							</label>
						</div>

						<button
							type="submit"
							class="w-full bg-yellow-400 text-gray-800 hover:bg-yellow-500 py-3 rounded-lg transition-colors"
						>
							📧 Send Message
						</button>

						<p class="text-sm text-gray-500 text-center">
							We typically respond within 24-48 hours during business days.
						</p>
					</div>
				</form>
			</div>
		</div>

		<div class="space-y-6">
			<div class="bg-white rounded-lg shadow-sm border p-6">
				<h3 class="text-xl font-bold mb-4">Quick Contact</h3>
				<div class="space-y-4">
					<div class="flex items-center space-x-3">
						<span class="text-2xl">📧</span>
						<div>
							<p class="font-medium">Email</p>
							<p class="text-gray-600">support@boilerswap.com</p>
						</div>
					</div>
					<div class="flex items-center space-x-3">
						<span class="text-2xl">⏰</span>
						<div>
							<p class="font-medium">Response Time</p>
							<p class="text-gray-600">24-48 hours</p>
						</div>
					</div>
				</div>
			</div>

			<div class="bg-white rounded-lg shadow-sm border p-6">
				<h3 class="text-xl font-bold mb-4">Quick Links</h3>
				<div class="space-y-3">
					<a
						href="help-center"
						class="flex items-center space-x-3 text-gray-600 hover:text-gray-900"
					>
						<span class="text-xl">🏠</span>
						<span>Help Center</span>
					</a>
					<a href="faq" class="flex items-center space-x-3 text-gray-600 hover:text-gray-900">
						<span class="text-xl">?</span>
						<span>Frequently Asked Questions</span>
					</a>
					<a
						href="safety-guidelines"
						class="flex items-center space-x-3 text-gray-600 hover:text-gray-900"
					>
						<span class="text-xl">🛡️</span>
						<span>Safety Guidelines</span>
					</a>
					<a
						href="report-issue"
						class="flex items-center space-x-3 text-gray-600 hover:text-gray-900"
					>
						<span class="text-xl">🚨</span>
						<span>Report an Issue</span>
					</a>
				</div>
			</div>

			<div class="bg-red-50 border border-red-200 rounded-lg p-6">
				<h3 class="text-xl font-bold text-red-800 mb-4">Safety Emergency</h3>
				<p class="text-red-700 text-sm mb-2">If you feel unsafe during a BoilerSwap exchange:</p>
				<ul class="text-red-700 text-sm space-y-1">
					<li>• Call Purdue Police: (765) 494-8221</li>
					<li>• Emergency: 911</li>
					<li>• Report to us immediately</li>
				</ul>
			</div>
		</div>
	</div>
</div>
