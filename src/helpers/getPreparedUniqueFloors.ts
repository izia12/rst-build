import { PureWASMJsDataValue, UniqueFloorsExportToWASM } from "../types/data.types";

export function getPreparedUniqueFloor(entries: [string, PureWASMJsDataValue][]):UniqueFloorsExportToWASM[]{
	const main_arr:UniqueFloorsExportToWASM[] =[]
	for (let i = 0; i<entries.length; i++){
		const item  = entries[i];
		if(
			(item[1].maxAs1>=0 ||
			item[1].maxAs2>=0 ||
			item[1].maxAs3>=0 ||
			item[1].maxAs4>=0)
		){
			const preparedItem:UniqueFloorsExportToWASM = {
				level:item[0],
				max_as1:item[1].maxAs1,  // ← Изменено на snake_case
				max_as2:item[1].maxAs2,  // ← Изменено на snake_case
				max_as3:item[1].maxAs3,  // ← Изменено на snake_case
				max_as4:item[1].maxAs4, 
				title:"",
				steps:[item[1].mainStep??200, item[1].secondaryStep??200],
				color:"",
			}
			main_arr.push(preparedItem)
		}
	}
	return main_arr;
}