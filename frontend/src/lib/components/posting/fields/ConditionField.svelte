<script lang="ts">
	import { ConditionIterable, ConditionLabels, type Condition } from '$lib/models'

	let { conditionValue = $bindable() } = $props<{ conditionValue: Condition }>()
</script>

Condition *
<div class="space-y-2">
	{#each ConditionIterable as option}
		<label class="flex items-center">
			<input
				type="radio"
				bind:group={conditionValue}
				name="condition"
				value={option}
				required
				class="mr-2"
			/>
			{ConditionLabels[option]}
		</label>
	{/each}
</div>
