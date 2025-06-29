import { ReactElement, useState } from "react";
import { Checkbox } from "./custom-components/Checkbox";
import { useToast } from "./custom-components/Toast";
import { useAppDispatch } from "../store/store";
import { setArmStep } from "../store/slices/slice.wasm";
import setting from "../assets/edit-button-svgrepo-com.svg"

type propsType ={
	platesLength:number,
	rodsLength:number,
	checkboxId:string,
	materials:number[],
	maxAs1:number,
	maxAs2:number,
	maxAs3:number,
	maxAs4:number,
	maxAsw1:number,
	maxAsw2:number,
	parentMainStep:number
	parentSecondaryStep:number
}
export const UniqeItem = (
	{
		checkboxId, 
		platesLength, 
		rodsLength, 
		materials, 
		maxAs1, 
		maxAs2, 
		maxAs3, 
		maxAs4, 
		maxAsw1, 
		maxAsw2,
		parentMainStep,
		parentSecondaryStep
	}:propsType):ReactElement=>{
	const dispatch = useAppDispatch();
	const [mainStep, setMainStep] = useState(parentMainStep===null?200:parentMainStep)
	const [secondaryStep, setSecondaryStep] = useState(parentSecondaryStep===null?200:parentSecondaryStep)
	const [settingMainStep, setSettingMainStep] = useState<"setting" | "wasSet" | "notSet">("notSet");
	const [settingSecStep, setSettingSecStep] = useState<"setting" | "wasSet" | "notSet">("notSet");
	const { showToast } = useToast();

	function handleKeyDown(e:React.KeyboardEvent<HTMLInputElement>, stepKind:"main"|"secondary"){
		if(e.key==="Enter" && stepKind==="main"){
			setMainStep(+e.currentTarget.value);
			setSettingMainStep("wasSet")
			if(settingSecStep==="wasSet"){
				onSetDataMainStep()
			}
		}
		if(e.key==="Enter" && stepKind==="secondary"){
			setSecondaryStep(+e.currentTarget.value);
			setSettingSecStep("wasSet")
			if(settingMainStep==="wasSet"){
				onSetDataMainStep()
			}
		}
	}
	// console.log(parentMainStep, parentSecondaryStep);
	
	function onSetDataMainStep(){
		dispatch(setArmStep({plane:checkboxId, mainStep, secondaryStep}))
	}
	
	return(
		<tr 
			className="hover:bg-gray-50 even:bg-gray-50 transition-colors"
		>
			<td className="whitespace-nowrap px-4 py-2 border-b border-gray-200">
				<Checkbox 
					id={checkboxId} 
					maxValues={{as1:maxAs1, as2:maxAs2, as3:maxAs3, as4:maxAs4, asw1:maxAsw1, asw2:maxAsw2 }}
					currentStep={{mainStep:parentMainStep, secondaryStep:parentSecondaryStep}}
				/>
			</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{checkboxId}</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">
				{platesLength || 0}
			</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">
				{rodsLength || 0}
			</td>
			<td className="px-4">
				{materials.toString()||0}
			</td>
			<td className="px-4">
				{maxAs1}
			</td>
			<td className="px-4">
				{maxAs2}
			</td>
			<td className="px-4">
				{maxAs3}
			</td>
			<td className="px-4">
				{maxAs4}
			</td>
			<td className="px-4">
				{maxAsw1}
			</td>
			<td className="px-4">
				{maxAsw2}
			</td>
			<td className="px-4 text-center w-16">
				{
				settingMainStep=="notSet" && parentMainStep===null ?
					<button className="text-blue-600 underline"
					onClick={()=>{
						showToast('Успешно сохранено!', 'success')
						setSettingMainStep("setting")
					}}
					>
						добавить осн. шаг
					</button>
					:
					settingMainStep=="setting"? 
					<input type="number" className="w-16 border text-center px-1 py-0.5" 
						value={mainStep}
						onChange={(e)=>setMainStep(+e.currentTarget.value)}
						onKeyDown={(e)=>handleKeyDown(e, "main")}
						onBlur={()=>{
							setSettingMainStep("wasSet")
							if(settingSecStep==="wasSet"){
								onSetDataMainStep()
							}
						}}
					/>
					:mainStep
			}
			{(settingMainStep=="wasSet"||parentMainStep!==null) && <button
				onClick={()=>setSettingMainStep("setting")}
			>
				<img 
					src={setting} 
					alt="" 
					style={{ filter: 'invert(56%) sepia(74%) saturate(4591%) hue-rotate(191deg) brightness(99%) contrast(92%)' }}
					className='w-4 ml-2 text-blue-300'
				/>
			</button>}
			</td>
			<td className="px-4 text-center w-16">
				{
				settingSecStep=="notSet" &&parentSecondaryStep===null ?
					<button className="text-blue-600 underline"
					onClick={()=>{
						// console.log(settingSecStep)
						setSettingSecStep("setting")
					}}
					>
						добавить доп шаг
					</button>
					:
					settingSecStep=="setting"? 
					<input type="number" className="w-16 border text-center px-1 py-0.5" 
						value={secondaryStep}
						onChange={(e)=>setSecondaryStep(+e.currentTarget.value)}
						onKeyDown={(e)=>handleKeyDown(e, "secondary")}
						onBlur={()=>{
							setSettingSecStep("wasSet")
							if(settingMainStep==="wasSet"){
								onSetDataMainStep()
							}
						}}
					/>
					:secondaryStep
			}
			{(settingSecStep === "wasSet" || parentSecondaryStep!==null) && 
			<button
				onClick={()=>setSettingSecStep("setting")}
			>
				<img 
					src={setting} 
					alt="" 
					style={{ filter: 'invert(56%) sepia(74%) saturate(4591%) hue-rotate(191deg) brightness(99%) contrast(92%)' }}
					className='w-4 ml-2 text-blue-300'
				/>
			</button>
			}
			</td>
		</tr>
	)
}