import type { Cookies } from '@sveltejs/kit'

export const load = async ({ cookies }: { cookies: Cookies }) => {
	return {
		signedIn: !!cookies.get('session_id')
	}
}
