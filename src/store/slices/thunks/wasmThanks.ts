import { createAsyncThunk } from "@reduxjs/toolkit"

import { ArmDiameters, MainfetchedWasmJSData, PureWASMJsData, WasmDataJsType, WASMDataType } from "../../../types/data.types"
import init, { convert_sli_xsl_to_json_string, get_horizontal_elements_object_js, get_sortament_data, parse_data } from "../../../assets/pkg/rst_build"
import { getPureWASMJsData } from "../../../helpers/getPureWASMJsData"
export const fetchWasmData = createAsyncThunk<Array<WASMDataType>, {sliData:string, txtData:string, xlsxData:Uint8Array}>(
	'users/fetchByIdStatus',
	async ({sliData,txtData, xlsxData} , thunkAPI) => {
        try {
			await init()
			parse_data(sliData,txtData, xlsxData)
			return JSON.parse(convert_sli_xsl_to_json_string())
        } catch (error) {
            return thunkAPI.rejectWithValue(error);
        }
	}
)
export const fetchWasmJSData = createAsyncThunk<PureWASMJsData, undefined>(
	'data/fetchWasmJsData',
	async (_, thunkAPI) => {
        try {
            await init();
            const result = get_horizontal_elements_object_js();
            if (!result) throw new Error('WASM data not ready'); // Добавляем проверку
			console.log( result);
			// console.log( getPureWASMJsData(result),"fjhdjfhjfhjdf");
			const newRes = await getPureWASMJsData(result)
            return newRes;
        } catch (error) {
            return thunkAPI.rejectWithValue(error instanceof Error ? error.message : 'Unknown error');
        }
    }
);
export const fetchArmDimeters = createAsyncThunk<ArmDiameters[], undefined>(
	'data/fetchArmDimeters',
	async (_, thunkAPI) => {
        try {
            await init();
            const result = get_sortament_data();
            if (!result) throw new Error('WASM data not ready'); // Добавляем проверку
			console.log(result,"hello");
			
            return result as ArmDiameters[];
        } catch (error) {
            return thunkAPI.rejectWithValue(error instanceof Error ? error.message : 'Unknown error');
        }
    }
);

