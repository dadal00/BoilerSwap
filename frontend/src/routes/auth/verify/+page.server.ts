import { error, redirect } from '@sveltejs/kit'
import { PRIVATE_BACKEND_URL } from '$env/static/private'

export const load = async ({ url, fetch, cookies }) => {
	const token = url.searchParams.get('token')

	if (!token) {
		throw error(400, 'Missing token')
	}

	const response = await fetch(PRIVATE_BACKEND_URL + '/verify', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		credentials: 'include',
		body: JSON.stringify({ token: token })
	})

	if (!response.ok) {
		throw error(401, 'Invalid or expired token')
	}

	cookies.set('session_id', await response.text(), {
		path: '/',
		httpOnly: true,
		secure: true,
		sameSite: 'strict',
		maxAge: 60 * 60
	})

	throw redirect(303, '/browse')
}
