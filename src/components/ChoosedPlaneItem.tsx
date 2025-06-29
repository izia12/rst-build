import { useState } from 'react';
import settingsIcon from '../assets/settings-svgrepo-com.svg';
import EditMode from './EditMode';
import Modal from './custom-components/Modal';
import { AsValuesType } from '../store/slices/slice.wasm';
import { StepArmType } from '../types/data.types';

type propsType={
	id:string,
	planes:number[],
	color:string,
	name:string
	asValues:AsValuesType,
	steps:StepArmType
}
export default function ChoosedPlaneItem({id, planes, color, name, asValues, steps}:propsType) {
	const [isOpen, setIsOpen] = useState(false);
	console.log(steps);
	
  return (
	<>
		<tr className="hover:bg-gray-50 even:bg-gray-50 transition-colors">
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{name || "Неизвестный"}</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{planes.join(", ")}</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">
			<div className='w-8 h-4' style={{background:color}}>
			</div>
			</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{asValues.as1}</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{asValues.as2}</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{asValues.as3}</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{asValues.as4}</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{asValues.asw1}</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{asValues.asw2}</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{steps?.mainStep}</td>
			<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{steps?.secondaryStep}</td>

			<td>
				<button
					onClick={()=>setIsOpen(!isOpen)}
				>
					<img 
						src={settingsIcon} alt="settings" 
						style={{ filter: 'invert(56%) sepia(74%) saturate(4591%) hue-rotate(191deg) brightness(99%) contrast(92%)' }}
						className='w-4 text-blue-300'
					/>
				</button>
			</td>
		</tr>
		<Modal isOpen={isOpen} onClose={()=>setIsOpen(false)}>
			<EditMode 
				id={id} 
				name={name} 
				color={color} 
				planes={planes} 
				asValues={asValues} 
				steps={steps}
				onClose={()=>setIsOpen(false)}
			/>
		</Modal>
	</>
  )
}
