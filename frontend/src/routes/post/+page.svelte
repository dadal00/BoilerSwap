<script lang="ts">
	import PostingForm from '$lib/components/posting/PostingForm.svelte'
	import PostingTips from '$lib/components/posting/PostingTips.svelte'
</script>

<svelte:head>
	<title>Post Item - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-8 max-w-2xl">
	<div class="text-center mb-8">
		<h1 class="text-3xl font-bold text-gray-900 mb-4">Post a New Item</h1>
		<p class="text-gray-600">Help a fellow Boilermaker and keep usable items out of the trash!</p>
	</div>
	<PostingForm />
	<PostingTips />
</div>
