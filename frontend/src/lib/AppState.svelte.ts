class AppState {
	private signedIn: boolean = $state(false)

	isSignedIn() {
		return this.signedIn
	}

	setSignedIn(signedIn: boolean) {
		this.signedIn = signedIn
	}
}

export const appState = new AppState()
