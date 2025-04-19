import { VerticesType } from "../types/types";

export type incomeLineItem = {
	"type": "LINE",
	"vertices": [
		{
			"x": number,
			"y": number,
			"z": number
		},
		{
			"x": number,
			"y": number,
			"z": number
		}
	],
	"handle": "11A",
	"ownerHandle": "1F",
	"layer": "0"
}

export function getVerticesArray(vertices: VerticesType[]): Array<number> {
	return vertices
		.filter(vertex => vertex.x != null && vertex.y != null && vertex.z != null)
		.reduce((acc, vertex) => {
			acc.push(vertex.x, vertex.y, vertex.z);
			return acc;
		}, [] as number[]);
}

export function getLinesVerticesArray(lines: VerticesType[]): Array<number> {
	return getVerticesArray(lines);
}

export function getFacesVerticesArray(faces: VerticesType[]): Array<number> {
	return faces
		.filter(vertex => vertex.x != null && vertex.y != null && vertex.z != null)
		.reduce((acc, vertex) => {
			acc.push(vertex.x, vertex.y, vertex.z);
			return acc;
		}, [] as number[]);
}
