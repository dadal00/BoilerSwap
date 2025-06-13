export type Account = {
	email: string
	password: string
}

export type Verification = {
	id: string
	code: string
}

export type ItemType = 'Furniture' | 'Electronics' | 'Books' | 'Kitchen' | 'Clothing'

export type Condition = 'Excellent' | 'Good' | 'Fair'

export type Location =
	| 'Cary Quad - East'
	| 'Wiley Hall'
	| 'Harrison Hall'
	| 'Earhart Hall'
	| 'Hillenbrand Hall'
	| 'Third Street Suites'

export type Product = {
	item_type: ItemType
	title: string
	condition: Condition
	location: Location
	description?: string
}
