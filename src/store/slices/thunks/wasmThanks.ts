import { createAsyncThunk } from "@reduxjs/toolkit"
import init, { convert_sli_xsl_to_json_string, parse_data } from '../../../assets/pkg/rst_build'
import { WASMDataType } from "../../../types/data.types"
export const fetchWasmData = createAsyncThunk<Array<WASMDataType>, {sliData:string, xlsxData:Uint8Array}>(
	'users/fetchByIdStatus',
	async ({sliData, xlsxData} , thunkAPI) => {
        try {
			await init()
			parse_data(sliData, xlsxData)
			return JSON.parse(convert_sli_xsl_to_json_string())
        } catch (error) {
            // Если ошибка не является экземпляром AxiosError или нет ответа, просто возвращаем ошибку как есть
            return thunkAPI.rejectWithValue(error);
        }
	}
)

