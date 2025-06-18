<script lang="ts">
	import { PUBLIC_BACKEND_URL } from '$env/static/public'
	import type { Account } from '$lib/models'

	let activeTab = $state('login')
	let authenticationID: string = ''
	let account: Account = { email: '', password: '', action: 'signup' }
	let confirmPassword: string = $state('')

	function showTab(tab: string) {
		activeTab = tab
	}

	async function login() {
		account.action = 'login'

		if (!/.+@purdue\.edu$/.test(account.email)) {
			console.log('Signup failed: email must be a Purdue address')
			return
		}
		if (account.password === '') {
			console.log('Signup failed: invalid password')
			return
		}

		try {
			const response = await fetch(PUBLIC_BACKEND_URL + '/authenticate', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(account)
			})

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`)
			}
		} catch (err) {
			console.log('Signup failed: ', err)
		}
	}

	async function signup() {
		account.action = 'signup'

		if (!/.+@purdue\.edu$/.test(account.email)) {
			console.log('Signup failed: email must be a Purdue address')
			return
		}
		if (account.password === '') {
			console.log('Signup failed: invalid password')
			return
		}
		if (account.password !== confirmPassword) {
			console.log('Signup failed: passwords do not match')
			return
		}

		try {
			const response = await fetch(PUBLIC_BACKEND_URL + '/authenticate', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(account)
			})

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`)
			}
		} catch (err) {
			console.log('Signup failed: ', err)
		}
	}
</script>

<svelte:head>
	<title>Login - BoilerSwap</title>
</svelte:head>

<header class="border-b bg-white sticky top-0 z-50">
	<div class="container mx-auto px-4 py-4">
		<div class="flex items-center justify-between">
			<a href="/" class="flex items-center space-x-2">
				<div
					class="w-8 h-8 bg-gradient-to-br from-yellow-400 to-amber-500 rounded-lg flex items-center justify-center"
				>
					<span class="text-white font-bold text-sm">BS</span>
				</div>
				<h1 class="text-2xl font-bold text-gray-900">BoilerSwap</h1>
			</a>

			<a href="/" class="border border-gray-300 text-gray-700 hover:bg-gray-50 px-4 py-2 rounded">
				← Back to Home
			</a>
		</div>
	</div>
</header>

<div class="container mx-auto px-6 py-16 max-w-md">
	<div class="text-center mb-8">
		<h1 class="text-3xl font-bold text-gray-900 mb-4">Welcome to BoilerSwap</h1>
		<p class="text-gray-600">Join the Purdue community of sustainable sharing</p>
	</div>

	<div class="bg-white rounded-lg shadow-sm border p-6">
		<div class="flex border-b mb-6">
			<button
				onclick={() => showTab('login')}
				id="login-tab"
				class="flex-1 py-2 text-center {activeTab === 'login'
					? 'border-yellow-400 text-yellow-600 border-b-2'
					: 'text-gray-500'}">Login</button
			>
			<button
				onclick={() => showTab('signup')}
				id="signup-tab"
				class="flex-1 py-2 text-center {activeTab === 'signup'
					? 'border-yellow-400 text-yellow-600 border-b-2'
					: 'text-gray-500'}">Sign Up</button
			>
			<button
				onclick={() => showTab('forgot')}
				id="forgot-tab"
				class="flex-1 py-2 text-center {activeTab === 'forgot'
					? 'border-yellow-400 text-yellow-600 border-b-2'
					: 'text-gray-500'}">Reset</button
			>
		</div>
		{#if activeTab === 'login'}
			<div class="space-y-4">
				<div>
					<label class="block text-sm font-medium mb-2">
						Purdue Email Address
						<input
							type="email"
							placeholder="yourname@purdue.edu"
							class="w-full px-4 py-2 border rounded-lg"
							bind:value={account.email}
						/>
					</label>
				</div>
				<div>
					<label class="block text-sm font-medium mb-2">
						Password
						<input
							type="password"
							class="w-full px-4 py-2 border rounded-lg"
							bind:value={account.password}
						/>
					</label>
				</div>
				<button
					class="w-full bg-yellow-400 text-gray-800 hover:bg-yellow-500 py-2 rounded-lg transition-colors"
					onclick={login}
				>
					Login
				</button>
				<div class="text-center">
					<button onclick={() => showTab('forgot')} class="text-yellow-600 hover:underline text-sm">
						Forgot your password?
					</button>
				</div>
			</div>
		{:else if activeTab === 'signup'}
			<div class="space-y-4">
				<div>
					<label class="block text-sm font-medium mb-2">
						Full Name
						<input type="text" placeholder="John Doe" class="w-full px-4 py-2 border rounded-lg" />
					</label>
				</div>
				<div>
					<label class="block text-sm font-medium mb-2">
						Purdue Email Address
						<input
							type="email"
							placeholder="yourname@purdue.edu"
							class="w-full px-4 py-2 border rounded-lg"
							maxlength="64"
							bind:value={account.email}
						/>
					</label>
					<p class="text-xs text-gray-500 mt-1">Must be a valid @purdue.edu email address</p>
				</div>
				<div>
					<label class="block text-sm font-medium mb-2">
						Password
						<input
							type="password"
							class="w-full px-4 py-2 border rounded-lg"
							maxlength="64"
							required
							bind:value={account.password}
						/>
					</label>
				</div>
				<div>
					<label class="block text-sm font-medium mb-2">
						Confirm Password
						<input
							type="password"
							class="w-full px-4 py-2 border rounded-lg"
							maxlength="64"
							bind:value={confirmPassword}
						/>
					</label>
				</div>
				<button
					class="w-full bg-yellow-400 text-gray-800 hover:bg-yellow-500 py-2 rounded-lg transition-colors"
					onclick={signup}
				>
					Create Account
				</button>
				<p class="text-xs text-gray-500 text-center">
					By creating an account, you agree to our <a
						href="disclaimer"
						class="text-yellow-600 hover:underline">Terms of Service</a
					>
				</p>
			</div>
		{:else if activeTab === 'forgot'}
			<div class="space-y-4">
				<p class="text-gray-600 text-sm">
					Enter your Purdue email address and we'll send you a link to reset your password.
				</p>
				<div>
					<label class="block text-sm font-medium mb-2">
						Purdue Email Address
						<input
							type="email"
							placeholder="yourname@purdue.edu"
							class="w-full px-4 py-2 border rounded-lg"
						/>
					</label>
				</div>
				<button
					class="w-full bg-yellow-400 text-gray-800 hover:bg-yellow-500 py-2 rounded-lg transition-colors"
					>Send Reset Link</button
				>
				<div class="text-center">
					<button onclick={() => showTab('login')} class="text-yellow-600 hover:underline text-sm">
						Back to login
					</button>
				</div>
			</div>
		{/if}
	</div>
</div>
