export type orientation = "horizontal" | "vertical"

export type FaceType = {
    "entity_type": ShapeType,
    "vertices": FaceVerticesType
    "handle": string,
    "ownerHandle": string,
    "layer": string,
    "color_id":number
}
export type LineType = {
    "entity_type": ShapeType,
    "vertices": LineVerticesType,
    "handle": string,
    "ownerHandle": string,
    "layer": string,
    "color_id":number
}
export type TriangleFaceType = {
    "entity_type": ShapeType,
    "vertices": TriangleVerticesType,
    "handle": string,
    "ownerHandle": string,
    "layer": string,
    "color_id":number
}
export type ShapeType="LINE"|"3DFACE"
export type VerticesType = {
	"x": number,
	"y": number,
	"z": number
}
export type LineVerticesType=[VerticesType, VerticesType]
export type FaceVerticesType=[VerticesType, VerticesType, VerticesType, VerticesType]
export type TriangleVerticesType = [VerticesType, VerticesType, VerticesType,]
export type MainDataType=Array<LineType|FaceType|TriangleFaceType>