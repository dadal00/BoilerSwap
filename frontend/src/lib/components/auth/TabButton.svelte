<script lang="ts">
	import type { TabOptions } from '$lib/models'

	let {
		activeTabValue = $bindable(),
		tabName,
		showTab
	} = $props<{
		activeTabValue: string
		tabName: TabOptions
		showTab: (tab: TabOptions) => void
	}>()
</script>

<button
	onclick={() => showTab(tabName)}
	class="flex-1 py-2 text-center {activeTabValue === tabName
		? 'border-yellow-400 text-yellow-600 border-b-2'
		: 'text-gray-500'}">{tabName}</button
>
