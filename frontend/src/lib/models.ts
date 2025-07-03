export type TokenPayload = {
	token: string
}

export const TabOptionsIterable = [
	'Login',
	'Signup',
	'Reset',
  ] as const;

export type TabOptions = typeof TabOptionsIterable[number];

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
	'Other',
  ] as const;
  
export type ItemType = typeof ItemTypeIterable[number];

export const ConditionIterable = [
	'Excellent',
	'Good',
	'Fair',
  ] as const;
  
export type Condition = typeof ConditionIterable[number];

export const ConditionEmojis: Record<Condition, string> = {
	Excellent: '✨',
	Good: '✅',
	Fair: '🟡',
}

export const ConditionLabels: Record<Condition, string> = {
	Excellent: 'Excellent - Like new, minimal wear',
	Good: 'Good - Some wear but fully functional',
	Fair: 'Fair - Noticeable wear but still usable',
}

export type Emoji = 'chair' | 'snowflake' | 'books' | 'pan' | 'monitor' | 'decor'

export const EmojiLabels: Record<Emoji, string> = {
	chair: '🪑',
	snowflake: '❄️',
	books: '📚',
	pan: '🍳',
	monitor: '🖥️',
	decor: '🎨'
}

export const LocationIterable = [
	'CaryQuadEast',
	'WileyHall',
	'HarrisonHall',
	'EarhartHall',
	'HillenbrandHall',
	'ThirdStreetSuites',
  ] as const;
  
export type Location = typeof LocationIterable[number];

export const LocationLabels: Record<Location, string> = {
	CaryQuadEast: 'Cary Quad - East',
	WileyHall: 'Wiley Hall',
	HarrisonHall: 'Harrison Hall',
	EarhartHall: 'Earhart Hall',
	HillenbrandHall: 'Hillenbrand Hall',
	ThirdStreetSuites: 'Third Street Suites',
};

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
