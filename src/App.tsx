import { useCallback, useEffect, useState } from 'react'
import './App.css'
import { fetchWasmData, fetchWasmJSData } from './store/slices/thunks/wasmThanks.ts';
import { useAppDispatch, useAppSelector } from './store/store.ts';
import Three from './components/Three.tsx';
import { startPerfomance } from './store/slices/slice.wasm.ts';
import Modal from './components/custom-components/Modal.tsx';
import { UniqeItems } from './components/UniqeItems.tsx';
import { Loader } from './components/custom-components/Loader.tsx';
import { get_changed_row_data } from './assets/pkg/rst_build';
import ArmSettings from './components/Additional information for fittings/ArmSettings.tsx';
import {Canvas} from './components/Canvas.tsx';
import { ToastProvider } from './components/custom-components/Toast.tsx';
import CreateNewGroupPlanesButton from './components/CreateNewGroupPlanesButton.tsx';
import { ExcelView } from './components/Excel/ExcelView.tsx';


function App() {
	const dispatch = useAppDispatch();
	const [sliInput, setSliInput] = useState<string | null>(null);
	const [textInput, setTextInput] = useState<string | null>(null);
	const [xlsxInput, setXlsxInput] = useState<Uint8Array | null>(null);
	const [isOpen, setIsOpen] = useState(false);
	const [openForCreateUI, setOpenForCreateUI] = useState(false);
	const pending = useAppSelector(state => state.wasm.loading)
	const choosedPlanesFromList = useAppSelector(state=>state.wasm.choosedPlainsFromList);
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
	async function onLiraInputChange(event: React.ChangeEvent<HTMLInputElement>) {

		const file = event.target?.files?.[0]
		if (file) {
			const reader = new FileReader();
			reader.onload = (e) => {
				if (e.target) {
					setTextInput(e.target.result as string)
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

	const handleSaveDxf = async () => {
		try {
			const data = await get_changed_row_data(choosedPlanesFromList);
			const combinedData = new Uint8Array(data);
			let offset = 0;
			for (let i = 0; i < 4; i++) {
				// Чтение размера файла
				const sizeView = new DataView(combinedData.buffer, offset, 4);
				const fileSize = sizeView.getUint32(0, true);
				offset += 4;
				// Извлечение DXF файла
				const dxfFile = combinedData.slice(offset, offset + fileSize);
				offset += fileSize;
				// Сохранение файла
				saveFile(dxfFile, `as${i+1}.dxf`);
				await new Promise(resolve => setTimeout(resolve, 50));
			}
		} catch (error) {
			console.error('Ошибка сохранения:', error);
		}
	}
	const saveFile = (data: Uint8Array, filename: string) => {
		const blob = new Blob([data], { type: 'application/dxf' });
		const url = URL.createObjectURL(blob);
		const link = document.createElement('a');
		link.href = url;
		link.download = filename;
		document.body.appendChild(link);
		link.click();
		document.body.removeChild(link);
		URL.revokeObjectURL(url);
	}
	const onInputsChanged = useCallback(async () => {
		if (sliInput && xlsxInput && textInput) {
			await dispatch(fetchWasmData({
				sliData: sliInput,
				txtData:textInput,
				xlsxData: xlsxInput
			})).unwrap(); // Добавляем unwrap() для обработки ошибок

			const perf = performance.now();
			dispatch(startPerfomance(perf));
			await dispatch(fetchWasmJSData());
		}
	},[dispatch, sliInput, textInput, xlsxInput])
	useEffect(() => {
		if (sliInput && xlsxInput && textInput) {
			void onInputsChanged();
		}

	}, [onInputsChanged, sliInput, textInput, xlsxInput]); // Добавляем зависимости
	
	return (
		<div className="min-h-screen bg-gradient-to-br from-secondary-50 to-primary-50">
			<div className="container mx-auto px-4 sm:px-6 py-4 sm:py-8">
				{/* Header */}
				<div className="mb-6 sm:mb-8">
					<h1 className="text-2xl sm:text-3xl font-bold text-secondary-900 mb-2">3D Проектирование</h1>
					<p className="text-sm sm:text-base text-secondary-600">Система анализа и проектирования арматурных конструкций</p>
				</div>

				{pending && <Loader fullScreen={true}/>}
				
				{/* File Upload Section */}
				<div className="card mb-6 sm:mb-8 p-4 sm:p-6">
					<h2 className="text-lg sm:text-xl font-semibold text-secondary-900 mb-4 sm:mb-6">Загрузка файлов</h2>
					<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
						{/* SLI File Upload */}
						<div className="space-y-2">
							<label className="block text-sm font-medium text-secondary-700">SLI файл</label>
							<div className="relative">
								<input
									type="file"
									accept=".sli"
									className="hidden"
									id="sli-input"
									onChange={onSliInputChange}
								/>
								<label
									htmlFor="sli-input"
									className="flex flex-col items-center justify-center h-24 sm:h-32 border-2 border-dashed border-secondary-300 rounded-lg cursor-pointer bg-secondary-50 hover:bg-secondary-100 transition-colors group touch-manipulation"
								>
									<div className="flex flex-col items-center">
										<svg className="w-8 h-8 text-secondary-400 group-hover:text-primary-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
										</svg>
										<span className="mt-2 text-sm text-secondary-600 group-hover:text-primary-600 transition-colors">
											{sliInput ? 'Файл загружен' : 'Выберите .sli файл'}
										</span>
									</div>
								</label>
							</div>
						</div>

						{/* TXT File Upload */}
						<div className="space-y-2">
							<label className="block text-sm font-medium text-secondary-700">TXT файл</label>
							<div className="relative">
								<input
									type="file"
									accept=".txt"
									className="hidden"
									id="txt-input"
									onChange={onLiraInputChange}
								/>
								<label
									htmlFor="txt-input"
									className="flex flex-col items-center justify-center h-24 sm:h-32 border-2 border-dashed border-secondary-300 rounded-lg cursor-pointer bg-secondary-50 hover:bg-secondary-100 transition-colors group touch-manipulation"
								>
									<div className="flex flex-col items-center">
										<svg className="w-8 h-8 text-secondary-400 group-hover:text-primary-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
										</svg>
										<span className="mt-2 text-sm text-secondary-600 group-hover:text-primary-600 transition-colors">
											{textInput ? 'Файл загружен' : 'Выберите .txt файл'}
										</span>
									</div>
								</label>
							</div>
						</div>

						{/* XLSX File Upload */}
						<div className="space-y-2">
							<label className="block text-sm font-medium text-secondary-700">XLSX файл</label>
							<div className="relative">
								<input
									type="file"
									accept=".xlsx,.xls"
									className="hidden"
									id="xlsx-input"
									onChange={onXlsxInputChange}
								/>
								<label
									htmlFor="xlsx-input"
									className="flex flex-col items-center justify-center h-24 sm:h-32 border-2 border-dashed border-secondary-300 rounded-lg cursor-pointer bg-secondary-50 hover:bg-secondary-100 transition-colors group touch-manipulation"
								>
									<div className="flex flex-col items-center">
										<svg className="w-8 h-8 text-secondary-400 group-hover:text-success-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
										</svg>
										<span className="mt-2 text-sm text-secondary-600 group-hover:text-success-600 transition-colors">
											{xlsxInput ? 'Файл загружен' : 'Выберите .xlsx файл'}
										</span>
									</div>
								</label>
							</div>
						</div>
					</div>
				</div>

				{/* Action Buttons */}
				<div className="flex flex-wrap gap-4 mb-8">
					<button
						onClick={() => setIsOpen(true)}
						className="btn-primary w-full sm:w-auto min-h-[44px] touch-manipulation"
					>
						<svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
						</svg>
						Выбрать унификацию
					</button>
					
					<button
						onClick={handleSaveDxf}
						className="btn-success w-full sm:w-auto min-h-[44px] touch-manipulation"
						// disabled={choosedPlanesFromList.length === 0}
					>
						<svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
						</svg>
						Сохранить DXF
					</button>
					<div className="w-full sm:w-auto">
						<ArmSettings/>
					</div>
				</div>

				{/* Main Content */}
				<div className="">
					<div className="space-y-6">
						<ExcelView/>
						<Three />
						<Canvas />
					</div>
				</div>

				{/* Modal */}
				<Modal
					isOpen={isOpen}
					onClose={() => setIsOpen(false)}
					width={2200}
					button={
						<ToastProvider>
							<CreateNewGroupPlanesButton
								openForCreateUI={openForCreateUI}
								setOpenForCreateUI={setOpenForCreateUI}
							/>
						</ToastProvider>
					}
				>
					<ToastProvider>
						<UniqeItems openForCreateUI={openForCreateUI} setOpenForCreateUI={setOpenForCreateUI} />
					</ToastProvider>
				</Modal>
			</div>
		</div>
	)
}



export default App
