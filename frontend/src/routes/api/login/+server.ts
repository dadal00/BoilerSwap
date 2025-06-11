import { BACKEND_URL } from '$env/static/private'
import type { RequestHandler } from '@sveltejs/kit'

export const GET: RequestHandler = async () => {
	const response = await fetch(BACKEND_URL + "/login")

	console.log(response)

	return new Response(response.body, {
		status: 200,
		headers: {
			'Content-Type': 'text/plain'
		}
	})
}
