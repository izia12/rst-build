import React from 'react'
import { Button } from './custom-components/Button'
import { useToast } from './custom-components/Toast'
import { useAppSelector } from '../store/store'
import { getPreparedUniqueFloor } from '../helpers/getPreparedUniqueFloors'
import { getPreparedDataFromUniqueGroups } from '../helpers/getPreparedDataFromUniqueGroups'
import init,{ create_custom_sortament_report } from '../assets/pkg/rst_build'
import { fetchExcelViewData } from '../store/slices/thunks/wasmThanks'
import { useAppDispatch } from './../store/store';


type propsType={
	openForCreateUI:boolean
	setOpenForCreateUI:(isOpen:boolean)=>void
}
export default function CreateNewGroupPlanesButton({ openForCreateUI, setOpenForCreateUI}:propsType) {
	const {showToast} = useToast();
	const dispatch = useAppDispatch()
	const floors1 = useAppSelector(state=>state.wasm.specifiedFitParams)
	const items = useAppSelector(state=>state.wasm.wasmJsData)
	const choosedItems  = useAppSelector(state=>state.wasm.choosedPlainsFromList)
	const choosedUniques = useAppSelector(state=>state.wasm.groupUniqueItems)
	const checkAllStepsAreEqual = ()=>{
		const choosedPlains = choosedItems.map(el=>el.plainNumber)
		const res =  Object.entries(items)
					.filter(([el]) => {
						const choosedAllPlains = choosedPlains
						return choosedAllPlains.includes(+el);
					})
		const mainSteps = res.map(([, el])=>el.mainStep);
		const secondarySteps = res.map(([, el])=>el.secondaryStep);
		return new Set(mainSteps).size===1 && new Set(secondarySteps).size===1
	}
	const floors = choosedUniques.map(el=>el.planes).flat();
	const entries = Object.entries(items);
	
	const filteredItems = entries.filter(([key])=>{
		return !floors.includes(+key)
	})
	
	const preparedItems = getPreparedUniqueFloor(filteredItems)
	const preparedUniqes = getPreparedDataFromUniqueGroups(choosedUniques)
	const fiteredArms = floors1.filter(el=>el.isDefault);
	const summaryData = {
		availableDiameters: new Uint32Array(fiteredArms.map(el=>el.diameter)),
		jsonArray:JSON.stringify([...preparedItems, ...preparedUniqes])
	}
		async function saveXlsx() {
		try{
			await init()
			setTimeout(async()=>{
				 dispatch(
					await fetchExcelViewData({diameters:summaryData.availableDiameters, floorsJson:summaryData.jsonArray})
				)
				console.log(summaryData);
			},2000)
			
			const data = await create_custom_sortament_report(summaryData.availableDiameters, summaryData.jsonArray	);
			const combinedData = new Uint8Array(data);
			saveFile(combinedData)

		}catch(e){
			console.log(e);
		}
	}
	// const fn = get_custom_sortament_report();
	const saveFile = (data: Uint8Array,) => {
		const blob = new Blob([data], { type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' });
		const url = URL.createObjectURL(blob);
		const link = document.createElement('a');
		link.href = url;
		link.download = "out.xlsx";
		document.body.appendChild(link);
		link.click();
		document.body.removeChild(link);
		URL.revokeObjectURL(url);
	}
	return (
		<div className='flex gap-2 absolute bottom-12 right-4'>
			<Button
			// disabled = {!checkAllStepsAreEqual()}
			buttonName='Получить Excel'
				onClick={async() => {
					if (!checkAllStepsAreEqual()) {
						// showToast("Шаги выбранных этажей должны быть одинаковы",'error');
						await saveXlsx()
						return
					}
					setOpenForCreateUI(!openForCreateUI)
				}}
				classes=" p-2 bg-blue-500 rounded-md shadow-lg hover:shadow-none transition-shadow"
			/>
			<Button
				onClick={() => {
					if (!checkAllStepsAreEqual()) {
						showToast("Шаги выбранных этажей должны быть одинаковы",'error');
						return
					}
					setOpenForCreateUI(!openForCreateUI)
				}}
				buttonName='Создать унификацию'
				classes=" p-2 bg-blue-500 rounded-md shadow-lg hover:shadow-none transition-shadow"
			/>
		</div>
	)
}
