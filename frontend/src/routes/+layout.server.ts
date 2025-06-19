export const load = async ({ cookies }) => {
    console.log("Running!")
	return {
		signedIn: !!cookies.get('session_id')
	}
}
