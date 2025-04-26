import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { WasmDataJsType, WASMDataType } from '../../types/data.types'
import { fetchWasmData, fetchWasmJSData } from './thunks/wasmThanks';

type UniqeItem = {
	id: string,
	planes: number[],
	name: string,
	color:string
}

export interface WasmDataState {
	wasmData: Array<WASMDataType>
	loading: boolean,
	choosedPlainsFromList:number[],
	groupUniqueItems: UniqeItem[],
	perfomance: {
		start: number,
		end: number,
	},
	wasmJsData: WasmDataJsType,
	error: null | Error
}

const initialState: WasmDataState = {
	wasmData: [],
	wasmJsData: {},
	choosedPlainsFromList:[],
	loading: false,
	perfomance: {
		start: 0,
		end: 0,
	},
	groupUniqueItems: [],
	error: null
}

export const wasmSlice = createSlice({
	name: 'counter',
	initialState,
	reducers: {
		startPerfomance: (state, action: PayloadAction<number>) => {
			state.perfomance.start = action.payload
		},
		endPerfomance: (state, action: PayloadAction<number>) => {
			state.perfomance.end = action.payload;
		},
		addToGroupUniqueItem:(state,action:PayloadAction<UniqeItem>)=>{
			state.groupUniqueItems.push(action.payload);
			state.choosedPlainsFromList.length=0
		},
		updateUniqueItem:(state, action:PayloadAction<{id:string, newUniqeItem:UniqeItem}>)=>{
			const uniqueItem = state.groupUniqueItems.find(ui=>ui.id === action.payload.id)
			const newUniqeItem = action.payload.newUniqeItem
			if(uniqueItem){
				uniqueItem.color = newUniqeItem.color
				uniqueItem.name =newUniqeItem.name
				uniqueItem.planes = newUniqeItem.planes
			}
		},

		deleteGroupUniqueItem:(state, action:PayloadAction<string>)=>{
			state.groupUniqueItems=state.groupUniqueItems.filter(i=>i.id!==action.payload)
		},
		addChosedItems:(state, action:PayloadAction<number>)=>{
			state.choosedPlainsFromList.push(action.payload)
		},
		deleteChoosedItem:(state, action:PayloadAction<number>)=>{
			state.choosedPlainsFromList=state.choosedPlainsFromList.filter(el=>el!==action.payload)
		}
	},
	extraReducers: (builder) => {
		builder.addCase(fetchWasmData.fulfilled, (state, action) => {
			state.wasmData = action.payload
			state.loading = false
		})
		builder.addCase(fetchWasmData.pending, (state) => {
			state.loading = true
		})
		builder.addCase(fetchWasmData.rejected, (state, action) => {
			state.error = action.error as Error
			state.loading = false
		})
		builder.addCase(fetchWasmJSData.pending, (state) => {
			state.loading = true
		})
		builder.addCase(fetchWasmJSData.fulfilled, (state, action) => {
			state.wasmJsData = action.payload
			state.loading = false
		})
		builder.addCase(fetchWasmJSData.rejected, (state, action) => {
			state.error = action.error as Error
			state.loading = false
		})
	}
})

// Action creators are generated for each case reducer function
export const {
	startPerfomance,
	endPerfomance,
	addChosedItems,
	deleteChoosedItem,
	addToGroupUniqueItem,
	deleteGroupUniqueItem,
	updateUniqueItem
} = wasmSlice.actions

export default wasmSlice.reducer