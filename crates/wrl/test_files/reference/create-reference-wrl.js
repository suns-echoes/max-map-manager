#! /usr/bin/env node
import { writeFileSync } from 'fs';

const TILE_DATA_SIZE = 4096;
const TILE_COUNT = 64;
const WIDTH = 16;
const HEIGHT = 16;

const header = Buffer.from([ 0x57, 0x52, 0x4c, 0x01, 0x00 ]);
const width = Buffer.from([ WIDTH, 0x00 ]);
const height = Buffer.from([ HEIGHT, 0x00 ]);
const minimap = Buffer.from(Array(WIDTH * HEIGHT).fill(0x00).map((_, i) => i % TILE_COUNT));
const bigmap = Buffer.from(
	new Array(WIDTH * HEIGHT * 2).fill(0x00).map((_, i) => i % 2 ? 0x00 : i % TILE_COUNT),
);
const tileCount = Buffer.from([ TILE_COUNT & 0xff, (TILE_COUNT >> 8) & 0xff ]);
const tileData = Buffer.from(
	new Array(TILE_COUNT * TILE_DATA_SIZE).fill(0x00).map((_, i) => ((i / TILE_DATA_SIZE) | 0) % 256)
);
const palette = Buffer.from(
	new Array(256 * 3).fill(0x00).map((_, i) => (i / 3) | 0),
);
const passTable = Buffer.from(new Array(TILE_COUNT).fill(0x00));

const wrlBuf = Buffer.concat([
	header,
	width,
	height,
	minimap,
	bigmap,
	tileCount,
	tileData,
	palette,
	passTable,
]);

writeFileSync('REF.WRL', wrlBuf);
