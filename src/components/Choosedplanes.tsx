import { ReactElement } from 'react'
import { useAppSelector } from '../store/store'
import ChoosedPlaneItem from './ChoosedPlaneItem';
import { AsValuesType } from '../store/slices/slice.wasm';
export default function Choosedplanes(): ReactElement {
	const choosedUniqueplains = useAppSelector(state => state.wasm.groupUniqueItems)
	function getMaxAsValueFromGroup(values: AsValuesType[]): AsValuesType {
		let maxAs1 = 0, maxAs2 = 0, maxAs3 = 0, maxAs4 = 0, maxAsw1 = 0, maxAsw2 = 0
		for (let i = 0; i < values.length; i++) {
			if (maxAs1 < values[i].as1) {
				maxAs1 = values[i].as1
			}
			if (maxAs2 < values[i].as2) {
				maxAs2 = values[i].as2
			}
			if (maxAs3 < values[i].as3) {
				maxAs3 = values[i].as3
			}
			if (maxAs4 < values[i].as4) {
				maxAs4 = values[i].as4
			}
			if (maxAsw1 < values[i].asw1) {
				maxAsw1 = values[i].asw1
			}
			if (maxAsw2 < values[i].asw2) {
				maxAsw2 = values[i].asw2
			}
		}
		return {
			as1: maxAs1,
			as2: maxAs2,
			as3: maxAs3,
			as4: maxAs4,
			asw1: maxAsw1,
			asw2: maxAsw2
		}
	}

	return (
		<>
			<table className=" divide-y divide-gray-200 bg-white max-h-min" style={{ maxWidth: "550px" }}>
				<thead className="bg-gray-50">
					<tr>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Название унификации</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Этаж</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Цвет</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Max as1</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Max as2</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Max as3</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Max as4</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Max asw1</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Max asw2</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Осн шаг</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Доп шаг</th>
					</tr>
				</thead>
				<tbody className="divide-y divide-gray-200">
				{choosedUniqueplains.map((cp, i) => (
					<ChoosedPlaneItem
						key={i}
						name={cp.name}
						color={cp.color}
						id={cp.id}
						planes={cp.planes}
						asValues={getMaxAsValueFromGroup(cp.maxAsValues)}
						steps={cp.steps}
					/>
			))}
				</tbody>
			</table>
			
		</>
	)
}
