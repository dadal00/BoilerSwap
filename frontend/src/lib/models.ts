import type { Dictionary } from 'meilisearch'

export type TokenPayload = {
	token: string
}

export const TabOptionsIterable = ['Login', 'Signup', 'Reset'] as const

export type TabOptions = (typeof TabOptionsIterable)[number]

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

export type ExpirationColor = 'green' | 'yellow' | 'red'

export const ItemTypeIterable = [
	'Furniture',
	'Electronics',
	'Books',
	'Kitchen',
	'Clothing',
	'Decor',
	'Other'
] as const

export type ItemType = (typeof ItemTypeIterable)[number]

export const ConditionIterable = ['Excellent', 'Good', 'Fair'] as const

export type Condition = (typeof ConditionIterable)[number]

export const ConditionEmojis: Record<Condition, string> = {
	Excellent: '✨',
	Good: '✅',
	Fair: '🟡'
}

export const ConditionLabels: Record<Condition, string> = {
	Excellent: 'Excellent - Like new, minimal wear',
	Good: 'Good - Some wear but fully functional',
	Fair: 'Fair - Noticeable wear but still usable'
}

export const EmojiIterable = ['Chair', 'Snowflake', 'Books', 'Pan', 'Monitor', 'Decor'] as const

export type Emoji = (typeof EmojiIterable)[number]

export const EmojiLabels: Record<Emoji, string> = {
	Chair: '🪑',
	Snowflake: '❄️',
	Books: '📚',
	Pan: '🍳',
	Monitor: '🖥️',
	Decor: '🎨'
}

export const LocationIterable = [
	'CaryQuadEast',
	'WileyHall',
	'HarrisonHall',
	'EarhartHall',
	'HillenbrandHall',
	'ThirdStreetSuites'
] as const

export type Location = (typeof LocationIterable)[number]

export const LocationLabels: Record<Location, string> = {
	CaryQuadEast: 'Cary Quad - East',
	WileyHall: 'Wiley Hall',
	HarrisonHall: 'Harrison Hall',
	EarhartHall: 'Earhart Hall',
	HillenbrandHall: 'Hillenbrand Hall',
	ThirdStreetSuites: 'Third Street Suites'
}

export const ItemsTableName = 'items'

export const ItemFields = {
	ITEM_ID: 'item_id',
	ITEM_TYPE: 'item_type',
	TITLE: 'title',
	CONDITION: 'condition',
	LOCATION: 'location',
	DESCRIPTION: 'description',
	EMOJI: 'emoji',
	EXPIRATION_DATE: 'expiration_date',
	FORMATTED: '_formatted'
} as const

export type Item = {
	[ItemFields.ITEM_ID]?: string
	[ItemFields.ITEM_TYPE]: ItemType
	[ItemFields.TITLE]: string
	[ItemFields.CONDITION]: Condition
	[ItemFields.LOCATION]: Location
	[ItemFields.DESCRIPTION]: string
	[ItemFields.EMOJI]: Emoji
	[ItemFields.EXPIRATION_DATE]?: string
	[ItemFields.FORMATTED]?: Dictionary
}

export enum Status {
	isSignedIn,
	isVerifying,
	isVerifyingForgot,
	isVerifyingUpdate
}

export type FullQuery = {
	query: string
	[ItemFields.ITEM_TYPE]: ItemType | ''
	[ItemFields.LOCATION]: Location | ''
	[ItemFields.CONDITION]: Condition | ''
	offset: number
}

export type VerifcationType = 'verify' | 'forget'
