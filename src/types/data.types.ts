export type ShapeNames = "3DFACE"|"3DFACE_TRIANGLE"|"LINE"
export type WASMDataType={
	entity_type:ShapeNames,
	vertices:Array<Vertex>,
	row:Array<RowData>,
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
}
export type TransformedNames = "3DFACES"|"3DFACE_TRIANGLES"|"LINES"
export type TransformedData={
	[key in TransformedNames]:Array<number>
}