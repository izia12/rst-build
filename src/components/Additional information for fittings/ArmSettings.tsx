import React, { ReactElement, useState } from 'react'
import Modal from '../custom-components/Modal'
import { AdditionInfoArm } from './AdditionInfoArm';
import { Button } from '../custom-components/Button';
import init, { get_excell_report_for_arms } from '../../assets/pkg/rst_build';


export default function ArmSettings(): ReactElement {
	const [isOpen, setIsOpen] = useState(false);
	async function saveXlsx() {
		try{
			await init()
			const data = await get_excell_report_for_arms();
			const combinedData = new Uint8Array(data);
			saveFile(combinedData)

		}catch(e){
			console.log(e);
		}
	}
	const saveFile = (data: Uint8Array,) => {
		const blob = new Blob([data], { type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' });
		const url = URL.createObjectURL(blob);
		const link = document.createElement('a');
		link.href = url;
		link.download = "out.xlsx";
		document.body.appendChild(link);
		link.click();
		document.body.removeChild(link);
		URL.revokeObjectURL(url);
	}
	return (
		<>
			<Modal
				isOpen={isOpen}
				onClose={() => setIsOpen(false)}
				width={1200}
				button={
					<Button
						onClick={() => saveXlsx()}
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
