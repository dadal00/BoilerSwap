import { PUBLIC_BACKEND_URL } from '$env/static/public'
import type { Account, TokenPayload } from '../models'

export async function fetchBackend(path: string, payload: Account | TokenPayload) {
	const response = await fetch(PUBLIC_BACKEND_URL + path, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify(payload)
	})

	if (!response.ok) {
		throw new Error(`HTTP error! status: ${response.status}`)
	}
}
