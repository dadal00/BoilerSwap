<script lang="ts">
	import { TabOptionsIterable, type TabOptions } from '$lib/models'
	import AuthHeader from '$lib/components/auth/AuthHeader.svelte'
	import TabButton from '$lib/components/auth/TabButton.svelte'
	import AuthFlow from '$lib/components/auth/AuthFlow.svelte'

	let activeTab: TabOptions = $state('Login')

	function showTab(tab: TabOptions) {
		activeTab = tab
	}
</script>

<svelte:head>
	<title>Login - BoilerSwap</title>
</svelte:head>

<AuthHeader />

<div class="container mx-auto px-6 py-16 max-w-md">
	<div class="text-center mb-8">
		<h1 class="text-3xl font-bold text-gray-900 mb-4">Welcome to BoilerSwap</h1>
		<p class="text-gray-600">Join the Purdue community of sustainable sharing</p>
	</div>

	<div class="bg-white rounded-lg shadow-sm border p-6">
		<div class="flex border-b mb-6">
			{#each TabOptionsIterable as tab}
				<TabButton bind:activeTabValue={activeTab} tabName={tab} {showTab} />
			{/each}
		</div>
		<AuthFlow bind:activeTabValue={activeTab} {showTab} />
	</div>
</div>
