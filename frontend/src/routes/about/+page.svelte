<script lang="ts">
	import Impact from '$lib/components/about/Impact.svelte'
	import Mission from '$lib/components/about/Mission.svelte'
	import Story from '$lib/components/about/Story.svelte'
	import Values from '$lib/components/about/Values.svelte'
</script>

<svelte:head>
	<title>About - BoilerSwap</title>
</svelte:head>

<div class="container mx-auto px-6 py-16 max-w-4xl">
	<div class="text-center mb-16">
		<h1 class="text-4xl font-bold text-gray-900 mb-4">About BoilerSwap</h1>
		<p class="text-xl text-gray-600">
			Building a sustainable community at Purdue, one swap at a time
		</p>
	</div>

	<Mission />
	<Story />
	<Values />
	<Impact />
</div>
