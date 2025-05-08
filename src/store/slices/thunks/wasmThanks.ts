import { createAsyncThunk } from "@reduxjs/toolkit"
import init, { convert_sli_xsl_to_json_string, get_changed_row_data, get_horizontal_elements_object_js, parse_data } from '../../../assets/pkg/rst_build'
import { WasmDataJsType, WASMDataType } from "../../../types/data.types"
export const fetchWasmData = createAsyncThunk<Array<WASMDataType>, {sliData:string, xlsxData:Uint8Array}>(
	'users/fetchByIdStatus',
	async ({sliData, xlsxData} , thunkAPI) => {
        try {
			await init()
			parse_data(sliData, xlsxData)
			const changedData = await get_changed_row_data([8.3,11.6,14.9])
			console.log(changedData, "Changed Data");

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

