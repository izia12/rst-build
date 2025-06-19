import { createAsyncThunk } from "@reduxjs/toolkit"

import { ArmDiameters, WasmDataJsType, WASMDataType } from "../../../types/data.types"
import init, { convert_sli_xsl_to_json_string, get_horizontal_elements_object_js, get_sortament_data, parse_data } from "../../../assets/pkg/rst_build"
export const fetchWasmData = createAsyncThunk<Array<WASMDataType>, {sliData:string, txtData:string, xlsxData:Uint8Array}>(
	'users/fetchByIdStatus',
	async ({sliData,txtData, xlsxData} , thunkAPI) => {
        try {
			await init()
			parse_data(sliData,txtData, xlsxData)

			return JSON.parse(convert_sli_xsl_to_json_string())
        } catch (error) {
            // Если ошибка не является экземпляром AxiosError или нет ответа, просто возвращаем ошибку как есть
            return thunkAPI.rejectWithValue(error);
        }
	}
)
export const fetchWasmJSData = createAsyncThunk<WasmDataJsType, undefined>(
	'data/fetchWasmJsData',
	async (_, thunkAPI) => {
        try {
            await init();
            const result = get_horizontal_elements_object_js();
            if (!result) throw new Error('WASM data not ready'); // Добавляем проверку
            return result as WasmDataJsType;
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

