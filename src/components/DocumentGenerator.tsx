import React from 'react';
import { useAppSelector } from '../store/store';
import { useAppDispatch } from '../store/store';
import { generateDocumentWithColorPalette } from '../store/slices/thunks/wasmThanks';
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

	// Параллельная генерация документов по этажам
	const handleGenerateDocumentsParallel = async () => {
		// Получаем готовые данные из GLOBAL_ENTITIES
		const wasmModule = await import('../assets/pkg/rst_build.js');
		if (typeof wasmModule.default === 'function') {
			await wasmModule.default();
		}

		// Экспортируем обработанные данные из GLOBAL_ENTITIES
		const processedDataJson = wasmModule.get_processed_data_for_frontend();
		const processedData = JSON.parse(processedDataJson);

		// Собираем комбинации из excelViewData
		const selectedCombinations: SelectedCombination[] = [];
		excelViewData.forEach(floor => {
			floor.values.forEach(armatureCombination => {
				armatureCombination.combinations.forEach(item => {
					selectedCombinations.push({
						floor_level: floor.level,
						function_name: armatureCombination.function_name,
						as_target_value: armatureCombination.as_target_value,
						combination: {
							main_diameter: item.main_diameter,
							additional_diameter: item.additional_diameter,
							total_area: item.total_area,
							deviation: item.deviation,
							result_scale: item.result_scale,
							is_min_deviation: item.is_min_deviation || false,
							is_default_checked: item.is_default_checked
						}
					});
				});
			});
		});

		// Используем этажи из excelViewData
		const uniqueFloors = [...new Set(selectedCombinations.map(c => c.floor_level))];
		
		// Группируем комбинации по этажам
		const floorCombinations: { [key: string]: SelectedCombination[] } = {};
		uniqueFloors.forEach(floor => {
			floorCombinations[floor] = selectedCombinations.filter(c => c.floor_level === floor);
		});

		if (selectedCombinations.length === 0) {
			alert('Выберите хотя бы одну комбинацию');
			return;
		}

		try {

			// Создаем все воркеры параллельно
			const allPromises = uniqueFloors.map((floorLevel, index) => {
				const currentFloorCombinations = floorCombinations[floorLevel] || [];

				return new Promise<{ success: boolean; floorLevel: string; docxData?: Uint8Array; docxSize?: number; error?: string }>((resolve, reject) => {
					const worker = new Worker(
						new URL('../workers/parallel-document-worker.ts', import.meta.url),
						{ type: 'module' }
					);

					// Отправляем данные воркеру
					worker.postMessage({
						workerId: index + 1,
						floorLevel,
						floorData: processedData,
						floorCombinations: currentFloorCombinations
					});

					worker.onmessage = (event) => {
						worker.terminate();
						resolve(event.data);
					};

					worker.onerror = (error) => {
						worker.terminate();
						reject(error);
					};
				});
			});

			// Ждем завершения всех воркеров одновременно
			const results = await Promise.all(allPromises);
			// Обрабатываем результаты
			for (const result of results) {
				if (result.success) {

					// Скачиваем файл для этажа
					const blob = new Blob([result.docxData], {
						type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document'
					});
					const url = URL.createObjectURL(blob);
					const link = document.createElement('a');
					link.href = url;
					link.download = `floor_${result.floorLevel}_${new Date().toISOString().slice(0, 10)}.docx`;
					document.body.appendChild(link);
					link.click();
					document.body.removeChild(link);
					URL.revokeObjectURL(url);

					// Небольшая задержка между скачиваниями
					await new Promise(resolve => setTimeout(resolve, 100));
				}
			}
		} catch (error) {
			alert('Ошибка при параллельной генерации документов: ' + error);
		}
	};

	return (
		<div className="space-y-4">
			{/* Кнопки генерации */}
			<div className="flex gap-4">
				{/* Обычная кнопка */}
				<Button
					onClick={handleGenerateDocument}
					disabled={!excelViewData || excelViewData.length === 0 || documentGeneration.loading}
					className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
				>
					{documentGeneration.loading ? 'Генерация...' : 'Получить документ'}
				</Button>

				{/* Параллельная генерация документов */}
				<Button
					onClick={handleGenerateDocumentsParallel}
					disabled={!excelViewData || excelViewData.length === 0 || documentGeneration.loading}
					className="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
				>
					🚀 Параллельно по этажам
				</Button>
			</div>

			{documentGeneration.loading &&
				<div className="mt-2">
					<div className="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-500"></div>
				</div>
			}
		</div>
	);
};