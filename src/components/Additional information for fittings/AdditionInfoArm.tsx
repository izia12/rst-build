import { ReactElement } from "react";
import { useAppSelector } from "../../store/store";
import SetArmItem from "./SetArmItem";
import { SpecifiedArmItem } from "./SpecifiedArmItem";


export const AdditionInfoArm = (): ReactElement => {
	const diameters = useAppSelector(state=>state.wasm.specifiedFitParams);
	// const armDiametrs = diameters.map(el=>({value:el.area.toString(), label:el.diameter.toString()}))
	
	return (
		<>
		<SetArmItem/>
			<table className=" divide-y divide-gray-200 bg-white " style={{ maxWidth: "550px" }}>
				<thead className="bg-gray-50">
					<tr>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Диаметр</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Цена</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Цена</th>
					</tr>
				</thead>
				<tbody>
				{diameters.filter(el=>el.default||el.isSpecified).map((el)=>(
					<SpecifiedArmItem key={el.area} {...el}/>
				))}
				</tbody>
			</table>
		</>
	)
}