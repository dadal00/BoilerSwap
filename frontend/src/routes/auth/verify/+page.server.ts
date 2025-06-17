import { error, redirect } from '@sveltejs/kit'
import { PRIVATE_BACKEND_URL } from '$env/static/private'
import type { Token } from '$lib/models.js'

export const load = async ({ url, fetch }) => {
	const token = url.searchParams.get('token')

	if (!token) {
		throw error(400, 'Missing token')
	}

	let payload: Token = { token: token }

	const response = await fetch(PRIVATE_BACKEND_URL + '/verify', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		credentials: 'include',
		body: JSON.stringify(payload)
	})

	if (!response.ok) {
		throw error(401, 'Invalid or expired token')
	}

	throw redirect(303, '/browse')
}
