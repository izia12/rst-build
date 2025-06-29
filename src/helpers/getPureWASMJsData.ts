import {  PureWASMJsData, WasmDataJsType } from "../types/data.types"

export async function getPureWASMJsData(data:WasmDataJsType):Promise<PureWASMJsData>{
	return new Promise((resolve, reject)=>{
		const resultObj={} as PureWASMJsData
		for (const key in data){
			const item = data[key];
			resultObj[key]={
				"plates":item.plates?.length,
				"rods":item.rods?.length,
				"Materials":item.Materials,
				"maxAs1":item.maxAs1,
				"maxAs2":item.maxAs2,
				"maxAs3":item.maxAs3,
				"maxAs4":item.maxAs4,
				"maxAsw1":item.maxAsw1,
				"maxAsw2":item.maxAsw2,
				"mainStep": null,
				"secondaryStep": null,
				"isSelected":false
			}
		}
		resolve(resultObj)
	})
	
}