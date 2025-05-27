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
