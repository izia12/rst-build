// 🏗️ ПАРАЛЛЕЛЬНЫЙ ВОРКЕР ДЛЯ ГЕНЕРАЦИИ ДОКУМЕНТОВ ПО ЭТАЖАМ
// Принимает готовые данные из GLOBAL_ENTITIES и генерирует документ для одного этажа
// НОВАЯ АРХИТЕКТУРА: Без парсинга, только генерация

interface DocumentWorkerMessage {
	workerId: number;
	floorLevel: string;
	floorData: unknown[];           // ← Готовые данные этажа из GLOBAL_ENTITIES
	floorCombinations: SelectedCombination[];   // ← Комбинации для этого этажа
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
	};
}

interface DocumentWorkerResponse {
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

self.onmessage = async (event: MessageEvent<DocumentWorkerMessage>) => {
	const { workerId, floorLevel, floorData, floorCombinations } = event.data;

	try {
		

		const totalStart = performance.now();

		// Загружаем WASM модуль (как в parallel-image-worker)
		const wasmModule = await import('../assets/pkg/rst_build.js');
		if (typeof wasmModule.default === 'function') {
			await wasmModule.default();
		}

		// Проверяем доступность новой функции
		if (typeof wasmModule.create_docx_for_single_floor === 'function') {
			const funcStart = performance.now();

			// Подготавливаем данные для новой функции
			const floorDataJson = JSON.stringify(floorData);
			const combinationsJson = JSON.stringify(floorCombinations);

	

			// Вызываем НОВУЮ функцию для генерации одного этажа
			const docxData = await wasmModule.create_docx_for_single_floor(
				floorLevel,
				floorDataJson,
				combinationsJson
			);

			const funcTime = Math.round(performance.now() - funcStart);
			const totalTime = Math.round(performance.now() - totalStart);

			// ТОЛЬКО ВРЕМЯ! (как в parallel-image-worker)
	

			const response: DocumentWorkerResponse = {
				success: true,
				workerId: workerId,
				floorLevel: floorLevel,
				docxData: new Uint8Array(docxData),
				docxSize: docxData.length,
				functionTime: funcTime,
				totalTime: totalTime,
				combinationCount: floorCombinations.length
			};

			self.postMessage(response);
		} else {
			throw new Error('Функция create_docx_for_single_floor недоступна');
		}

	} catch (error: unknown) {
		console.error(`❌ [FLOOR-WORKER-${workerId}] Ошибка обработки этажа ${floorLevel}:`, error);

		const response: DocumentWorkerResponse = {
			success: false,
			workerId: workerId,
			floorLevel: floorLevel,
			error: (error as Error).message || String(error)
		};

		self.postMessage(response);
	}
};