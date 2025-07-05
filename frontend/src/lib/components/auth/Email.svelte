<script lang="ts">
	import { PUBLIC_MAX_CHARS } from '$env/static/public'
	import type { Account } from '$lib/models'

	let { account = $bindable() } = $props<{ account: Account }>()
</script>

<div>
	<label class="block text-sm font-medium mb-2">
		Purdue Email Address
		<input
			type="email"
			placeholder="yourname@purdue.edu"
			class="w-full px-4 py-2 border rounded-lg"
			maxlength={Number(PUBLIC_MAX_CHARS)}
			bind:value={account.email}
		/>
	</label>
	<p class="text-xs text-gray-500 mt-1">Must be a valid @purdue.edu email address</p>
</div>
