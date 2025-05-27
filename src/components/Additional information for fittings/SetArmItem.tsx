import { ReactElement, useState } from 'react'
import { Select } from '../custom-components/Select'
import { useAppDispatch, useAppSelector } from '../../store/store';
import { Input } from '../custom-components/Input';
import { Button } from '../custom-components/Button';
import { setFitParamsItem } from '../../store/slices/slice.wasm';

export default function SetArmItem(): ReactElement {
	const dispatch = useAppDispatch();
	const diameters = useAppSelector(state=>state.wasm.specifiedFitParams);
	const armDiameters = diameters
		.filter(el=>el.isSpecified===false)
		.map(el=>({value:el.area.toString(), label:el.diameter.toString()}));
	const [selectedItem, setSelectedItem] = useState<null| number>(null);
	const [price, setPrice] = useState(0);
	function handleSetPrice(){
		dispatch(setFitParamsItem({
			area: +selectedItem,
			price: price,
		}
		))
	}
	return (
		<tr className="hover:bg-gray-50 even:bg-gray-50 transition-colors">
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">
				<Select
					options={armDiameters}
					onSelect={setSelectedItem}
				/>
			</td>
			<td>
				<Input
					onChange={(e)=>setPrice(+e.target.value)}
					type={"number"}
				/>
			</td>
			<td>
				<Button
					onClick={handleSetPrice}
					buttonName='Задать цену'
				/>
			</td>
		</tr>
	)
}
