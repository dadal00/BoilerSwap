import { redirect, type Cookies } from '@sveltejs/kit'

export const load = async ({ cookies }: { cookies: Cookies }) => {
	if (!cookies.get('session_id')) {
		redirect(302, '/auth')
	}
}
