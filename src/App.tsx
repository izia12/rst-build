import { useEffect, useState } from 'react'

import './App.css'
import { fetchWasmData, fetchWasmJSData } from './store/slices/thunks/wasmThanks.ts';
import { useAppDispatch, useAppSelector } from './store/store.ts';
import Three from './components/Three.tsx';
import { startPerfomance } from './store/slices/slice.wasm.ts';
import Quadrilaterals from './components/SVG.tsx';
import Modal from './components/custom-components/Modal.tsx';
import { UniqeItems } from './components/UniqeItems.tsx';
import { Loader } from './components/custom-components/Loader.tsx';
import { Button } from './components/custom-components/Button.tsx';

function App() {
	const dispatch = useAppDispatch();
	const [sliInput, setSliInput] = useState<string | null>(null);
	const [xlsxInput, setXlsxInput] = useState<Uint8Array<ArrayBufferLike> | null>(null);
	const [isOpen, setIsOpen] = useState(false);
	const [openForCreateUI, setOpenForCreateUI] = useState(false);
	const pending = useAppSelector(state=>state.wasm.loading)
	async function onSliInputChange(event: React.ChangeEvent<HTMLInputElement>) {
		const file = event.target?.files?.[0]
		if (file) {
			const reader = new FileReader();
			reader.onload = (e) => {
				if (e.target) {
					setSliInput(e.target.result as string)
				}
			}
			reader.readAsText(file)
			// setSliInput(file)
		}
	}
	async function onXlsxInputChange(event: React.ChangeEvent<HTMLInputElement>) {
		const file = event.target?.files?.[0]
		if (file) {
			const reader = new FileReader();
			reader.onload = (e) => {
				if (e.target) {
					const arrayBuffer = e.target.result as ArrayBuffer;
					setXlsxInput(new Uint8Array(arrayBuffer));
				}
			};
			reader.readAsArrayBuffer(file);
		}
	}
	async function onTwoInputsChange() {
		if (sliInput && xlsxInput) {
			await dispatch(fetchWasmData({
				sliData: sliInput,
				xlsxData: xlsxInput
			})).unwrap(); // Добавляем unwrap() для обработки ошибок
			
			const perf = performance.now();
			dispatch(startPerfomance(perf));
			await dispatch(fetchWasmJSData());
		}
	}
	useEffect(() => {
		if (sliInput && xlsxInput) {
			void onTwoInputsChange();
		}
	}, [sliInput, xlsxInput]); // Добавляем зависимости
	return (
		<>
			<div className="container m-auto w-full" >
				<div role="form w-full">
					{pending&&<Loader/>}
					<div className="form-group flex mb-4">
						{/* <label htmlFor="exampleInputFile">Choose a DXF file</label> */}
						<div className="flex items-center mx-2">
							<label
								className="flex flex-col items-center justify-center w-full h-48 border-2 border-dashed border-gray-300 rounded-lg cursor-pointer bg-gray-50 hover:bg-gray-100">
								<span className="text-gray-500">Перетащите файл сюда или нажмите для выбора</span>
								<input
									className={"py-1 px-2 text-xl bg-blue-500 text-white rounded"}
									type="file"
									accept=".sli"
									id="file-input"
									name="file"
									onChange={(e) => {
										onSliInputChange(e);
									}} />
							</label>
						</div>
						<div className="flex items-center mx-2">
							<label
								className="flex flex-col items-center justify-center w-full h-48 border-2 border-dashed border-gray-300 rounded-lg cursor-pointer bg-gray-50 hover:bg-gray-100">
								<span className="text-gray-500">Перетащите файл сюда или нажмите для выбора</span>
								<input
									className={"py-1 px-2 text-xl bg-blue-500 text-white rounded"}
									type="file"
									accept=".xlsx"
									id="file-input"
									name="file"
									onChange={(e) => {
										onXlsxInputChange(e)
									}} />
							</label>
						</div>
						<div className="progress progress-striped" style={{ width: '300px' }}>
							<div id="file-progress-bar" className="progress-bar progress-bar-success" role="progressbar"
								style={{ width: '0' }}>
							</div>
						</div>
					</div>
					<button
						onClick={() => setIsOpen(true)}
						className="rounded bg-blue-500 px-4 py-2 text-white hover:bg-blue-600"
					>
						Выбрать унификацию
					</button>

					<Modal 
						isOpen={isOpen} 
						onClose={() => setIsOpen(false)} 
						width={1200}
						button={
							<Button
							onClick={()=>setOpenForCreateUI(!openForCreateUI)}
							classes="absolute bottom-12 right-4 p-2 bg-blue-500 rounded-md shadow-lg hover:shadow-none transition-shadow"  
							/>
						}
						>
						<UniqeItems openForCreateUI={openForCreateUI} setOpenForCreateUI={setOpenForCreateUI}/>

					</Modal>
					<Three/>
					<Quadrilaterals />
					{/* <InteractiveCubes /> */}
				</div>
				{/*<FileUploadAndDocxGenerator/>*/}
				{/*<DiagramGenerator/>*/}
			</div>
		</>
	)
}



export default App
