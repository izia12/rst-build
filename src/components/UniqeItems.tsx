import { ReactElement, useState } from "react";
import { useAppSelector } from "../store/store";
import { UniqeItem } from "./UniqeItem";
import Choosedplanes from "./Choosedplanes";
import CreateUniqueItem from "./CreateUniqueItem";
import toBig from "../assets/sort-numeric-alt-down-svgrepo-com.svg"
import toSmall from "../assets/sort-numeric-up-svgrepo-com.svg"
type propsType={
	setOpenForCreateUI:(val:boolean)=>void
	openForCreateUI:boolean
}
export const UniqeItems = ({openForCreateUI, setOpenForCreateUI}:propsType): ReactElement => {
	const pending = useAppSelector(state => state.wasm.loading);
	const wasmJsData = useAppSelector(state => state.wasm.wasmJsData);
	const selectedPlainsToUnification = useAppSelector(state => state.wasm.groupUniqueItems)
	// const [openForCreateUI, setOpenForCreateUI] = useState(false);
	const [sortToBig, setSortToBig] = useState(false)
	return (
		<div className="p-4 relative">
			<div className=" flex justify-between">
				{pending && <div className="text-gray-500 mb-4">Loading...</div>}
				<div className=" overflow-x-auto rounded-lg border  border-gray-200 justify-between" >
					<table className=" divide-y divide-gray-200 bg-white " style={{ maxWidth: "550px" }}>
						<thead className="bg-gray-50">
							<tr>
								<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">
									<button 
										onClick={()=>setSortToBig(!sortToBig)} 
										className="bg-emerald-400"
										
										>
										<img src={sortToBig?toBig:toSmall} alt="" className="w-8"
											style={{ filter: 'invert(56%) sepia(74%) saturate(4591%) hue-rotate(191deg) brightness(99%) contrast(92%)' }}
										/>
									</button>
								</th>
								<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Z Name</th>
								<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Plate Elements</th>
								<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Rod Elements</th>
							</tr>
						</thead>
						<tbody className="divide-y divide-gray-200">
							{
								Object.entries(wasmJsData || {})
									?.filter(([el]) => {
										const choosedAllPlains = selectedPlainsToUnification.map(el => el.planes).flat()
										return !choosedAllPlains.includes(+el);
									})
									.sort(([el,],[el2])=>{
										if(sortToBig){
											if((+el>+el2) )return 1
											else return -1
										}else{
											if((+el>+el2))return -1
											else return 1
										}
									})
									?.map(([key, value]) => (
										<UniqeItem
											key={key}
											checkboxId={key}
											platesLength={value?.plates?.length}
											rodsLength={value?.rods?.length}
										/>
									))}
						</tbody>
					</table>

					<CreateUniqueItem
						onClose={()=>{
							return setOpenForCreateUI(false)
						}}
						isOpen={openForCreateUI}
					/>
				</div>
				<div>
					<table className=" divide-y divide-gray-200 bg-white max-h-min" style={{ maxWidth: "550px" }}>
						<thead className="bg-gray-50">
							<tr>
								<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Название унификации</th>
								<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Этаж</th>
								<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Цвет</th>
							</tr>
						</thead>
						<tbody className="divide-y divide-gray-200">
							<Choosedplanes />
						</tbody>
					</table>
				</div>
			</div>
		</div>
	)
}