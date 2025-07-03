<script lang="ts">
	import {
		ConditionEmojis,
		ConditionLabels,
		EmojiLabels,
		LocationLabels,
		type Condition,
		type Emoji,
		type ExpirationColor,
		type ItemType,
		type Location
	} from '$lib/models'

	const { emoji, title, category, description, location, expiration, condition, expirationColor } =
		$props<{
			emoji: Emoji
			title: string
			category: ItemType
			description: string
			location: Location
			expiration: string
			condition: Condition
			expirationColor: ExpirationColor
		}>()
</script>

<div
	class="bg-white rounded-lg shadow-sm border-l-4 border-l-{expirationColor}-500 overflow-hidden card-hover"
>
	<div class="h-48 bg-gray-200 flex items-center justify-center">
		<div class="text-6xl">{EmojiLabels[emoji as Emoji]}</div>
	</div>
	<div class="p-6">
		<div class="flex justify-between items-start mb-2">
			<h3 class="font-semibold text-lg">{title}</h3>
			<span class="bg-yellow-100 text-yellow-800 px-2 py-1 rounded text-sm">{category}</span>
		</div>
		<p class="text-gray-600 text-sm mb-4">
			{description}
		</p>
		<div class="flex gap-x-3 items-start mb-2">
			<div class="flex items-center text-sm text-gray-500 mb-2">
				<span class="mr-1">📍</span>
				{LocationLabels[location as Location]}
			</div>
			<div class="flex items-center text-sm text-gray-500 mb-2">
				<span class="mr-1">{ConditionEmojis[condition as Condition]}</span>
				{condition}
			</div>
		</div>
		<div class="flex items-center text-sm text-{expirationColor}-600 font-medium">
			<span class="mr-1">⏰</span>
			Expires in {expiration}
		</div>
	</div>
</div>
