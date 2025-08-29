import {  ReactElement, useState } from "react";
import { SpecifiedFitParamsType } from "../../types/data.types";
import { useAppDispatch } from "../../store/store";
import { setDefaultForArmItem, setPriceForArmItem } from "../../store/slices/slice.wasm";


export const SpecifiedArmItem = ({area, diameter, price, isDefault}:SpecifiedFitParamsType): ReactElement => {
	const [priceArmItem, setpriceArmItem] = useState<"setting"|"wasSet"|"notSet">("notSet");
	const dispatch = useAppDispatch();

	return (
		<tr key={diameter}>
			<td> 
				<input 
					type="checkbox" 
					className="rounded w-4 h-4" 
					name="" id="" 
					checked={isDefault} 
					onClick={()=>dispatch(setDefaultForArmItem(area))}
				/>
			</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">
				{diameter}мм
			</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">
				{price} сом/тон
			</td>
			{(price === null && priceArmItem==="notSet")&&
				<td>
					<button onClick={()=>{
						setpriceArmItem("setting")
						
					}}>
						Задать цену?
					</button>
				</td>
			}
			{
				priceArmItem==="setting" ?
				<td>
					<input type="number" 
						className="w-full px-4 py-2 border rounded-lg transition-all focus:outline-none focus:ring-2 "
						onChange={(e)=>{
							dispatch(setPriceForArmItem({
								area,
								price:+e.currentTarget.value
							}))
						}}
						onKeyDown={(e)=>{
							if(e.key==="Enter"){
								setpriceArmItem("wasSet")
							}
						}}
						onBlur={()=>{
							setpriceArmItem("notSet")
						}}
						autoFocus
					/>
				</td>
				:
				priceArmItem==="wasSet" ?
				<td>{price}</td>
				:
				<td><div></div></td>
			}
		</tr>
	)
}