import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { ArmDiameters, SpecifiedFitParamsType, WasmDataJsType, WASMDataType } from '../../types/data.types'
import { fetchArmDimeters, fetchWasmData, fetchWasmJSData } from './thunks/wasmThanks';

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
	armDiameters:ArmDiameters[],
	wasmJsData: WasmDataJsType,
	error: null | Error
	specifiedFitParams:SpecifiedFitParamsType[]
}

const initialState: WasmDataState = {
	wasmData: [],
	wasmJsData: {},
	choosedPlainsFromList:[],
	specifiedFitParams:[],
	loading: false,
	perfomance: {
		start: 0,
		end: 0,
	},
	armDiameters:[],
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
			state.choosedPlainsFromList.push(action.payload);
		},
		deleteChoosedItem:(state, action:PayloadAction<number>)=>{
			state.choosedPlainsFromList=state.choosedPlainsFromList.filter(el=>el!==action.payload);
		},
		setFitParamsItem:(state, action:PayloadAction<{area:number, price:number}>)=>{
			const item = state.specifiedFitParams.find(el=>el.area === action.payload.area);
			item.price = action.payload.price;
			item.isSpecified = true;
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
		builder.addCase(fetchArmDimeters.pending, (state) => {
			state.loading = true;
		})
		builder.addCase(fetchArmDimeters.fulfilled, (state, action) => {
			state.armDiameters = action.payload;
			state.specifiedFitParams = state.armDiameters.map(el=>({area:el.area, diameter:el.diameter, isSpecified:false, price:0}))
			state.loading = false
		})
		builder.addCase(fetchArmDimeters.rejected, (state, action) => {
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
	updateUniqueItem,
	setFitParamsItem
} = wasmSlice.actions

export default wasmSlice.reducer