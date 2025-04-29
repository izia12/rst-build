import  { ReactElement } from 'react'
import { useAppSelector } from '../store/store'
import ChoosedPlaneItem from './ChoosedPlaneItem';
export default function Choosedplanes():ReactElement {
	const choosedUniqueplains = useAppSelector(state=>state.wasm.groupUniqueItems)
	// const choosedPlains = useAppSelector(state=>state.wasm.choosedItems);
  return (
	<>
		{choosedUniqueplains.map((cp,i)=>(
			<ChoosedPlaneItem
				key={i}
				name={cp.name}
				color={cp.color}
				id={cp.id}
				planes={cp.planes}
			/>
		))}
	</>
  )
}
