import { ReactElement } from "react";
import { Checkbox } from "./custom-components/Checkbox";

type propsType ={
	platesLength:number,
	rodsLength:number,
	checkboxId:string
}
export const UniqeItem = ({checkboxId, platesLength, rodsLength}:propsType):ReactElement=>{
	
	return(
		<tr 
			className="hover:bg-gray-50 even:bg-gray-50 transition-colors"
		>
			<td className="whitespace-nowrap px-4 py-2 border-b border-gray-200">
				<Checkbox id={checkboxId} />
			</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{checkboxId}</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">
				{platesLength || 0}
			</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">
				{rodsLength || 0}
			</td>
		</tr>
	)
}