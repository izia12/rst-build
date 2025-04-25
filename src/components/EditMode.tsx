import { ReactElement, useState } from 'react'
import { useAppSelector } from '../store/store'


type propsType = {
	id: string
}
export default function EditMode({ id }: propsType): ReactElement {
	const uniqueItems = useAppSelector(state => state.wasm.groupUniqueItems)
	const uniqueItem = uniqueItems.find(el => el.id === id)
	const [color, setColor] = useState(uniqueItem?.name)
	const [name, setName] = useState("")
	return (
		<div>
			<input type="text"
				value={name}
				onChange={(e) => setName(e.currentTarget.value)}
			/>
			<div className="flex flex-col gap-2">
				<label className="text-sm font-medium text-gray-700">Выберите цвет</label>
				<div className="flex items-center gap-3">
					<input
						type="color"
						className="w-12 h-12 border rounded-md cursor-pointer bg-white"
						value={color}
						onChange={(e) => {
							setColor(e.currentTarget.value)
						}}
					/>
					<span className="text-sm text-gray-500">Нажмите для выбора цвета</span>
				</div>
			</div>
			<div>planes</div>
		</div>
	)
}
