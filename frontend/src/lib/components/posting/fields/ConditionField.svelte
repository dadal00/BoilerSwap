<script lang="ts">
	import { ConditionIterable, ConditionLabels, type Condition } from '$lib/models'

	let { condition_value = $bindable() } = $props<{ condition_value: Condition }>()
</script>

Condition *
<div class="space-y-2">
	{#each ConditionIterable as option}
		<label class="flex items-center">
			<input
				type="radio"
				bind:group={condition_value}
				name="condition"
				value={option}
				required
				class="mr-2"
			/>
			{ConditionLabels[option]}
		</label>
	{/each}
</div>
