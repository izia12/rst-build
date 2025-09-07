import React from 'react';
import { useAppSelector } from '../store/store';
import { useAppDispatch } from '../store/store';
import { generateDocumentForSelectedCombinations, generateDocumentWithColorPalette } from '../store/slices/thunks/wasmThanks';
import { Button } from './custom-components/Button';
import { SelectedCombination, SelectedCombinationsData } from '../types/data.types';

export const DocumentGenerator: React.FC = () => {
	const dispatch = useAppDispatch();
	const { excelViewData, documentGeneration } = useAppSelector(state => state.wasm);

	const handleGenerateDocument = async () => {
		// Собираем выбранные комбинации с их данными
		const selectedCombinations: SelectedCombination[] = [];
		const selectedFloors: string[] = [];

		excelViewData.forEach(floor => {
			floor.values.forEach(armatureCombination => {
				armatureCombination.combinations.forEach(item => {
					if (item.is_default_checked) {
						console.log(`🔵 [FRONTEND_SENDING] Floor: ${floor.level}, Function: ${armatureCombination.function_name}`);
						console.log(`🔵 [FRONTEND_SENDING] Sending result_scale: ${item.result_scale || 'NULL'}`);
						console.log(`🔵 [FRONTEND_SENDING] Item details:`, item);

						// Добавляем выбранную комбинацию
						selectedCombinations.push({
							floor_level: floor.level,
							function_name: armatureCombination.function_name,
							as_target_value: armatureCombination.as_target_value,
							combination: {
								...item,
								result_scale: item.result_scale || `[${item.total_area.toFixed(3)}см2:Ø${item.main_diameter}${item.additional_diameter ? '+Ø' + item.additional_diameter : ''}]`
							}
						});


						// Добавляем этаж в список если его еще нет
						if (!selectedFloors.includes(floor.level)) {
							selectedFloors.push(floor.level);
						}
					}
				});
			});
		});

		if (selectedCombinations.length === 0) {
			alert('Не выбрано ни одной комбинации для генерации документа');
			return;
		}

		const selectedData: SelectedCombinationsData = {
			combinations: selectedCombinations,
			floors: selectedFloors
		};

		// Убраны детальные логи формирования данных

		try {
			// Используем новую функцию с цветовой палитрой
			const result = await dispatch(generateDocumentWithColorPalette(selectedData)).unwrap();

			// Создаем blob и скачиваем файл
			const blob = new Blob([result], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
			const url = URL.createObjectURL(blob);
			const link = document.createElement('a');
			link.href = url;
			link.download = `document_${new Date().toISOString().slice(0, 10)}.docx`;
			document.body.appendChild(link);
			link.click();
			document.body.removeChild(link);
			URL.revokeObjectURL(url);
		} catch (error) {
			console.error('Ошибка при генерации документа:', error);
			alert('Ошибка при генерации документа');
		}
	};

	return (
		<div className="space-y-4">
			{/* 🧪 ЭКСПЕРИМЕНТАЛЬНАЯ СЕКЦИЯ */}
			<div className="bg-gradient-to-r from-yellow-100 to-orange-100 p-4 rounded-lg border-2 border-dashed border-yellow-400">
				<h3 className="text-lg font-bold text-orange-800 mb-3">🧪 ЭКСПЕРИМЕНТЫ: WASM + Web Workers</h3>
				<p className="text-sm text-gray-600 mb-4">
					Тестируем производительность разных подходов генерации документов
				</p>
				<div className="flex flex-wrap gap-3">
					{/* 🧪 НОВЫЙ ТЕСТОВЫЙ МОДУЛЬ */}
					<Button
						onClick={async () => {
							try {
								console.log('🧪 [ТЕСТ-MODULE] Запуск тестового модуля...');
								const startTime = performance.now();

								const wasmModule = await import('../assets/pkg/rst_build.js');

								// Вызываем новую тестовую функцию
								const result = await wasmModule.create_test_docx_with_images(5, 3); // 5 картинок, сложность 3
								const totalTime = performance.now() - startTime;

								console.log('🧪 [ТЕСТ-MODULE] Тестовый модуль завершен за:', Math.round(totalTime), 'мс');
								console.log('🧪 [ТЕСТ-MODULE] Размер DOCX:', result.length, 'байт');

								// Скачиваем файл
								const blob = new Blob([result], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
								const url = URL.createObjectURL(blob);
								const link = document.createElement('a');
								link.href = url;
								link.download = `test_document_${new Date().toISOString().slice(0, 10)}.docx`;
								document.body.appendChild(link);
								link.click();
								document.body.removeChild(link);
								URL.revokeObjectURL(url);

								alert(`✅ Тестовый модуль работает!\n⏱️ Время: ${Math.round(totalTime)}мс\n📄 Размер: ${result.length} байт\n💾 Файл скачан`);
							} catch (error) {
								console.error('❌ [ТЕСТ-MODULE] Ошибка:', error);
								alert('Ошибка тестового модуля: ' + error);
							}
						}}
						className="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
					>
						🧪 Тестовый DOCX (Main Thread)
					</Button>
					<Button
						onClick={async () => {
							try {
								console.log('🧪 [ТЕСТ-MAIN] Тест WASM в главном потоке...');
								const startTime = performance.now();

								const wasmModule = await import('../assets/pkg/rst_build.js');
								const loadTime = performance.now() - startTime;

								console.log('🧪 [ТЕСТ-MAIN] WASM загружен за:', Math.round(loadTime), 'мс');

								// Простой тест функции
								if (typeof wasmModule.convert_data_to_js_order_byz === 'function') {
									const testStart = performance.now();
									const result = wasmModule.convert_data_to_js_order_byz();
									const testTime = performance.now() - testStart;

									console.log('🧪 [ТЕСТ-MAIN] Функция выполнена за:', Math.round(testTime), 'мс');
									console.log('🧪 [ТЕСТ-MAIN] Результат (размер):', result.length, 'символов');

									alert(`✅ WASM в главном потоке работает!\n⏱️ Загрузка: ${Math.round(loadTime)}мс\n⚡ Выполнение: ${Math.round(testTime)}мс`);
								} else {
									alert('❌ Тестовая функция недоступна');
								}
							} catch (error) {
								console.error('❌ [ТЕСТ-MAIN] Ошибка:', error);
								alert('Ошибка WASM в главном потоке: ' + error);
							}
						}}
						disabled={!excelViewData || excelViewData.length === 0}
						className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
					>
						🧪 Тест WASM (Main Thread)
					</Button>

					{/* Тест простого Web Worker */}
					<Button
						onClick={async () => {
							try {
								console.log('🧪 [ТЕСТ-WORKER] Тест простого Web Worker...');
								const startTime = performance.now();

								// Создаем простой тестовый Worker
								const worker = new Worker(
									new URL('../workers/test-worker.ts', import.meta.url),
									{ type: 'module' }
								);

								worker.postMessage({ command: 'test', data: 'hello' });

								worker.onmessage = (event) => {
									const totalTime = performance.now() - startTime;
									console.log('🧪 [ТЕСТ-WORKER] Ответ от Worker:', event.data);
									console.log('🧪 [ТЕСТ-WORKER] Время выполнения:', Math.round(totalTime), 'мс');

									worker.terminate();
									alert(`✅ Простой Worker работает!\n⏱️ Время: ${Math.round(totalTime)}мс\n📨 Ответ: ${event.data.result}`);
								};

								worker.onerror = (error) => {
									console.error('❌ [ТЕСТ-WORKER] Ошибка Worker:', error);
									worker.terminate();
									alert('Ошибка простого Worker: ' + error.message);
								};

							} catch (error) {
								console.error('❌ [ТЕСТ-WORKER] Критическая ошибка:', error);
								alert('Критическая ошибка простого Worker: ' + error);
							}
						}}
						disabled={!excelViewData || excelViewData.length === 0}
						className="bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
					>
						🧪 Простой Worker
					</Button>
					<Button
						onClick={async () => {
							try {
								console.log('🧪 [ТЕСТ-WASM-WORKER] Тест WASM в Web Worker...');
								const startTime = performance.now();

								// Создаем Worker для тестирования WASM
								const worker = new Worker(
									new URL('../workers/wasm-test-worker.ts', import.meta.url),
									{ type: 'module' }
								);

								worker.postMessage({ command: 'test_wasm' });

								worker.onmessage = (event) => {
									const totalTime = performance.now() - startTime;
									console.log('🧪 [ТЕСТ-WASM-WORKER] Ответ от Worker:', event.data);
									console.log('🧪 [ТЕСТ-WASM-WORKER] Общее время:', Math.round(totalTime), 'мс');

									worker.terminate();

									if (event.data.success) {
										alert(`✅ WASM в Worker РАБОТАЕТ!\n⏱️ Общее время: ${Math.round(totalTime)}мс\n📊 Результат: ${event.data.dataSize} символов\n🚀 Инициализация: ${event.data.initTime || 'N/A'}мс\n🔍 Превью: ${event.data.resultPreview ? event.data.resultPreview.substring(0, 50) + '...' : 'N/A'}`);
									} else {
										alert(`❌ WASM в Worker ПРОБЛЕМА!\n🚫 Ошибка: ${event.data.error}\n📊 Размер: ${event.data.dataSize || 'N/A'}\n🔍 Данные: ${event.data.resultPreview || 'N/A'}`);
									}
								};

								worker.onerror = (error) => {
									console.error('❌ [ТЕСТ-WASM-WORKER] Ошибка Worker:', error);
									worker.terminate();
									alert('❌ Критическая ошибка WASM Worker: ' + error.message);
								};

							} catch (error) {
								console.error('❌ [ТЕСТ-WASM-WORKER] Критическая ошибка:', error);
								alert('Критическая ошибка WASM Worker: ' + error);
							}
						}}
						disabled={!excelViewData || excelViewData.length === 0}
						className="bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
					>
						🧪 Тест WASM в Worker
					</Button>
				</div>
			</div>

			{/* 🧪 Тест нового модуля в Worker */}
			<div className="bg-gradient-to-r from-orange-100 to-red-100 p-4 rounded-lg border-2 border-dashed border-orange-400">
				<h3 className="text-lg font-bold text-red-800 mb-3">🧪 НОВЫЙ ТЕСТ: Новый DOCX модуль</h3>
				<p className="text-sm text-gray-600 mb-4">
					Тестируем новую Rust функцию которая создает картинки и DOCX без парсинга файлов
				</p>
				<div className="flex flex-wrap gap-3">
					<Button
						onClick={async () => {
							try {
								console.log('🧪 [ТЕСТ-NEW-MAIN] Тест нового модуля в основном потоке...');
								const startTime = performance.now();

								// Импортируем WASM модуль в основном потоке
								console.log('🧪 [ТЕСТ-NEW-MAIN] Загрузка WASM модуля...');
								const loadStart = performance.now();

								const wasmModule = await import('../assets/pkg/rst_build.js');
								const loadTime = performance.now() - loadStart;

								console.log('🧪 [ТЕСТ-NEW-MAIN] WASM модуль загружен за:', Math.round(loadTime), 'мс');

								// Инициализация
								if (typeof wasmModule.default === 'function') {
									await wasmModule.default();
									console.log('🧪 [ТЕСТ-NEW-MAIN] WASM инициализирован');
								}

								// Вызов новой тестовой функции
								if (typeof wasmModule.create_test_docx_with_images === 'function') {
									console.log('🧪 [ТЕСТ-NEW-MAIN] Вызов новой тестовой функции...');
									const funcStart = performance.now();

									// Вызываем с теми же параметрами
									const result = await wasmModule.create_test_docx_with_images(3, 2); // 3 картинки, сложность 2
									const funcTime = performance.now() - funcStart;
									const totalTime = performance.now() - startTime;

									console.log('🧪 [ТЕСТ-NEW-MAIN] Новая функция выполнена за:', Math.round(funcTime), 'мс');
									console.log('🧪 [ТЕСТ-NEW-MAIN] Размер DOCX:', result.length, 'байт');
									console.log('🧪 [ТЕСТ-NEW-MAIN] Общее время:', Math.round(totalTime), 'мс');

									// Скачиваем файл
									const blob = new Blob([result], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
									const url = URL.createObjectURL(blob);
									const link = document.createElement('a');
									link.href = url;
									link.download = `test_main_document_${new Date().toISOString().slice(0, 10)}.docx`;
									document.body.appendChild(link);
									link.click();
									document.body.removeChild(link);
									URL.revokeObjectURL(url);

									alert(`✅ Новый модуль в основном потоке РАБОТАЕТ!\n⏱️ Время загрузки: ${Math.round(loadTime)}мс\n⏱️ Время выполнения: ${Math.round(funcTime)}мс\n⏱️ Общее время: ${Math.round(totalTime)}мс\n📄 Размер DOCX: ${result.length} байт\n💾 Файл скачан`);
								} else {
									alert('❌ Функция create_test_docx_with_images недоступна - нужна пересборка WASM');
								}

							} catch (error) {
								console.error('❌ [ТЕСТ-NEW-MAIN] Критическая ошибка:', error);
								alert('Критическая ошибка нового модуля (основной поток): ' + error);
							}
						}}
						className="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
					>
						🧪 Новый модуль (Основной поток)
					</Button>
					<Button
						onClick={async () => {
							try {
								console.log('🧪 [ТЕСТ-NEW-WORKER] Тест нового модуля в Worker...');
								const startTime = performance.now();

								// Создаем Worker для нового модуля
								const worker = new Worker(
									new URL('../workers/wasm-test-worker.ts', import.meta.url),
									{ type: 'module' }
								);

								worker.postMessage({ command: 'test_new_module' });

								worker.onmessage = (event) => {
									const totalTime = performance.now() - startTime;
									console.log('🧪 [ТЕСТ-NEW-WORKER] Ответ от Worker:', event.data);
									console.log('🧪 [ТЕСТ-NEW-WORKER] Общее время UI:', Math.round(totalTime), 'мс');

									worker.terminate();

									if (event.data.success) {
										// Скачиваем DOCX из Worker
										const blob = new Blob([event.data.docxData], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
										const url = URL.createObjectURL(blob);
										const link = document.createElement('a');
										link.href = url;
										link.download = `test_worker_document_${new Date().toISOString().slice(0, 10)}.docx`;
										document.body.appendChild(link);
										link.click();
										document.body.removeChild(link);
										URL.revokeObjectURL(url);

										alert(`✅ Новый модуль в Worker РАБОТАЕТ!\n⏱️ Worker время: ${event.data.functionTime}мс\n⏱️ Общее время: ${Math.round(totalTime)}мс\n📄 Размер DOCX: ${event.data.docxSize} байт\n💾 Файл скачан`);
									} else {
										alert(`❌ Новый модуль в Worker НЕ работает!\n🚫 Ошибка: ${event.data.error}\n📝 Подробности: ${event.data.details}`);
									}
								};

								worker.onerror = (error) => {
									console.error('❌ [ТЕСТ-NEW-WORKER] Ошибка Worker:', error);
									worker.terminate();
									alert('Критическая ошибка Worker: ' + error.message);
								};

							} catch (error) {
								console.error('❌ [ТЕСТ-NEW-WORKER] Критическая ошибка:', error);
								alert('Критическая ошибка нового модуля: ' + error);
							}
						}}
						className="bg-orange-600 hover:bg-orange-700 text-white font-bold py-2 px-4 rounded"
					>
						🧪 Новый модуль (Worker)
					</Button>

					{/* ТЯЖЕЛЫЕ ТЕСТЫ */}
					<div className="w-full mt-4 pt-4 border-t-2 border-dashed border-red-500">
						<h4 className="text-md font-bold text-red-700 mb-2">🔥 ТЯЖЕЛЫЕ НАГРУЗОЧНЫЕ ТЕСТЫ</h4>
						<div className="flex flex-wrap gap-2">
							<Button
								onClick={async () => {
									try {
										console.log('🔥 [HEAVY-MAIN] Тяжелый тест в основном потоке...');
										const startTime = performance.now();

										const wasmModule = await import('../assets/pkg/rst_build.js');
										if (typeof wasmModule.default === 'function') {
											await wasmModule.default();
										}

										if (typeof wasmModule.create_test_docx_with_images === 'function') {
											const funcStart = performance.now();

											// 50 изображений, сложность 100 = 50,000 фигур
											const result = await wasmModule.create_test_docx_with_images(50, 100);
											const funcTime = performance.now() - funcStart;
											const totalTime = performance.now() - startTime;

											console.log('🔥 [HEAVY-MAIN] Тяжелая функция выполнена за:', Math.round(funcTime), 'мс');
											console.log('🔥 [HEAVY-MAIN] Размер DOCX:', result.length, 'байт');

											const blob = new Blob([result], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
											const url = URL.createObjectURL(blob);
											const link = document.createElement('a');
											link.href = url;
											link.download = `heavy_main_50img_50k_shapes_${new Date().toISOString().slice(0, 10)}.docx`;
											document.body.appendChild(link);
											link.click();
											document.body.removeChild(link);
											URL.revokeObjectURL(url);

											alert(`🔥 ТЯЖЕЛЫЙ тест (основной поток)\n📊 50 изображений, 50,000 фигур\n⏱️ Время: ${Math.round(funcTime)}мс\n⏱️ Общее: ${Math.round(totalTime)}мс\n📄 Размер: ${(result.length / 1024 / 1024).toFixed(1)}MB`);
										} else {
											alert('❌ Функция недоступна');
										}
									} catch (error) {
										console.error('❌ [HEAVY-MAIN] Ошибка:', error);
										alert('Ошибка тяжелого теста: ' + error);
									}
								}}
								className="bg-red-600 hover:bg-red-700 text-white font-bold py-1 px-2 text-sm rounded"
							>
								🔥 Основной (50 изобр, 50k фигур)
							</Button>

							<Button
								onClick={async () => {
									try {
										console.log('🔥 [HEAVY-WORKER] Тяжелый тест в Worker...');
										const startTime = performance.now();

										const worker = new Worker(
											new URL('../workers/wasm-test-worker.ts', import.meta.url),
											{ type: 'module' }
										);

										// Отправляем команду с параметрами тяжелого теста
										worker.postMessage({ command: 'test_heavy_module', imageCount: 50, complexity: 100 });

										worker.onmessage = (event) => {
											const totalTime = performance.now() - startTime;
											console.log('🔥 [HEAVY-WORKER] Ответ от Worker:', event.data);

											worker.terminate();

											if (event.data.success) {
												const blob = new Blob([event.data.docxData], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
												const url = URL.createObjectURL(blob);
												const link = document.createElement('a');
												link.href = url;
												link.download = `heavy_worker_50img_50k_shapes_${new Date().toISOString().slice(0, 10)}.docx`;
												document.body.appendChild(link);
												link.click();
												document.body.removeChild(link);
												URL.revokeObjectURL(url);

												alert(`🔥 ТЯЖЕЛЫЙ тест (Worker)\n📊 50 изображений, 50,000 фигур\n⏱️ Worker: ${event.data.functionTime}мс\n⏱️ Общее: ${Math.round(totalTime)}мс\n📄 Размер: ${(event.data.docxSize / 1024 / 1024).toFixed(1)}MB`);
											} else {
												alert(`❌ Тяжелый тест Worker ОШИБКА: ${event.data.error}`);
											}
										};

										worker.onerror = (error) => {
											console.error('❌ [HEAVY-WORKER] Ошибка Worker:', error);
											worker.terminate();
											alert('Критическая ошибка Worker: ' + error.message);
										};

									} catch (error) {
										console.error('❌ [HEAVY-WORKER] Критическая ошибка:', error);
										alert('Критическая ошибка: ' + error);
									}
								}}
								className="bg-purple-600 hover:bg-purple-700 text-white font-bold py-1 px-2 text-sm rounded"
							>
								🔥 Worker (50 изобр, 50k фигур)
							</Button>

							<Button
								onClick={async () => {
									try {
										console.log('💀 [EXTREME-MAIN] ЭКСТРЕМАЛЬНЫЙ тест в основном потоке...');
										const startTime = performance.now();

										const wasmModule = await import('../assets/pkg/rst_build.js');
										if (typeof wasmModule.default === 'function') {
											await wasmModule.default();
										}

										if (typeof wasmModule.create_test_docx_with_images === 'function') {
											const funcStart = performance.now();

											// 100 изображений, сложность 300 = 300,000 фигур
											const result = await wasmModule.create_test_docx_with_images(100, 300);
											const funcTime = performance.now() - funcStart;
											const totalTime = performance.now() - startTime;

											console.log('💀 [EXTREME-MAIN] ЭКСТРЕМАЛЬНАЯ функция выполнена за:', Math.round(funcTime), 'мс');

											const blob = new Blob([result], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
											const url = URL.createObjectURL(blob);
											const link = document.createElement('a');
											link.href = url;
											link.download = `extreme_main_100img_300k_shapes_${new Date().toISOString().slice(0, 10)}.docx`;
											document.body.appendChild(link);
											link.click();
											document.body.removeChild(link);
											URL.revokeObjectURL(url);

											alert(`💀 ЭКСТРЕМАЛЬНЫЙ тест (основной поток)\n📊 100 изображений, 300,000 фигур\n⏱️ Время: ${Math.round(funcTime)}мс\n⏱️ Общее: ${Math.round(totalTime)}мс\n📄 Размер: ${(result.length / 1024 / 1024).toFixed(1)}MB`);
										} else {
											alert('❌ Функция недоступна');
										}
									} catch (error) {
										console.error('❌ [EXTREME-MAIN] Ошибка:', error);
										alert('Ошибка экстремального теста: ' + error);
									}
								}}
								className="bg-black hover:bg-gray-800 text-white font-bold py-1 px-2 text-sm rounded"
							>
								💀 Основной (100 изобр, 300k фигур)
							</Button>

							{/* 📊 ЧЕСТНОЕ СРАВНЕНИЕ */}
							<div className="w-full mt-4 pt-4 border-t-4 border-solid border-red-500">
								<h4 className="text-lg font-bold text-red-700 mb-2">📊 ЧЕСТНОЕ СРАВНЕНИЕ! (МИНИМУМ ЛОГОВ)</h4>
								<p className="text-sm text-gray-600 mb-3">ОДНА И ТА ЖЕ функция create_partial_images - 5000 изображений. МИНИМУМ ЛОГОВ!</p>

								<div className="flex gap-2">
									<Button
										onClick={async () => {
											try {
												console.log('📊 ОСНОВНОЙ ПОТОК');
												const startTime = performance.now();
												const wasmModule = await import('../assets/pkg/rst_build.js');
												if (wasmModule.default) await wasmModule.default();
												const imagesArray = await wasmModule.create_partial_images(0, 5000, 100);
												const funcTime = performance.now() - startTime;
												let totalSize = 0;
												for (let i = 0; i < imagesArray.length; i++) totalSize += (imagesArray[i] as Uint8Array).length;
												console.log(`📊 ОСНОВНОЙ: ${Math.round(funcTime)}мс, ${imagesArray.length} изобр, ${(totalSize / 1024 / 1024).toFixed(1)}MB`);
												alert(`📊 ОСНОВНОЙ ПОТОК\n⏱️ ${Math.round(funcTime)}мс\n📊 КОЛИЧЕСТВО: ${imagesArray.length} изображений\n💾 Размер: ${(totalSize / 1024 / 1024).toFixed(1)}MB`);
											} catch (error) {
												alert('Ошибка: ' + error);
											}
										}}
										className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
									>
										📊 ОСНОВНОЙ (5000 изобр)
									</Button>

									<Button
										onClick={async () => {
											try {
												console.log('🚀 ВОРКЕРЫ');
												const startTime = performance.now();
												const workerPromises = [];
												for (let i = 0; i < 50; i++) {
													const startIndex = i * 100;
													const workerPromise = new Promise((resolve, reject) => {
														const worker = new Worker(new URL('../workers/parallel-image-worker.ts', import.meta.url), { type: 'module' });
														worker.postMessage({ workerId: i + 1, startIndex, imageCount: 100, complexity: 100 });
														worker.onmessage = (event) => { worker.terminate(); resolve(event.data); };
														worker.onerror = (error) => { worker.terminate(); reject(error); };
													});
													workerPromises.push(workerPromise);
												}
												const results = await Promise.all(workerPromises);
												const parallelTime = performance.now() - startTime;
												let processedImages = 0, totalSize = 0, maxWorkerTime = 0;
												results.forEach((result: any) => {
													processedImages += result.imageCount;
													totalSize += result.totalSize;
													maxWorkerTime = Math.max(maxWorkerTime, result.functionTime);
												});
												console.log(`🚀 ВОРКЕРЫ: ${Math.round(parallelTime)}мс, макс воркер: ${maxWorkerTime}мс`);
												alert(`🚀 ВОРКЕРЫ\n⏱️ ${Math.round(parallelTime)}мс\n📊 КОЛИЧЕСТВО: ${processedImages} изображений\n💾 Размер: ${(totalSize / 1024 / 1024).toFixed(1)}MB\n🚀 Макс воркер: ${maxWorkerTime}мс`);
											} catch (error) {
												alert('Ошибка: ' + error);
											}
										}}
										className="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
									>
										🚀 ВОРКЕРЫ (50×100 изобр)
									</Button>
								</div>
							</div>

							{/* Старые кнопки */}
							<div className="w-full mt-4 pt-4 border-t-4 border-solid border-green-500">
								<h4 className="text-lg font-bold text-green-700 mb-2">🚀 РЕАЛЬНЫЙ ПАРАЛЛЕЛИЗМ!</h4>
								<p className="text-sm text-gray-600 mb-3">5 воркеров генерируют изображения ПАРАЛЛЕЛЬНО!</p>

								<Button
									onClick={async () => {
										try {
											console.log('🚀 [REAL-PARALLEL] НАСТОЯЩИЙ параллелизм с 5 воркерами!');
											const startTime = performance.now();

											// Параметры теста - увеличиваем в 100 раз!
											const totalImages = 5000; // Общее количество изображений (100x больше)
											const numWorkers = 50;   // Количество воркеров (10x больше)
											const imagesPerWorker = Math.ceil(totalImages / numWorkers); // 100 изображений на воркер
											const complexity = 100; // Та же сложность

											console.log(`🚀 [REAL-PARALLEL] Запуск ${numWorkers} воркеров по ${imagesPerWorker} изображений каждый...`);

											// Создаем массив промисов для параллельной обработки
											const workerPromises = [];

											for (let i = 0; i < numWorkers; i++) {
												const startIndex = i * imagesPerWorker;
												const imageCount = Math.min(imagesPerWorker, totalImages - startIndex);

												// Пропускаем если изображений не осталось
												if (imageCount <= 0) continue;

												console.log(`🚀 [REAL-PARALLEL] Воркер ${i + 1}: индекс ${startIndex}, количество ${imageCount}`);

												// Создаем промис для каждого воркера
												const workerPromise = new Promise((resolve, reject) => {
													const worker = new Worker(
														new URL('../workers/parallel-image-worker.ts', import.meta.url),
														{ type: 'module' }
													);

													// Отправляем задание воркеру
													worker.postMessage({
														workerId: i + 1,
														startIndex: startIndex,
														imageCount: imageCount,
														complexity: complexity
													});

													worker.onmessage = (event) => {
														worker.terminate();
														if (event.data.success) {
															resolve(event.data);
														} else {
															reject(new Error(event.data.error));
														}
													};

													worker.onerror = (error) => {
														worker.terminate();
														reject(error);
													};
												});

												workerPromises.push(workerPromise);
											}

											console.log(`🚀 [REAL-PARALLEL] Ожидание завершения ${workerPromises.length} воркеров...`);

											// Ожидаем завершения всех воркеров ПАРАЛЛЕЛЬНО!
											const results = await Promise.all(workerPromises);
											const parallelTime = performance.now() - startTime;

											console.log('🚀 [REAL-PARALLEL] Все воркеры завершили работу!');

											// Подсчитываем статистику
											let processedImages = 0;
											let totalSize = 0;
											let maxWorkerTime = 0;
											let minWorkerTime = Infinity;

											results.forEach((result: any) => {
												processedImages += result.imageCount;
												totalSize += result.totalSize;
												maxWorkerTime = Math.max(maxWorkerTime, result.functionTime);
												minWorkerTime = Math.min(minWorkerTime, result.functionTime);
												console.log(`✅ Воркер ${result.workerId}: ${result.imageCount} изображений за ${result.functionTime}мс`);
											});

											console.log(`🚀 [REAL-PARALLEL] ПАРАЛЛЕЛЬНОЕ время: ${Math.round(parallelTime)}мс`);
											console.log(`🚀 [REAL-PARALLEL] Максимальное время воркера: ${maxWorkerTime}мс`);

											// Расчитываем ускорение
											const expectedSequentialTime = 235000; // Ожидаемое последовательное время для 5000 изображений (~47мс * 5000)
											const speedup = expectedSequentialTime / parallelTime;

											alert(`🚀 РЕАЛЬНЫЙ ПАРАЛЛЕЛИЗМ РАБОТАЕТ!\n\n🔥 ${numWorkers} воркеров параллельно\n📊 ${processedImages} изображений, ${(totalSize / 1024 / 1024).toFixed(1)}MB\n\n⏱️ Параллельное время: ${Math.round(parallelTime)}мс\n⏱️ Макс воркер: ${maxWorkerTime}мс\n⏱️ Мин воркер: ${minWorkerTime}мс\n\n🚀 УСКОРЕНИЕ: ${speedup.toFixed(1)}x!\n\nВот это НАСТОЯЩИЙ параллелизм!`);

										} catch (error) {
											console.error('❌ [REAL-PARALLEL] Ошибка:', error);
											alert('Ошибка параллельной обработки: ' + error);
										}
									}}
									className="bg-gradient-to-r from-green-600 to-blue-600 hover:from-green-700 hover:to-blue-700 text-white font-bold py-3 px-6 text-lg rounded-lg shadow-lg transform hover:scale-105 transition-all"
								>
									🚀 ПАРАЛЛЕЛЬНО: 50 воркеров × 100 изображений
								</Button>
							</div>
						</div>
					</div>
				</div>
			</div>

			{/* Обычная кнопка */}
			<Button
				onClick={handleGenerateDocument}
				disabled={!excelViewData || excelViewData.length === 0 || documentGeneration.loading}
				className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
			>
				{documentGeneration.loading ? 'Генерация...' : 'Получить документ'}
			</Button>

			{documentGeneration.loading &&
				<div className="mt-2">
					<div className="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-500"></div>
				</div>
			}
		</div>
	);
};