import React from 'react'
import { Button } from './custom-components/Button'
import { useToast } from './custom-components/Toast'
import { useAppSelector } from '../store/store'


type propsType={

	openForCreateUI:boolean
	setOpenForCreateUI:(isOpen:boolean)=>void
}
export default function CreateNewGroupPlanesButton({ openForCreateUI, setOpenForCreateUI}:propsType) {
	const {showToast} = useToast()
	const items  = useAppSelector(state=>state.wasm.wasmJsData)
	const choosedItems  = useAppSelector(state=>state.wasm.choosedPlainsFromList)
	const checkAllStepsAreEqual = ()=>{
		const choosedPlains = choosedItems.map(el=>el.plainNumber)
		const res =  Object.entries(items)
					.filter(([, val])=>val.isSelected)
					.filter(([el]) => {
						const choosedAllPlains = choosedPlains
						return choosedAllPlains.includes(+el);
					})
		const mainSteps = res.map(([, el])=>el.mainStep);
		const secondarySteps = res.map(([, el])=>el.secondaryStep);
		return new Set(mainSteps).size===1 && new Set(secondarySteps).size===1
	}
	return (
		<Button
			onClick={() => {
				if (!checkAllStepsAreEqual()) {
					showToast("Шаги выбранных этажей должны быть одинаковы",'error');
					return
				}
				setOpenForCreateUI(!openForCreateUI)
			}}
			classes="absolute bottom-12 right-4 p-2 bg-blue-500 rounded-md shadow-lg hover:shadow-none transition-shadow"
		/>
	)
}
