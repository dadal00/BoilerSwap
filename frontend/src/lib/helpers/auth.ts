import { goto } from '$app/navigation'
import {
	PUBLIC_BACKEND_URL,
	PUBLIC_CODE_LENGTH,
	PUBLIC_MIN_PASSWORD_LENGTH
} from '$env/static/public'
import { Status, type Account } from '$lib/models'
import { appState } from '$lib/AppState.svelte'
import { fetchBackend } from './utils'

export async function forgot(email: string): Promise<void> {
	if (appState.getLimited()) {
		return
	}

	if (!/.+@purdue\.edu$/.test(email)) {
		appState.setAuthError('Email must be a Purdue address')
		return
	}

	try {
		appState.nowLimited()

		await fetchBackend('/forgot', { token: email })

		appState.setStatus(Status.isVerifyingForgot, true)
		goto('/auth/verify/forget')
	} catch (err) {}
}

export async function login(account: Account): Promise<void> {
	if (appState.getLimited()) {
		return
	}

	account.action = 'login'

	if (!/.+@purdue\.edu$/.test(account.email)) {
		appState.setAuthError('Email must be a Purdue address')
		return
	}
	if (account.password === '' || account.password.length < Number(PUBLIC_MIN_PASSWORD_LENGTH)) {
		appState.setAuthError('Password must be 10+ characters')
		return
	}

	try {
		appState.nowLimited()

		await fetchBackend('/authenticate', account)

		appState.setStatus(Status.isVerifying, true)
		goto('/auth/verify')
	} catch (err) {}
}

export async function signup(account: Account, confirmPassword: string): Promise<void> {
	if (appState.getLimited()) {
		return
	}

	account.action = 'signup'

	if (!/.+@purdue\.edu$/.test(account.email)) {
		appState.setAuthError('Email must be a Purdue address')
		return
	}
	if (account.password === '' || account.password.length < Number(PUBLIC_MIN_PASSWORD_LENGTH)) {
		appState.setAuthError('Password must be 10+ characters')
		return
	}
	if (account.password !== confirmPassword) {
		appState.setAuthError('Passwords do not match')
		return
	}

	try {
		appState.nowLimited()

		await fetchBackend('/authenticate', account)

		appState.setStatus(Status.isVerifying, true)
		goto('/auth/verify')
	} catch (err) {}
}

export async function verify(auth_code: string): Promise<void> {
	if (appState.getLimited()) {
		return
	}

	if (!appState.getStatus(Status.isVerifying)) {
		return
	}

	if (!/^\d+$/.test(auth_code) || auth_code.length != 6) {
		appState.setAuthError('Only ' + PUBLIC_CODE_LENGTH + ' numbers')
		return
	}

	try {
		appState.nowLimited()

		await fetchBackend('/verify', { token: auth_code })

		appState.setStatus(Status.isSignedIn, true)
		goto('/browse')
	} catch (err) {}
}

export async function verify_forget(auth_code: string) {
	if (appState.getLimited()) {
		return
	}

	if (!appState.getStatus(Status.isVerifyingForgot)) {
		return
	}

	if (!/^\d+$/.test(auth_code) || auth_code.length != 6) {
		appState.setAuthError('Only ' + PUBLIC_CODE_LENGTH + ' numbers')
		return
	}

	try {
		appState.nowLimited()

		await fetchBackend('/verify', { token: auth_code })

		appState.setStatus(Status.isVerifyingUpdate, true)
		goto('/auth/verify/update')
	} catch (err) {}
}

export async function update(new_password: string) {
	if (appState.getLimited()) {
		return
	}

	if (!appState.getStatus(Status.isVerifyingUpdate)) {
		return
	}

	if (
		new_password === '' ||
		new_password.length > 100 ||
		new_password.length < Number(PUBLIC_MIN_PASSWORD_LENGTH)
	) {
		appState.setAuthError('Password must be 10+ characters')
		return
	}

	try {
		appState.nowLimited()

		await fetchBackend('/verify', { token: new_password })

		appState.setStatus(Status.isSignedIn, true)
		goto('/browse')
	} catch (err) {}
}

export async function signout() {
	if (appState.getLimited()) {
		return
	}

	if (!appState.getStatus(Status.isSignedIn)) {
		return
	}

	appState.nowLimited()

	const response = await fetch(PUBLIC_BACKEND_URL + '/delete', {
		method: 'DELETE',
		credentials: 'include'
	})

	if (!response.ok) {
		throw new Error(`HTTP error! status: ${response.status}`)
	}

	appState.setStatus(Status.isSignedIn, false)
}

export async function resend(resendSeconds: number) {
	if (resendSeconds != 0) {
		return
	}
	if (appState.getLimited()) {
		return
	}

	appState.nowLimited()

	const response = await fetch(PUBLIC_BACKEND_URL + '/resend', {
		method: 'POST',
		credentials: 'include'
	})

	if (!response.ok) {
		appState.setAuthError((await response.text()).slice(0, 50))
		throw new Error(`HTTP error! status: ${response.status}`)
	}
}
