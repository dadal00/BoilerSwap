<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_BACKEND_URL } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
	import { type ItemType, type Condition, type Location } from '$lib/models'
	import ConditionField from './fields/ConditionField.svelte'
	import DescriptionField from './fields/DescriptionField.svelte'
	import ItemTypeField from './fields/ItemTypeField.svelte'
	import LocationField from './fields/LocationField.svelte'
	import TitleField from './fields/TitleField.svelte'
	import PostingFormButton from './PostingFormButton.svelte'

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

<div class="bg-white rounded-lg shadow-sm border p-6">
	<form onsubmit={submitItem}>
		<div class="space-y-6">
			<div>
				<label class="block text-sm font-medium mb-2">
					<ItemTypeField bind:item_type_value={item_type} />
				</label>
			</div>

			<div>
				<label class="block text-sm font-medium mb-2">
					<TitleField bind:title_value={title} />
				</label>
			</div>

			<div>
				<label class="block text-sm font-medium mb-2">
					<ConditionField bind:condition_value={condition} />
				</label>
			</div>

			<div>
				<label class="block text-sm font-medium mb-2">
					<DescriptionField bind:description_value={description} />
				</label>
			</div>

			<div>
				<label class="block text-sm font-medium mb-2">
					<LocationField bind:location_value={location} />
				</label>
			</div>

			<PostingFormButton></PostingFormButton>
		</div>
	</form>
</div>
