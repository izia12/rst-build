import  { ReactElement } from 'react'
import { useAppSelector } from '../store/store'
import ChoosedPlaneItem from './ChoosedPlaneItem';
import { AsValuesType } from '../store/slices/slice.wasm';
export default function Choosedplanes():ReactElement {
	const choosedUniqueplains = useAppSelector(state=>state.wasm.groupUniqueItems)
	// const choosedPlains = useAppSelector(state=>state.wasm.choosedItems);
	function getMaxAsValueFromGroup(values:AsValuesType[]):AsValuesType{
		let maxAs1=0, maxAs2=0, maxAs3=0, maxAs4=0
		for (let i =0; i<values.length; i++){
			if(maxAs1<values[i].as1){
				maxAs1=values[i].as1
			}
			if(maxAs2<values[i].as2){
				maxAs2=values[i].as2
			}
			if(maxAs3<values[i].as3){
				maxAs3=values[i].as3
			}
			if(maxAs4<values[i].as4){
				maxAs4=values[i].as4
			}
		}
		return {
			as1:maxAs1,
			as2:maxAs2,
			as3:maxAs3,
			as4:maxAs4,
		}
	}
	
  return (
	<>
		{choosedUniqueplains.map((cp,i)=>(
			<ChoosedPlaneItem
				key={i}
				name={cp.name}
				color={cp.color}
				id={cp.id}
				planes={cp.planes}
				asValues={getMaxAsValueFromGroup(cp.maxAsValues)}
			/>
		))}
	</>
  )
}
