<script lang="ts">
	import PostingForm from '$lib/components/posting/PostingForm.svelte'
	import PostingHeader from '$lib/components/posting/PostingHeader.svelte'
	import PostingTips from '$lib/components/posting/PostingTips.svelte'
</script>

<svelte:head>
	<title>Post Item - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-8 max-w-2xl">
	<PostingHeader />
	<PostingForm />
	<PostingTips />
</div>
