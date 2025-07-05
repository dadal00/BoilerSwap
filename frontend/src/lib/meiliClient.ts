import { PUBLIC_MEILI_URL, PUBLIC_MEILI_KEY } from '$env/static/public'
import { Meilisearch } from 'meilisearch'
import {
	ItemFields,
	ItemsTableName,
	type Condition,
	type Item,
	type ItemType,
	type Location
} from './models'
import { appState } from './AppState.svelte'

const client = new Meilisearch({ host: PUBLIC_MEILI_URL, apiKey: PUBLIC_MEILI_KEY })

export async function search(
	query: string,
	itemTypeFilter: ItemType | '',
	locationFilter: Location | '',
	conditionFilter: Condition | ''
) {
	const filters: string[] = []

	if (itemTypeFilter !== '') {
		filters.push(`${ItemFields.ITEM_TYPE} = ${itemTypeFilter}`)
	}

	if (locationFilter !== '') {
		filters.push(`${ItemFields.LOCATION} = ${locationFilter}`)
	}

	if (conditionFilter !== '') {
		filters.push(`${ItemFields.CONDITION} = ${conditionFilter}`)
	}

	const response = await client.index(ItemsTableName).search(query, {
		filter: filters
	})
	appState.setQueryResults(response.hits as Item[], response.estimatedTotalHits)
}
