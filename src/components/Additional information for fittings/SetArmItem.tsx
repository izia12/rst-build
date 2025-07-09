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
		.filter(el=>el.isDefault===false&&el.isSpecified===false)
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
		<div className=" flex items-center hover:bg-gray-50 even:bg-gray-50 transition-colors gap-2">
			<div className="px-4 py-2 text-gray-700 border-b border-gray-200 w-80">
				<Select
					options={armDiameters}
					onSelect={setSelectedItem}
					className=''
				/>
			</div>
			<div>
				<Input
					onChange={(e)=>setPrice(+e.currentTarget.value)}
					type={"number"}
				/>
			</div>
			<div className='w-full'>
				<Button
					classes='px-4 py-2 block'
					onClick={handleSetPrice}
					buttonName='Задать цену'
				/>
			</div>
		</div>
	)
}
