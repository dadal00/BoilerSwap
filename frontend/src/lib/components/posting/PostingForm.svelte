<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_BACKEND_URL } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
	import { type Item } from '$lib/models'
	import { onDestroy, onMount } from 'svelte'
	import ConditionField from './fields/ConditionField.svelte'
	import DescriptionField from './fields/DescriptionField.svelte'
	import EmojiField from './fields/EmojiField.svelte'
	import ItemTypeField from './fields/ItemTypeField.svelte'
	import LocationField from './fields/LocationField.svelte'
	import TitleField from './fields/TitleField.svelte'
	import PostingFormButton from './PostingFormButton.svelte'

	let item: Item = $state({
		item_type: 'Furniture',
		condition: 'Fair',
		title: '',
		description: '',
		location: 'CaryQuadEast',
		emoji: 'Books'
	})

	let error: string = $derived(appState.getPostError())

	async function submitItem(event: SubmitEvent) {
		appState.setPostError('')
		event.preventDefault()

		if (appState.getProductLimited()) {
			return
		}

		appState.nowProductLimited()
		const response = await fetch(PUBLIC_BACKEND_URL + '/post-item', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			credentials: 'include',
			body: JSON.stringify(item)
		})

		if (!response.ok) {
			appState.setPostError((await response.text()).slice(0, 50))
			throw new Error(`HTTP error! status: ${response.status}`)
		}

		alert('Item posted successfully!')
		goto('/browse')
	}

	onMount(() => {
		appState.setPostError('')
	})

	onDestroy(() => {
		appState.setPostError('')
	})
</script>

<div class="bg-white rounded-lg shadow-sm border p-6">
	<form onsubmit={submitItem}>
		<div class="space-y-6">
			{#if error != ''}
				<p class="text-red-600 text-sm font-medium text-center">{error}</p>
			{/if}
			<div>
				<label class="block text-sm font-medium mb-2">
					<ItemTypeField bind:itemTypeValue={item.item_type} />
				</label>
			</div>

			<div>
				<label class="block text-sm font-medium mb-2">
					<TitleField bind:titleValue={item.title} />
				</label>
			</div>

			<div>
				<label class="block text-sm font-medium mb-2">
					<ConditionField bind:conditionValue={item.condition} />
				</label>
			</div>

			<div>
				<label class="block text-sm font-medium mb-2">
					<DescriptionField bind:descriptionValue={item.description} />
				</label>
			</div>

			<div>
				<label class="block text-sm font-medium mb-2">
					<LocationField bind:locationValue={item.location} />
				</label>
			</div>

			<div>
				<label class="block text-sm font-medium mb-2">
					<EmojiField bind:emojiValue={item.emoji} />
				</label>
			</div>

			<PostingFormButton></PostingFormButton>
		</div>
	</form>
</div>
