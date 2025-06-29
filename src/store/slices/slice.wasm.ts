import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { ArmDiameters, MaxAsFn, PureWASMJsData, SpecifiedFitParamsType, StepArmType, WASMDataType } from '../../types/data.types'
import {  fetchWasmData, fetchWasmJSData } from './thunks/wasmThanks';
import { stepVariantsToState } from '../../constants/stepConstants';


type UniqeItem = {
	id: string,
	planes: number[],
	name: string,  
	color:string, 
	maxAsValues:AsValuesType[],
	steps:StepArmType
}

export type AsValuesType={
	as1:number,
	as2:number,
	as3:number,
	as4:number,
	asw1:number,
	asw2:number,
}
export type PlainItemtype = {
	plainNumber:number,
	asValues:AsValuesType,
	steps:StepArmType
}
export interface WasmDataState {
	wasmData: Array<WASMDataType>
	loading: boolean,
	choosedPlainsFromList:PlainItemtype[],
	groupUniqueItems: UniqeItem[],
	perfomance: {
		start: number,
		end: number,
	},
	armDiameters:ArmDiameters[],
	wasmJsData: PureWASMJsData,
	error: null | Error
	specifiedFitParams:SpecifiedFitParamsType[],
	maxAsFns:MaxAsFn[]
}

const initialState: WasmDataState = {
	wasmData: [],
	wasmJsData: {},
	choosedPlainsFromList:[],
	specifiedFitParams:stepVariantsToState,
	loading: false,
	perfomance: {
		start: 0,
		end: 0,
	},
	maxAsFns:[],
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
		addChosedItems:(state, action:PayloadAction<PlainItemtype>)=>{
			state.choosedPlainsFromList.push(action.payload);
		},
		deleteChoosedItem:(state, action:PayloadAction<PlainItemtype>)=>{
			state.choosedPlainsFromList=state.choosedPlainsFromList.filter(el=>el!==action.payload);
		},
		setFitParamsItem:(state, action:PayloadAction<{area:number, price:number}>)=>{
			const item = state.specifiedFitParams.find(el=>el.area === action.payload.area);
			console.log(item);
			
			item.price = action.payload.price;
			item.isSpecified = true;
		},
		setArmStep:(state, action:PayloadAction<{plane:string, secondaryStep:number, mainStep:number}>)=>{
			const foundUniqueItem = state.wasmJsData[action.payload.plane];
			if(foundUniqueItem){
				foundUniqueItem.mainStep = action.payload.mainStep;
				foundUniqueItem.secondaryStep = action.payload.secondaryStep;
				foundUniqueItem.isSelected=true
			}
			// state.wasmJsData ={
			// 	...state.wasmJsData,
			// 	foundUniqueItem
			// }
		},
		selectToUniqueGroup:(state, action:PayloadAction<string>)=>{
			state.wasmJsData[action.payload].isSelected=true
		},
		unSelectToUniqueGroup:(state, action:PayloadAction<string>)=>{
			state.wasmJsData[action.payload].isSelected=false
		},
		setPriceForArmItem:(state, action:PayloadAction<{area:number, price:number}>)=>{
			const found = state.specifiedFitParams.find(el=>el.area===action.payload.area);
			found.price = action.payload.price
			found.isSpecified=true
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
			state.error = action.error as Error;
			state.loading = false;
		})
		// builder.addCase(fetchArmDimeters.pending, (state) => {
		// 	state.loading = true;
		// })
		// builder.addCase(fetchArmDimeters.fulfilled, (state, action) => {
		// 	state.armDiameters = action.payload;
		// 	state.specifiedFitParams = state.armDiameters.map(el=>({area:el.area, diameter:el.diameter, isSpecified:false, price:0}))
		// 	state.loading = false;
		// })
		// builder.addCase(fetchArmDimeters.rejected, (state, action) => {
		// 	state.error = action.error as Error;
		// 	state.loading = false;
		// })
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
	setFitParamsItem,
	setArmStep,
	selectToUniqueGroup,
	unSelectToUniqueGroup,
	setPriceForArmItem
} = wasmSlice.actions

export default wasmSlice.reducer