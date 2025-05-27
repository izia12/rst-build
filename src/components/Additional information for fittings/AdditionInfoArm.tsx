import { ReactElement } from "react";
import { Select } from "../custom-components/Select";
import { useAppSelector } from "../../store/store";
import SetArmItem from "./SetArmItem";

export const AdditionInfoArm = (): ReactElement => {
	const diameters = useAppSelector(state=>state.wasm.specifiedFitParams);
	// const armDiametrs = diameters.map(el=>({value:el.area.toString(), label:el.diameter.toString()}))
	return (
		<>
			<table className=" divide-y divide-gray-200 bg-white " style={{ maxWidth: "550px" }}>
				<thead className="bg-gray-50">
					<tr>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Диаметр</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Цена</th>
					</tr>
				</thead>
				<tbody>
				
					<SetArmItem/>
				{diameters.filter(el=>el.isSpecified).map((el)=>(
					<tr key = {el.diameter}>
						<td className="px-4 py-2 text-gray-700 border-b border-gray-200">
							{el.diameter}мм
						</td>
						<td className="px-4 py-2 text-gray-700 border-b border-gray-200">
							{el.price} сом/тон
						</td>
					</tr>
				))}
				</tbody>
			</table>
		</>
	)
}