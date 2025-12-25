export function createMapSlotName(planet: PlanetName, slotIndex: number): MapResName {
	return `${planet}_${slotIndex + 1}` as MapResName;
}
