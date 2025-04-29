import { transformLinesPointsIntoArray } from '../helpers/transformLinesPointsIntoArray';
import { WASMDataType } from "../types/data.types";

export const processGeometryData = (data: WASMDataType[]) => {
  const geometryGroups = {
    LINES: [] as number[],
    '3DFACE_TRIANGLES': [] as number[],
    '3DFACES': [] as number[],
  };

  data.forEach(item => {
    const vertices = item.vertices.flatMap(v => [v.x, v.y, v.z]);
    
    if (item.vertices.length === 2) {
      geometryGroups.LINES.push(...vertices);
    } else if (item.vertices.length === 3) {
      geometryGroups['3DFACE_TRIANGLES'].push(...vertices);
    } else if (item.vertices.length === 4) {
      geometryGroups['3DFACES'].push(...vertices);
    }
  });

  return {
    lines: transformLinesPointsIntoArray('LINES', geometryGroups.LINES, '#ffffff'),
    triangles: transformLinesPointsIntoArray('TRIANGLE_FACES', geometryGroups['3DFACE_TRIANGLES'], '#ffffff'),
    faces: transformLinesPointsIntoArray('3DFACES', geometryGroups['3DFACES'], '#ffffff')
  };
};