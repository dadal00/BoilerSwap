<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_TEMP_SESSION_DURATION_SECS } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
	import { Status } from '$lib/models'
	import { onDestroy, onMount } from 'svelte'
	import VerifyCode from '$lib/components/auth/verify/VerifyCode.svelte'

	let auth_code: string = $state('')
	let timer: number | null = null

	onMount(() => {
		if (!appState.getStatus(Status.isVerifyingForgot)) {
			goto('/browse')
		}

		timer = setTimeout(
			() => {
				appState.setStatus(Status.isVerifyingForgot, false)
			},
			Number(PUBLIC_TEMP_SESSION_DURATION_SECS) * 1000
		)
	})

	$effect(() => {
		if (!appState.getStatus(Status.isVerifyingForgot)) {
			goto('/browse')
		}
	})

	onDestroy(() => {
		appState.setStatus(Status.isVerifyingForgot, false)
		clearTimeout(timer!)
	})
</script>

<VerifyCode bind:auth_code verification_type="forget" />
