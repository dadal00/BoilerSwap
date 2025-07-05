<script lang="ts">
	import {
		ConditionEmojis,
		EmojiLabels,
		ItemFields,
		LocationLabels,
		type Condition,
		type Emoji,
		type Item,
		type Location
	} from '$lib/models'

	const { item } = $props<{
		item: Item
	}>()
</script>

<div
	class="bg-white rounded-lg shadow-sm border-l-4 border-l-{item.expirationColor}-500 overflow-hidden card-hover"
>
	<div class="h-48 bg-gray-200 flex items-center justify-center">
		<div class="text-6xl">{EmojiLabels[item[ItemFields.EMOJI] as Emoji]}</div>
	</div>
	<div class="p-6">
		<div class="flex justify-between items-start mb-2">
			<h3 class="font-semibold text-lg">{item[ItemFields.TITLE]}</h3>
			<span class="bg-yellow-100 text-yellow-800 px-2 py-1 rounded text-sm"
				>{item[ItemFields.ITEM_TYPE]}</span
			>
		</div>
		<p class="text-gray-600 text-sm mb-4">
			{item.description}
		</p>
		<div class="flex gap-x-3 items-start mb-2">
			<div class="flex items-center text-sm text-gray-500 mb-2">
				<span class="mr-1">📍</span>
				{LocationLabels[item[ItemFields.LOCATION] as Location]}
			</div>
			<div class="flex items-center text-sm text-gray-500 mb-2">
				<span class="mr-1">{ConditionEmojis[item[ItemFields.CONDITION] as Condition]}</span>
				{item.condition}
			</div>
		</div>
		<div class="flex items-center text-sm text-{item.expirationColor}-600 font-medium">
			<span class="mr-1">⏰</span>
			Expires in {item.expiration}
		</div>
	</div>
</div>
