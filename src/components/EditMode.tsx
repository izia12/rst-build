import { ReactElement, useState, useRef } from 'react'
import { useAppDispatch, useAppSelector } from '../store/store'
import { Button } from './custom-components/Button';
import { updateUniqueItem } from '../store/slices/slice.wasm';

type PropsType = {
	id: string;
	name:string,
	color:string
	planes:number[],
	onClose:()=>void
};

export default function EditMode({ id, color, name, planes, onClose }: PropsType): ReactElement {
	const wasmJsData = useAppSelector(state => state.wasm.wasmJsData);
	const selectedPlainsToUnification = useAppSelector(state => state.wasm.groupUniqueItems)
	const dispatch = useAppDispatch();
	const [newColor, setNewColor] = useState(color)
	const [newName, setNewName] = useState(name)
	const dragItem = useRef<number | null>(null);
	const dragOverItem = useRef<number | null>(null);
	const freeplains = Object.entries(wasmJsData || {})
	?.filter((el) => {
		const choosedAllPlains = selectedPlainsToUnification.map(el => el.planes).flat()
		return !choosedAllPlains.includes(+el);
	})
	const [newStatePlanes, setNewStatePlanes] = useState(planes); // Ваши элементы для перетаскивания
	const[selectedPlainsFromFree, setSelectedPlainsFromFree] = useState(freeplains)
	const handleDragStart = (e: React.DragEvent<HTMLDivElement>, index: number) => {
		dragItem.current = index;
		e.currentTarget.classList.add('opacity-50', 'scale-90');
	};

	const handleDragEnter = (e: React.DragEvent<HTMLDivElement>, index: number) => {
		dragOverItem.current = index;
		e.currentTarget.classList.add('border-2', 'border-blue-300');
	};

	const handleDragLeave = (e: React.DragEvent<HTMLDivElement>) => {
		e.currentTarget.classList.remove('border-2', 'border-blue-300');
	};

	const handleDrop = () => {
		if (dragItem.current === null || dragOverItem.current === null) return;

		const newPlanes = [...newStatePlanes];
		const draggedItem = newPlanes[dragItem.current];
		newPlanes.splice(dragItem.current, 1);
		newPlanes.splice(dragOverItem.current, 0, draggedItem);

		setNewStatePlanes(newPlanes);
		dragItem.current = null;
		dragOverItem.current = null;
	};

	return (
		<div>
			<div className="flex flex-col gap-2">
				<label className="text-sm font-medium text-gray-700">Название унификации</label>
				<input
					value={newName}
					onChange={(e) => {
						setNewName(e.currentTarget.value)
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
						value={color}
						onChange={(e) => {
							setNewColor(e.currentTarget.value)
						}}
					/>
					<span className="text-sm text-gray-500">Нажмите для выбора цвета</span>
				</div>
			</div>
			<div className='flex justify-between'>
				<div className='flex-col'>
					<div>Этажи в группе</div>
					<div className="space-y-2 mt-4">
						{newStatePlanes.map((plane, index) => (
							<div
								key={index}
								draggable
								onDragStart={(e) => handleDragStart(e, index)}
								onDragEnter={(e) => handleDragEnter(e, index)}
								onDragLeave={handleDragLeave}
								onDragEnd={handleDrop}
								className="p-3 bg-white rounded-lg shadow-sm transition-all duration-200 cursor-grab
								hover:shadow-md active:cursor-grabbing"
							>
								{plane}
							</div>
						))}
					</div>
				</div>
				<div className='flex-col'>
					<div>Этажи без группы</div>
					<div className="space-y-2 mt-4">
						{
							selectedPlainsFromFree
								?.filter(([sp])=>{
									const choosedAllPlains = selectedPlainsToUnification.map(el => el.planes).flat()
										return !choosedAllPlains.includes(+sp);
								})
								.sort(([el,],[el2])=>{
									if(+el>+el2)return-1
									else return 1
								})
								?.map(([el])=>(
									<div key={el}>
										<input type="checkbox" name="" id={el} onChange={(e)=>{
											const id = e.target.id
											console.log(id);
											setSelectedPlainsFromFree(selectedPlainsFromFree.filter(([sel])=>sel!==el))
										}}/>
										<span>{el}</span>
									</div>
								))
						}
					</div>
				</div>
			</div>
			<Button 
			buttonName='Обновить'
			onClick={()=>{
				dispatch(
					updateUniqueItem({
						id,
						newUniqeItem:{
							id,
							color:newColor,
							planes:newStatePlanes,
							name:newName
						}
					})
				)

				setTimeout(()=>{
					onClose()
				},1)
			}}/>
		</div>
	);
}
