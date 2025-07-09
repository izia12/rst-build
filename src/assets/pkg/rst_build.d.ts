/* tslint:disable */
/* eslint-disable */
export function create_custom_sortament_report(available_diameters: Uint32Array, floors_data_json: string): Uint8Array;
export function get_table_data(available_diameters: Uint32Array, floors_data_json: string): string;
export function log_data(x: number, y: number): void;
export function str_log_data(str: string): void;
export function parse_data(sli_data: string, txt_data: string, xlsx_data: Uint8Array): void;
export function convert_sli_xsl_to_json_string(): string;
export function convert_data_to_js_order_byz(): string;
export function create_docx(sli_data: string, txt_data: string, xlsx_data: Uint8Array): Uint8Array;
export function create_png_in_memory(): Uint8Array;
export function process_files(sli_data: string, txt_data: string, xlsx_data: Uint8Array): string;
export function get_changed_row_data(planes: any): Uint8Array;
export function get_sortament_data(): any;
export function find_combinations_with_custom_diameters(target_area: number, main_step: number, secondary_step: number, available_diameters: any): any;
export function create_csv_from_all_parsed_entities(): Uint8Array;
export function get_excell_report_for_arms(): Uint8Array;
export function get_custom_sortament_report(available_diameters: Uint32Array, floors_data_json: string): Uint8Array;
export function get_table_data_for_frontend(available_diameters: Uint32Array, floors_data_json: string): string;
export function get_horizontal_elements_object_js(): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly create_custom_sortament_report: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly get_table_data: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly log_data: (a: number, b: number) => void;
  readonly str_log_data: (a: number, b: number) => void;
  readonly parse_data: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly convert_sli_xsl_to_json_string: () => [number, number];
  readonly convert_data_to_js_order_byz: () => [number, number];
  readonly create_docx: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly create_png_in_memory: () => [number, number];
  readonly process_files: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly get_changed_row_data: (a: any) => [number, number];
  readonly get_sortament_data: () => any;
  readonly find_combinations_with_custom_diameters: (a: number, b: number, c: number, d: any) => any;
  readonly create_csv_from_all_parsed_entities: () => [number, number];
  readonly get_excell_report_for_arms: () => [number, number];
  readonly get_custom_sortament_report: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly get_table_data_for_frontend: (a: number, b: number, c: number, d: number) => [number, number];
  readonly get_horizontal_elements_object_js: () => any;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
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
