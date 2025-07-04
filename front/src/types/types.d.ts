declare type PlanetName = 'CRATER' | 'DESERT' | 'GREEN' | 'SNOW';

declare type MapResName = 'CRATER_1' | 'CRATER_2' | 'CRATER_3' | 'CRATER_4' | 'CRATER_5' | 'CRATER_6' |
	'DESERT_1' | 'DESERT_2' | 'DESERT_3' | 'DESERT_4' | 'DESERT_5' | 'DESERT_6' |
	'GREEN_1' | 'GREEN_2' | 'GREEN_3' | 'GREEN_4' | 'GREEN_5' | 'GREEN_6' |
	'SNOW_1' | 'SNOW_2' | 'SNOW_3' | 'SNOW_4' | 'SNOW_5' | 'SNOW_6';

declare type MapHashId = string & { __mapHashId: never };

declare interface MapInfo {
	mapHashId: MapHashId;
	filePath: string;
	planetName: string;
	planetSlot: string;
	name: string;
	width: number;
	height: number;
	minimap: Vec<u8>;
	description: string;
	author: string;
	date: string;
	version: string;
	comments: string;
	isInstalled: boolean;
}

declare interface MapAndSaves {
	map: string;
	mapHashId: string;
	saves: string[];
}

declare type Result<T, E> = {
	ok: true;
	data: T;
} | {
	ok: false;
	error: E;
};

declare type Size = {
	width: number;
	height: number;
};
