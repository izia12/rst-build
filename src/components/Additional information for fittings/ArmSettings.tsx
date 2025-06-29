import React, { ReactElement, useState } from 'react'
import Modal from '../custom-components/Modal'
import { AdditionInfoArm } from './AdditionInfoArm';
import { Button } from '../custom-components/Button';


export default function ArmSettings(): ReactElement {
	const [isOpen, setIsOpen] = useState(false);
	return (
		<>
			<Modal
				isOpen={isOpen}
				onClose={() => setIsOpen(false)}
				width={1200}
				button={
					<Button
						// onClick={() => setOpenForCreateUI(!openForCreateUI)}
						buttonName='получить комбинации'
						classes="absolute bottom-12 right-4 p-2 bg-blue-500 rounded-md shadow-lg hover:shadow-none transition-shadow"
					/>
				}
			>
				<AdditionInfoArm/>
			</Modal>
		
			<button
				onClick={()=>setIsOpen(!isOpen)}
				className="rounded bg-blue-500 px-4 py-2 text-white hover:bg-blue-600 ml-2"
			>
				Задать арматуры
			</button>
		</>
	)
}
