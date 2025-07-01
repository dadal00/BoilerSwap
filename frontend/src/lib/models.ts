export type AccountAction = 'login' | 'signup'

export type Account = {
	email: string
	password: string
	action: AccountAction
}

export type Verification = {
	id: string
	code: string
}

export type ItemType = 'Furniture' | 'Electronics' | 'Books' | 'Kitchen' | 'Clothing' | 'Other'

export type Condition = 'Excellent' | 'Good' | 'Fair'

export type Location =
	| 'CaryQuadEast'
	| 'WileyHall'
	| 'HarrisonHall'
	| 'EarhartHall'
	| 'HillenbrandHall'
	| 'ThirdStreetSuites'

export type Product = {
	item_type: ItemType
	title: string
	condition: Condition
	location: Location
	description?: string
}

export enum Status {
	isSignedIn,
	isVerifying,
	isVerifyingForgot,
	isVerifyingUpdate
}
