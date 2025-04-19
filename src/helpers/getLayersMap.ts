import { MainDataType } from "../types/types";
import { getLinesVerticesArray} from "./getLinesVerticesArray.ts";

export type MappedColorValue = {
	// entity_type: "LINE" | "FACE" | "TRIANGLE_FACE",
	lines?: Array<number>,
	faces?: Array<number>,
	triangle_faces?: Array<number>
	[key: string]: number[] | undefined;

}
export const colors = [
	"#FF0000", "#FF1A00", "#FF3300", "#FF4D00", "#FF6600",
	"#FF8000", "#FF9900", "#FFB300", "#FFCC00", "#FFFF00",
	"#E6FF00", "#CCFF00", "#B3FF00", "#99FF00", "#80FF00",
	"#66FF00", "#4DFF00", "#33FF00", "#1AFF00", "#00FF00",
	"#00FF1A", "#00FF33", "#00FF4D", "#00FF66", "#00FF80",
	"#00FF99", "#00FFB3", "#00FFCC", "#00FFDF", "#00FFFF",
	"#00E6FF", "#00CCFF", "#00B3FF", "#00A1FF", "#0094FF",
	"#0087FF", "#0079FF", "#0069FF", "#005DFF", "#004DFF",
	"#0041FF", "#003DFF", "#0039FF", "#0033FF", "#1A00FF",
	"#3300FF", "#4D00FF", "#6600FF", "#8000FF", "#9900FF",
	"#B300FF", "#CC00FF", "#DF00FF", "#FF00D1", "#FF00B3",
	"#FF00A1", "#FF0099", "#FF0080", "#FF0066", "#FF0054",
	"#FF004D", "#FF0033", "#FF0022", "#FF001A", "#FF4D4D",
	"#FF7F7F", "#FFB1B1", "#FFA6A6", "#FF9999", "#FF8787",
	"#FF7575", "#FF6666", "#FF5C5C", "#FF4D4D", "#FF4444",
	"#FF3F3F", "#FF3333", "#FF2A2A", "#FF2222", "#FF1A1A",
	"#FF1111", "#FF0909", "#FF0000", "#E50000", "#CC0000",
	"#B30000", "#990000", "#7F0000", "#660000", "#4D0000",
	"#330000", "#191919", "#333333", "#4D4D4D", "#666666",
	"#7F7F7F", "#999999", "#B2B2B2", "#CCCCCC", "#DFDFDF",
	"#EEEEEE", "#F5F5F5", "#FFFFFF", "#FFCC99", "#FFD700",
	"#FFC300", "#FFB800", "#FFA600", "#FF9200", "#FF7E00",
	"#FF6B00", "#FF5800", "#FF4400", "#FF3300", "#FF2200",
	"#FF1100", "#FF0000", "#D97100", "#EFC300", "#F6E000",
	"#FFEE00", "#EEFF00", "#C8FF00", "#A7FF00", "#85FF35",
	"#6FFF5D", "#57FF85", "#48FFA0", "#40FFD3", "#35D5FF",
	"#42A8FF", "#4B80FF", "#6A6EFF", "#7474FF", "#8F8FFF",
	"#A8A8FF", "#B3C5FF", "#B3B3FF", "#C9C9FF", "#D4D4FF",
	"#E8E8FF", "#F0F0FF", "#FF80BF", "#FF5B5B", "#FF6F68",
	"#FF7F95", "#FF8989", "#FFB2A2", "#FFCCB1", "#FFDBD1"
];
export type MappedShapesByColor = {
	[key:string]:MappedColorValue
}

export function getLayersMap(data: MainDataType): MappedShapesByColor {
	const mappedShapesByColor: MappedShapesByColor = {};
	for (let i = 0; i < data.length; i++) {
		const item = data[i];
		const colorId = item.color_id
		const color = colors[colorId];
		const verticesArray = getLinesVerticesArray(item.vertices)
		const propertyName = item.entity_type === "LINE" ? "lines" : (item.entity_type === "3DFACE") &&item.vertices.length ===3 ? "triangle_faces":"faces" ;

		if (!(color in mappedShapesByColor)) {
			mappedShapesByColor[color] = {
				[propertyName]: verticesArray,

			};
		} else {
			if(!(propertyName in mappedShapesByColor[color])){
				mappedShapesByColor[color][propertyName] = [];
			}
			mappedShapesByColor[color][propertyName]?.push(...verticesArray);
		}
	}

	return mappedShapesByColor;
}