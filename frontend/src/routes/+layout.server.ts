export const load = async ({ cookies }) => {
	return {
		signedIn: !!cookies.get('session_id')
	}
}
