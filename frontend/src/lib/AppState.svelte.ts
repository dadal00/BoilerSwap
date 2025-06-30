import { Attempt, Status } from './models'

class AppState {
	private signedIn: boolean = $state(false)
	private toVerify: boolean = $state(false)
	private toVerifyForgot: boolean = $state(false)
	private toVerifyUpdate: boolean = $state(false)

	private lastAttempt: number = Date.now()

	getAttempts(attempt: Attempt): number {
		switch (attempt) {
			case Attempt.lastAttempt:
				return this.lastAttempt
			default:
				throw new Error('Invalid flag')
		}
	}

	setAttempts(attempt: Attempt, value: number): void {
		switch (attempt) {
			case Attempt.lastAttempt:
				this.lastAttempt = value
				break
			default:
				throw new Error('Invalid flag')
		}
	}

	getStatus(status: Status): boolean {
		switch (status) {
			case Status.isSignedIn:
				return this.signedIn
			case Status.isVerifying:
				return this.toVerify
			case Status.isVerifyingForgot:
				return this.toVerifyForgot
			case Status.isVerifyingUpdate:
				return this.toVerifyUpdate
			default:
				throw new Error('Invalid flag')
		}
	}

	setStatus(status: Status, value: boolean): void {
		switch (status) {
			case Status.isSignedIn:
				this.signedIn = value
				break
			case Status.isVerifying:
				this.toVerify = value
				break
			case Status.isVerifyingForgot:
				this.toVerifyForgot = value
				break
			case Status.isVerifyingUpdate:
				this.toVerifyUpdate = value
				break
			default:
				throw new Error('Invalid flag')
		}
	}

	isLimited(): boolean {
		return Date.now() < this.lastAttempt + 500
	}
}

export const appState = new AppState()
