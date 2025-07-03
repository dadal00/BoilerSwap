import { goto } from '$app/navigation'
import { PUBLIC_BACKEND_URL } from '$env/static/public'
import { Status, type Account } from '$lib/models'
import { appState } from '$lib/AppState.svelte'
import { fetchBackend } from './utils'

export async function forgot(email: string): Promise<void> {
	if (appState.isLimited()) {
		return
	}

	if (!/.+@purdue\.edu$/.test(email)) {
		console.log('Recovery failed: email must be a Purdue address')
		return
	}

	try {
		appState.setLastAttempt(Date.now())

		await fetchBackend('/forgot', { token: email })

		appState.setStatus(Status.isVerifyingForgot, true)
		goto('/auth/verify/forget')
	} catch (err) {
		console.log('Login failed: ', err)
	}
}

export async function login(account: Account): Promise<void> {
	if (appState.isLimited()) {
		return
	}

	account.action = 'login'

	if (!/.+@purdue\.edu$/.test(account.email)) {
		console.log('Signup failed: email must be a Purdue address')
		return
	}
	if (account.password === '') {
		console.log('Signup failed: invalid password')
		return
	}

	try {
		appState.setLastAttempt(Date.now())

		await fetchBackend('/authenticate', account)

		appState.setStatus(Status.isVerifying, true)
		goto('/auth/verify')
	} catch (err) {
		console.log('Login failed: ', err)
	}
}

export async function signup(account: Account, confirmPassword: string): Promise<void> {
	if (appState.isLimited()) {
		return
	}

	account.action = 'signup'

	if (!/.+@purdue\.edu$/.test(account.email)) {
		console.log('Signup failed: email must be a Purdue address')
		return
	}
	if (account.password === '') {
		console.log('Signup failed: invalid password')
		return
	}
	if (account.password !== confirmPassword) {
		console.log('Signup failed: passwords do not match')
		return
	}

	try {
		appState.setLastAttempt(Date.now())

		await fetchBackend('/authenticate', account)

		appState.setStatus(Status.isVerifying, true)
		goto('/auth/verify')
	} catch (err) {
		console.log('Signup failed: ', err)
	}
}

export async function verify(auth_code: string): Promise<void> {
	if (appState.isLimited()) {
		return
	}

	if (!appState.getStatus(Status.isVerifying)) {
		return
	}

	if (!/^\d+$/.test(auth_code) || auth_code.length != 6) {
		console.log('Verification failed: only 6 numbers')
		return
	}

	try {
		appState.setLastAttempt(Date.now())

		await fetchBackend('/verify', { token: auth_code })

		appState.setStatus(Status.isSignedIn, true)
		goto('/browse')
	} catch (err) {
		console.log('verification failed: ', err)
	}
}

export async function verify_forget(auth_code: string) {
	if (appState.isLimited()) {
		return
	}

	if (!appState.getStatus(Status.isVerifyingForgot)) {
		return
	}

	if (!/^\d+$/.test(auth_code) || auth_code.length != 6) {
		console.log('Verification failed: only 6 numbers')
		return
	}

	try {
		appState.setLastAttempt(Date.now())

		await fetchBackend('/verify', { token: auth_code })

		appState.setStatus(Status.isVerifyingUpdate, true)
		goto('/auth/verify/update')
	} catch (err) {
		console.log('verification failed: ', err)
	}
}

export async function update(new_password: string) {
	if (appState.isLimited()) {
		return
	}

	if (!appState.getStatus(Status.isVerifyingUpdate)) {
		return
	}

	if (new_password === '' || new_password.length > 100) {
		console.log('Invalid password')
		return
	}

	try {
		appState.setLastAttempt(Date.now())

		await fetchBackend('/verify', { token: new_password })

		appState.setStatus(Status.isSignedIn, true)
		goto('/browse')
	} catch (err) {
		console.log('verification failed: ', err)
	}
}

export async function signout() {
	if (appState.isLimited()) {
		return
	}

	if (appState.getStatus(Status.isSignedIn)) {
		appState.setLastAttempt(Date.now())

		const response = await fetch(PUBLIC_BACKEND_URL + '/delete', {
			method: 'DELETE',
			credentials: 'include'
		})

		if (!response.ok) {
			throw new Error(`HTTP error! status: ${response.status}`)
		}

		appState.setStatus(Status.isSignedIn, false)
	}
}
