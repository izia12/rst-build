import { ReactElement, useState } from "react";
import { Checkbox } from "./custom-components/Checkbox";
import LinkButton from "./custom-components/LinkButton";
import { Select } from "./custom-components/Select";
import { useToast } from "./custom-components/Toast";


type propsType ={
	platesLength:number,
	rodsLength:number,
	checkboxId:string,
	materials:number[],
	maxAs1:number,
	maxAs2:number,
	maxAs3:number,
	maxAs4:number,
}
export const UniqeItem = ({checkboxId, platesLength, rodsLength, materials, maxAs1, maxAs2, maxAs3, maxAs4}:propsType):ReactElement=>{

	const [mainStep, setMainStep] = useState(20)
	const [secondaryStep, setSecondaryStep] = useState(20)
	const [settingMainStep, setSettingMainStep] = useState<"setting" | "wasSet" | "notSet">("notSet");
	const [settingSecStep, setSettingSecMainStep] = useState<"setting" | "wasSet" | "notSet">("notSet");
	const { showToast } = useToast();

	function handleKeyDown(e:React.KeyboardEvent<HTMLInputElement>, stepKind:"main"|"secondary"){
		if(e.key==="Enter" && stepKind==="main"){
			setMainStep(+e.currentTarget.value);
			setSettingMainStep("wasSet")
		}
		if(e.key==="Enter" && stepKind==="secondary"){
			setSecondaryStep(+e.currentTarget.value);
			setSettingSecMainStep("wasSet")
		}
	}
	return(
		<tr 
			className="hover:bg-gray-50 even:bg-gray-50 transition-colors"
		>
			<td className="whitespace-nowrap px-4 py-2 border-b border-gray-200">
				<Checkbox id={checkboxId} maxValues={
					{as1:maxAs1, as2:maxAs2, as3:maxAs3, as4:maxAs4}
				} />
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
			<td className="px-4 text-center w-16">
				{
				settingMainStep=="notSet" ?
					<button className="text-blue-600 underline"
					onClick={()=>{
						showToast('Успешно сохранено!', 'success')
						console.log(settingMainStep)
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
					/>
					:mainStep
			}

			</td>
			<td className="px-4 text-center w-16">
				{
				settingSecStep=="notSet" ?
					<button className="text-blue-600 underline"
					onClick={()=>{
						console.log(settingSecStep)
						setSettingSecMainStep("setting")
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
					/>
					:secondaryStep
			}

			</td>
		</tr>
	)
}