import  { ReactElement } from 'react'
import { useAppSelector } from '../store/store'
import settingsIcon from '../assets/settings-svgrepo-com.svg';
export default function Choosedplanes():ReactElement {
	const choosedUniqueplains = useAppSelector(state=>state.wasm.groupUniqueItems)
	// const choosedPlains = useAppSelector(state=>state.wasm.choosedItems);
  return (
	<>
		{choosedUniqueplains.map((cp,i)=>(
			<tr className="hover:bg-gray-50 even:bg-gray-50 transition-colors" key={i}>
				<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{cp.name || "Неизвестный"}</td>
				<td className="px-4 py-2 text-gray-700 border-b border-gray-200">{cp.planes.toString()}</td>
				<td className="px-4 py-2 text-gray-700 border-b border-gray-200">
					<div className='w-8 h-4' style={{background:cp.color}}>
					</div>
				</td>
				<td>
					<button>
						<img 
							src={settingsIcon} alt="settings" 
							style={{ filter: 'invert(56%) sepia(74%) saturate(4591%) hue-rotate(191deg) brightness(99%) contrast(92%)' }}
							className='w-4 text-blue-300'
						/>
					</button>
				</td>
			</tr>
		))}
	</>
  )
}
