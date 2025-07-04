declare type LockPromise = Promise<void>;

declare type RGBA = Uint8Array;
declare type u8 = Uint8Array;
declare type u16 = Uint16Array;
declare type u32 = Uint32Array;

declare type HEX = string;

declare type Vec<T> = T & { __type: T };

declare interface Window {
	_: any;
	openTauriConsole: () => void;
}

declare interface ImportMetaEnv {
	readonly DEV: boolean;
}

declare interface ImportMeta {
	readonly env: ImportMetaEnv;
}
