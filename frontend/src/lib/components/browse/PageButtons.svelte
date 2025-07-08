<script lang="ts">
	import { PUBLIC_PAGE_SIZE } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
</script>

<div class="flex justify-center gap-4 mt-8">
	<button
		class="{appState.getOffset() - Number(PUBLIC_PAGE_SIZE) < 0
			? 'bg-gray-300 cursor-not-allowed'
			: 'bg-yellow-400 hover:bg-yellow-500'} text-gray-800 py-2 px-4 rounded-lg transition-colors flex items-center justify-center"
		onclick={() => {
			if (appState.getLimited() || appState.getOffset() - Number(PUBLIC_PAGE_SIZE) < 0) {
				return
			}
			appState.nowLimited()
			appState.decrementOffset()
			window.scrollTo({ top: 0, behavior: 'auto' })
		}}
	>
		← Previous
	</button>

	<button
		class="{appState.getOffset() + Number(PUBLIC_PAGE_SIZE) > appState.getTotalHits()
			? 'bg-gray-300 cursor-not-allowed'
			: 'bg-yellow-400 hover:bg-yellow-500'} text-gray-800 py-2 px-4 rounded-lg transition-colors flex items-center justify-center"
		onclick={() => {
			if (
				appState.getLimited() ||
				appState.getOffset() + Number(PUBLIC_PAGE_SIZE) > appState.getTotalHits()
			) {
				return
			}
			appState.nowLimited()
			appState.incrementOffset()
			window.scrollTo({ top: 0, behavior: 'auto' })
		}}
	>
		Next →
	</button>
</div>
