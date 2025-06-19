import { PRIVATE_BACKEND_URL } from '$env/static/private'

export const load = async ({ cookies }) => {
	const token = cookies.get('session_id')
	if (token) {
		fetch(PRIVATE_BACKEND_URL + '/delete', {
			method: 'DELETE',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ token: token })
		})

		cookies.delete('session_id', { path: '/' })
	}
}
