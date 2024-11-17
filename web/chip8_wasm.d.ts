/* tslint:disable */
/* eslint-disable */
export class Chip8Wasm {
  free(): void;
  /**
   * @returns {Chip8Wasm}
   */
  static new(): Chip8Wasm;
  /**
   * @param {Uint8Array} rom
   */
  load_rom(rom: Uint8Array): void;
  emulate_cycle(): void;
  /**
   * @returns {any}
   */
  get_gfx(): any;
  /**
   * @param {number} key
   * @param {number} value
   */
  set_key(key: number, value: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_chip8wasm_free: (a: number, b: number) => void;
  readonly chip8wasm_new: () => number;
  readonly chip8wasm_load_rom: (a: number, b: number, c: number) => void;
  readonly chip8wasm_emulate_cycle: (a: number) => void;
  readonly chip8wasm_get_gfx: (a: number) => number;
  readonly chip8wasm_set_key: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
