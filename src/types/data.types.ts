export type ShapeNames = "3DFACE" | "3DFACE_TRIANGLE" | "LINE"
export type WASMDataType = {
	entity_type: ShapeNames,
	vertices: Array<Vertex>,
	row: RowData,
}
export type Vertex = {
	x: number,
	y: number,
	z: number
}
export type RowData = {
	id: number,
	as1: [number, number],
	as2: [number, number],
	as3: [number, number],
	as4: [number, number],
	asw1: [number, number],
	asw2: [number, number],
}
export type StepArmType = {
	mainStep: number | null,
	secondaryStep: number | null
}
export type TransformedNames = "3DFACES" | "3DFACE_TRIANGLES" | "LINES"
export type TransformedData = {
	[key in TransformedNames]: Array<number>
}

export type WasmDataJsType = {
	[key: string]: (MainfetchedWasmJSData & StepArmType)
}
export type MainfetchedWasmJSData = {
	"plates": WASMDataType[],
	"rods": WASMDataType[],
	"Materials": Array<number>,
	"maxAs1": number,
	"maxAs2": number,
	"maxAs3": number,
	"maxAs4": number,
	"maxAsw1": number,
	"maxAsw2": number,
}

export type ArmDiameters = {
	diameter: number,
	area: number,
}
export type SpecifiedFitParamsType = {
	diameter: number,
	area: number,
	price: number | null,
	isSpecified: boolean,
	isDefault: boolean
}
export type MaxAsFn = {
	[key: string]: number
}
export type PureWASMJsDataValue = {
	"plates": number,
	"rods": number,
	"Materials": Array<number>,
	"maxAs1": number,
	"maxAs2": number,
	"maxAs3": number,
	"maxAs4": number,
	"maxAsw1": number,
	"maxAsw2": number,
	"mainStep": number | null,
	"secondaryStep": number | null,
	"isSelected": boolean
}
export type PureWASMJsData = {
	[key: string]: PureWASMJsDataValue
}
export type UniqueFloorsExportToWASM = {
	title: string | null,
	level: string,
	max_as1: number,  // ← Изменено на snake_case
	max_as2: number,  // ← Изменено на snake_case
	max_as3: number,  // ← Изменено на snake_case
	max_as4: number,  // ← Изменено на snake_case
	steps: [number, number],
	color?: string | null
}

export type ExcelView = {
	level: string;
	title?: string;
	main_step: number;
	additional_step: number;
	values: ArmatureCombination[];
}
export type ArmatureCombination = {
	function_name: "as1" | "as2" | "as3" |"as4",
	as_target_value:number;
	combinations:CombinationItem[]
}

export type CombinationItem={
	main_diameter: number;
	additional_diameter: number;
	total_area: number;
	deviation: number;
	result_scale?: string; // Итоговая шкала для этой комбинации
}
export type CustomSettingsForCombinationItem={
	is_min_deviation:boolean;
	is_default_checked:boolean;
}
export type PreparedCombinationItem = CombinationItem & CustomSettingsForCombinationItem
export type PreparedArmatureCombination = {
	function_name: "as1" | "as2" | "as3" |"as4",
	as_target_value:number;
	combinations:PreparedCombinationItem[]
}
export type PreparedExcelView = {
	level: string;
	title?: string;
	main_step: number;
	additional_step: number;
	values: PreparedArmatureCombination[];
}

// Новые типы для передачи выбранных комбинаций в WASM
export type SelectedCombination = {
	floor_level: string;
	function_name: "as1" | "as2" | "as3" | "as4";
	as_target_value: number;
	combination: PreparedCombinationItem;
}

export type SelectedCombinationsData = {
	combinations: SelectedCombination[];
	floors: string[]; // Уникальные этажи для генерации
}