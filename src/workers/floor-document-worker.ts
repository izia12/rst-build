// 🏗️ ПАРАЛЛЕЛЬНЫЙ ВОРКЕР ДЛЯ ГЕНЕРАЦИИ ДОКУМЕНТОВ ЭТАЖЕЙ
// Генерирует DOCX для одного этажа с выбранными комбинациями в отдельном потоке

interface FloorWorkerMessage {
	workerId: number;
	floorLevel: string;
	floorCombinations: SelectedCombination[];
	title: string;
	// Добавляем данные для парсинга
	sliData: string;
	txtData: string;
	xlsxData: ArrayBuffer; // Используем ArrayBuffer для передачи бинарных данных
}

interface SelectedCombination {
	floor_level: string;
	function_name: string;
	as_target_value: number;
	combination: {
		main_diameter: number;
		additional_diameter: number;
		total_area: number;
		deviation: number;
		result_scale?: string;
		is_default_checked: boolean;
	}
}

interface FloorWorkerResponse {
	success: boolean;
	workerId: number;
	floorLevel: string;
	docxData?: Uint8Array;
	docxSize?: number;
	functionTime?: number;
	totalTime?: number;
	combinationCount?: number;
	error?: string;
}

self.onmessage = async (event: MessageEvent<FloorWorkerMessage>) => {
	const { workerId, floorLevel, floorCombinations, sliData, txtData, xlsxData } = event.data;

	// Проверяем что все необходимые данные переданы
	if (!workerId || !floorLevel || !floorCombinations || !Array.isArray(floorCombinations)) {
		console.error(`❌ [FLOOR-WORKER-${workerId || 'unknown'}] Неверные данные:`, {
			workerId, floorLevel,
			floorCombinationsType: typeof floorCombinations,
			floorCombinationsLength: floorCombinations ? floorCombinations.length : 'undefined'
		});
		self.postMessage({
			success: false,
			workerId: workerId || 0,
			floorLevel: floorLevel || 'unknown',
			error: 'Неверные входные данные для воркера'
		});
		return;
	}

	try {

		const wasmModule = await import('../assets/pkg/rst_build.js');

		if (typeof wasmModule.default === 'function') {
			await wasmModule.default();
		}
		
		// Парсим данные в контексте воркера
		if (sliData && txtData && xlsxData) {
			const uint8Array = new Uint8Array(xlsxData);
			wasmModule.parse_data(sliData, txtData, uint8Array);
		} else {
			console.warn(`⚠️ [FLOOR-WORKER-${workerId}] Недостаточно данных для парсинга:`, {
				hasSliData: !!sliData,
				hasTxtData: !!txtData,
				hasXlsxData: !!xlsxData
			});
			// Если данные не переданы, возвращаем ошибку
			throw new Error('Необходимые данные для парсинга отсутствуют в воркере');
		}
		// Попробуем сначала новую функцию, если она доступна
		if (typeof wasmModule.create_docx_for_single_floor === 'function') {
			// Подготавливаем данные для этажа в формате для новой оптимизированной функции
			const selectedData = {
				combinations: floorCombinations,
				floors: [floorLevel]
			};

			const selectedCombinationsJson = JSON.stringify(selectedData);



			// Генерируем DOCX для этого этажа используя оптимизированную функцию
			// Передаем пустую строку как floor_data_json, так как данные уже загружены через parse_data
			const docxData = await wasmModule.create_docx_for_single_floor(floorLevel, "", selectedCombinationsJson);

			const response: FloorWorkerResponse = {
				success: true,
				workerId: workerId,
				floorLevel: floorLevel,
				docxData: new Uint8Array(docxData),
				docxSize: docxData.length,
				combinationCount: floorCombinations.length
			};

			self.postMessage(response);

		} else if (typeof wasmModule.create_docx_with_selected_combinations === 'function') {

			// FALLBACK: Используем старую функцию
			const selectedData = {
				combinations: floorCombinations,
				floors: [floorLevel]
			};

			const selectedCombinationsJson = JSON.stringify(selectedData);

			// Генерируем DOCX для этого этажа
			const docxData = await wasmModule.create_docx_with_selected_combinations(selectedCombinationsJson);
			const response: FloorWorkerResponse = {
				success: true,
				workerId: workerId,
				floorLevel: floorLevel,
				docxData: new Uint8Array(docxData),
				docxSize: docxData.length,
				combinationCount: floorCombinations.length
			};

			self.postMessage(response);

		} else {
			throw new Error('Функция create_docx_with_selected_combinations недоступна в worker');
		}

	} catch (error: unknown) {
		const errorMessage = error instanceof Error ? error.message : String(error);
		console.error(`❌ [FLOOR-WORKER-${workerId}] Ошибка обработки этажа ${floorLevel}:`, error);
		console.error(`❌ [FLOOR-WORKER-${workerId}] Подробности ошибки:`, {
			errorType: typeof error,
			errorName: error instanceof Error ? error.name : 'Unknown',
			errorMessage: errorMessage,
			errorStack: error instanceof Error ? error.stack : 'No stack trace'
		});

		const response: FloorWorkerResponse = {
			success: false,
			workerId: workerId,
			floorLevel: floorLevel,
			error: `Ошибка воркера: ${errorMessage}`
		};

		self.postMessage(response);
	}
};