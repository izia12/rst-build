import { WASMDataType } from "../types/data.types";

export type TransformedWDTByPlates = {
	plates:WASMDataType[],
	rods:WASMDataType[]
}
export const transformMainWDT_To_Order_Z = (data:WASMDataType[]):Map<number, TransformedWDTByPlates>=>{
	const rodElements = []
	const hashMap:Map<number, TransformedWDTByPlates> = new Map();
	for(let i=0; i<data.length; i++){
		const item = data[i];
		const z = item.vertices[0].z;
		const isPlateEl = item.vertices.every(v=>z===v.z);
		if(!isPlateEl){
			rodElements.push(item);
			continue
		}
		if(!z){
			if(isPlateEl){
				hashMap.set(z, {plates:[item], rods:[]})
			}
		}
		hashMap.get(z)?.plates.push(item)
	}
	const sortedEntries = [...hashMap.entries()].sort(([keyA], [keyB]) => keyA - keyB);  
	const sortedMap = new Map(sortedEntries);
	return sortedMap
}