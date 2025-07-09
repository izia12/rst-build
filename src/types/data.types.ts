export type ShapeNames = "3DFACE"|"3DFACE_TRIANGLE"|"LINE"
export type WASMDataType={
	entity_type:ShapeNames,
	vertices:Array<Vertex>,
	row:RowData,
}
export type Vertex={
	x:number,
	y:number,
	z:number
}
export type RowData = {
	id:number,
	as1:[number, number],
	as2:[number, number],
	as3:[number, number],
	as4:[number, number],
	asw1:[number, number],
	asw2:[number, number],
}
export type StepArmType ={
	mainStep:number | null,
	secondaryStep:number |null
}
export type TransformedNames = "3DFACES"|"3DFACE_TRIANGLES"|"LINES"
export type TransformedData={
	[key in TransformedNames]:Array<number>
}

export type WasmDataJsType = {
	[key:string]:(MainfetchedWasmJSData & StepArmType)
}
export type MainfetchedWasmJSData={
		"plates":WASMDataType[],
		"rods":WASMDataType[],
		"Materials":Array<number>,
		"maxAs1":number,
		"maxAs2":number,
		"maxAs3":number,
		"maxAs4":number,
		"maxAsw1":number,
		"maxAsw2":number,
}

export type ArmDiameters={
	diameter:number,
    area:number,
}
export type SpecifiedFitParamsType = {
	diameter:number,
    area:number,
	price:number | null,
	isSpecified:boolean,
	isDefault:boolean
}
export type MaxAsFn={
	[key:string]:number
}
export type PureWASMJsDataValue={
	"plates":number,
	"rods":number,
	"Materials":Array<number>,
	"maxAs1":number,
	"maxAs2":number,
	"maxAs3":number,
	"maxAs4":number,
	"maxAsw1":number,
	"maxAsw2":number,
	"mainStep":number | null,
	"secondaryStep":number |null,
	"isSelected":boolean
}
export type PureWASMJsData={
	[key:string]:PureWASMJsDataValue
}
export type UniqueFloorsExportToWASM={
	title:string | null,
	level:string,
	max_as1:number,  // ← Изменено на snake_case
	max_as2:number,  // ← Изменено на snake_case
	max_as3:number,  // ← Изменено на snake_case
	max_as4:number,  // ← Изменено на snake_case
	steps:[number, number],
	color?:string | null
}
export type ArmatureCombination ={
    main_diameter: number;        // d1
    additional_diameter: number;  // d2 (0 если нет)
    total_area: number;          // общая площадь
    deviation: number;           // отклонение от target_area
    main_armature: string;       // "Ø16 мм"
    additional_armature: string; // "Ø12 мм" или "Нет"
}
export type ExcelView ={
    level: string;
    title?: string;
    function_name: string;
    target_area: number;
    main_step: number;
    additional_step: number;
    combinations: ArmatureCombination[]; // ← Массив всех найденных комбинаций!
    selected_combination_index: number;  // Индекс выбранной комбинации
}
