<script lang="ts">
	import { EmojiIterable, EmojiLabels, type Emoji } from '$lib/models'

	let { emojiValue = $bindable() } = $props<{ emojiValue: Emoji }>()
</script>

Emoji *
<select required class="w-full px-4 py-2 border rounded-lg" bind:value={emojiValue}>
	{#each EmojiIterable as category}
		<option value={category}>{EmojiLabels[category]}</option>
	{/each}
</select>
