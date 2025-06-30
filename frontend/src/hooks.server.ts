import type { Handle } from '@sveltejs/kit'
import { env } from '$env/dynamic/private'

export const handle: Handle = async ({ event, resolve }) => {
	event.cookies.set('api_token', `${env.API_TOKEN}`, {
		path: '/',
		httpOnly: true,
		sameSite: 'strict',
		secure: true
	})

	return resolve(event)
}
