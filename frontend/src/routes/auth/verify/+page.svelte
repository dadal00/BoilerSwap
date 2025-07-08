<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_TEMP_SESSION_DURATION_SECS } from '$env/static/public'
	import { appState } from '$lib/AppState.svelte'
	import { Status } from '$lib/models'
	import { onDestroy, onMount } from 'svelte'
	import VerifyCode from '$lib/components/auth/verify/VerifyCode.svelte'

	let auth_code: string = $state('')
	let timer: number | null = $state(null)

	onMount(() => {
		if (!appState.getStatus(Status.isVerifying)) {
			goto('/browse')
		}
		timer = setTimeout(
			() => {
				appState.setStatus(Status.isVerifying, false)
			},
			Number(PUBLIC_TEMP_SESSION_DURATION_SECS) * 1000
		)
	})

	$effect(() => {
		if (!appState.getStatus(Status.isVerifying)) {
			goto('/browse')
		}
	})

	onDestroy(() => {
		appState.setStatus(Status.isVerifying, false)
		clearTimeout(timer!)
	})
</script>

<VerifyCode bind:auth_code verification_type="verify" bind:timer />
