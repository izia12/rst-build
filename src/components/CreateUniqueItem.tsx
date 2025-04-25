import React, { ReactElement, useState } from 'react'
import Modal from './custom-components/Modal'
import { Button } from './custom-components/Button'
import { useAppDispatch, useAppSelector } from '../store/store'
import { addToGroupUniqueItem } from '../store/slices/slice.wasm'


type propsType = {
	onClose: () => void,
	onOpen?: () => void,
	isOpen: boolean
}
export default function CreateUniqueItem({ onClose, isOpen }: propsType): ReactElement {
	const choosedPlains = useAppSelector(state => state.wasm.choosedPlainsFromList);
	const dispatch = useAppDispatch();
	const [name, setName] = useState("");
	const [color, setColor] =useState("")
  return (
		<>
			<Modal isOpen={isOpen} onClose={onClose} width={400}>
				<div className="flex flex-col gap-4 w-full">
					<div className="flex flex-col gap-2">
						<label className="text-sm font-medium text-gray-700">Название унификации</label>
						<input
							onChange={(e) => {
								setName(e.currentTarget.value)
							}}
							type="text"
							placeholder="Введите название"
							className="px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-all"
						/>
					</div>

					<div className="flex flex-col gap-2">
						<label className="text-sm font-medium text-gray-700">Выберите цвет</label>
						<div className="flex items-center gap-3">
							<input
								type="color"
								className="w-12 h-12 border rounded-md cursor-pointer bg-white"
								onChange={(e)=>{
									setColor(e.currentTarget.value)
								}}
							/>
							<span className="text-sm text-gray-500">Нажмите для выбора цвета</span>
						</div>
					</div>
				</div>
				<Button
					disabled = {name===""}
					onClick={() => {
						console.log(22);
						dispatch(addToGroupUniqueItem({
							id: "fff",
							color:color|| ('#' + Math.floor(Math.random() * 16777215).toString(16).padStart(6, 0..toString())),
							name,
							planes: choosedPlains
						}))
						setTimeout(()=>onClose(),1)
					}}
					buttonName='Добавить унификацию'
				/>
			</Modal>
		</>
	)
}
