import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { WASMDataType } from '../../types/data.types'
import { fetchWasmData } from './thunks/wasmThanks';

type UniqItem={
	id:string,
	planes:number[],
	name:string,

}
export interface WasmDataState {
	wasmData: Array<WASMDataType>
	loading:boolean,
	perfomance:{
		start:number,
		end:number,
	},
	unificationItems:UniqItem[],
	error: null | Error
}

const initialState: WasmDataState = {
  wasmData: [],
  loading:false,
  perfomance:{
	start:0,
	end:0,
},
unificationItems:[],
  error: null 
}

export const wasmSlice = createSlice({
  name: 'counter',
  initialState,
  reducers: {
	startPerfomance:(state, action:PayloadAction<number>)=>{
		state.perfomance.start = action.payload
	},
	endPerfomance:(state, action:PayloadAction<number>)=>{
		state.perfomance.end = action.payload;
	}
  },
  extraReducers:(builder)=>{
	builder.addCase(fetchWasmData.fulfilled, (state, action)=>{
		state.wasmData = action.payload
	})
	builder.addCase(fetchWasmData.pending, (state)=>{
		state.loading = true
	})
	builder.addCase(fetchWasmData.rejected, (state, action)=>{
		state.error = action.error as Error
	})
  }
})

// Action creators are generated for each case reducer function
export const { startPerfomance, endPerfomance } = wasmSlice.actions

export default wasmSlice.reducer