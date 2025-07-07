<script lang="ts">
	import { appState } from '$lib/AppState.svelte'
</script>

<button
	type="submit"
	class="w-full {appState.getProductLimited()
		? 'bg-gray-300 cursor-not-allowed'
		: 'bg-gradient-to-r from-yellow-400 to-amber-500 hover:from-yellow-500 hover:to-amber-600'} text-white py-3 text-lg font-medium rounded-lg transition-colors"
>
	Post My Free Item
</button>
