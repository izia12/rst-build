import { createAsyncThunk } from "@reduxjs/toolkit"

import { ArmDiameters, PreparedExcelView, PureWASMJsData,  WASMDataType } from "../../../types/data.types"
import init, { convert_sli_xsl_to_json_string, get_horizontal_elements_object_js, get_sortament_data, get_table_data_for_frontend, parse_data, create_docx_for_selected_combinations, get_entities_json, create_docx_with_selected_combinations } from "../../../assets/pkg/rst_build"
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
			
            return result as ArmDiameters[];
        } catch (error) {
            return thunkAPI.rejectWithValue(error instanceof Error ? error.message : 'Unknown error');
        }
    }
);
export const fetchExcelViewData = createAsyncThunk<PreparedExcelView[], {diameters:Uint32Array, floorsJson:string}>(
	'data/excelViewData',
	async ({diameters, floorsJson}, thunkAPI) => {
        try {
            await init();
            // Передаем параметры в WASM функцию
            const result = get_table_data_for_frontend(diameters, floorsJson);
            if (!result) throw new Error('WASM data not ready');
            return JSON.parse(result) as PreparedExcelView[];
        } catch (error) {
            return thunkAPI.rejectWithValue(error instanceof Error ? error.message : 'Unknown error');
        }
    }
);

// НОВАЯ ФУНКЦИЯ: Генерация документа с цветовой палитрой
export const generateDocumentWithColorPalette = createAsyncThunk<Uint8Array, import('../../../types/data.types').SelectedCombinationsData>(
	'data/generateDocumentWithColors',
	async (selectedData, thunkAPI) => {
        try {
            await init();
            
            console.log('🎨 Generating document with color palette for:', selectedData);
            
            // Конвертируем данные в JSON строку
            const selectedCombinationsJson = JSON.stringify(selectedData);
            
            // Вызываем новую WASM функцию для генерации с цветовой палитрой
            const result = await create_docx_with_selected_combinations(selectedCombinationsJson);
            
            if (!result) throw new Error('Failed to generate document with color palette');
            return new Uint8Array(result);
        } catch (error) {
            console.error('❌ Color palette generation failed:', error);
            return thunkAPI.rejectWithValue(error instanceof Error ? error.message : 'Unknown error');
        }
    }
);

// СТАРАЯ ФУНКЦИЯ: Генерация документа по этажам
export const generateDocumentForSelectedCombinations = createAsyncThunk<Uint8Array, string[]>(
	'data/generateDocument',
	async (selectedFloors, thunkAPI) => {
        try {
            await init();
            // Конвертируем массив этажей в JSON строку
            const floorsJson = JSON.stringify(selectedFloors.map(floor => parseFloat(floor)));
            // Вызываем WASM функцию для генерации документа
            const result = create_docx_for_selected_combinations(floorsJson);
            if (!result) throw new Error('Failed to generate document');
            return result;
        } catch (error) {
            return thunkAPI.rejectWithValue(error instanceof Error ? error.message : 'Unknown error');
        }
    }
);

