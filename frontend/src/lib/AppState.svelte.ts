import {
	ItemFields,
	Status,
	type Condition,
	type FullQuery,
	type Item,
	type ItemType,
	type Location
} from './models'
import { page } from '$app/state'

class AppState {
	private signedIn: boolean = $state(false)
	private toVerify: boolean = $state(false)
	private toVerifyForgot: boolean = $state(false)
	private toVerifyUpdate: boolean = $state(false)

	private lastAttempt: number = Date.now()

	private query: string = $state('')
	private totalHits: number = $state(0)
	private itemTypeFilter: ItemType | '' = $state('')
	private locationFilter: Location | '' = $state('')
	private conditionFilter: Condition | '' = $state('')
	private hits: Item[] = $state([])

	setQuery(query: string): void {
		this.query = query
	}

	setItemTypeFilter(filter: ItemType | ''): void {
		this.itemTypeFilter = filter
	}

	setLocationFilter(filter: Location | ''): void {
		this.locationFilter = filter
	}

	setConditionFilter(filter: Condition | ''): void {
		this.conditionFilter = filter
	}

	getFullQuery(): FullQuery {
		return {
			query: this.query,
			[ItemFields.ITEM_TYPE]: this.itemTypeFilter,
			[ItemFields.LOCATION]: this.locationFilter,
			[ItemFields.CONDITION]: this.conditionFilter
		}
	}

	setQueryResults(items: Item[], totalItems: number): void {
		this.hits = items
		this.totalHits = totalItems
	}

	getTotalHits(): number {
		return this.totalHits
	}

	getHits(): Item[] {
		return page.url.pathname.includes('/browse') ? this.hits : this.hits.slice(0, 3)
	}

	setLastAttempt(value: number): void {
		this.lastAttempt = value
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
	isProductLimited(): boolean {
		return Date.now() < this.lastAttempt + 5000
	}
}

export const appState = new AppState()
