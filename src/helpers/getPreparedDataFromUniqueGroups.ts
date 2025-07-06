import { UniqeItem } from "../store/slices/slice.wasm";
import { UniqueFloorsExportToWASM } from "../types/data.types";

export function getPreparedDataFromUniqueGroups(uniqueGroups:UniqeItem[]):UniqueFloorsExportToWASM[]{
	const resultArr:UniqueFloorsExportToWASM[] = []
	for (let i=0; i<uniqueGroups.length; i++){
		const item  = uniqueGroups[i];
		resultArr.push({
			level:item.planes.toString(),
			max_as1:Math.max(...item.maxAsValues.map(el=>el.as1)),  // ← Изменено на snake_case
			max_as2:Math.max(...item.maxAsValues.map(el=>el.as2)),  // ← Изменено на snake_case
			max_as3:Math.max(...item.maxAsValues.map(el=>el.as3)),  // ← Изменено на snake_case
			max_as4:Math.max(...item.maxAsValues.map(el=>el.as4)),  // ← Изменено на snake_case
			steps:[item.steps.mainStep, item.steps.secondaryStep],
			title:item.name,
			color:item.color
		})
	}
	return resultArr
}