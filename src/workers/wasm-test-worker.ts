// Worker для тестирования WASM внутри Web Worker
// Попытается загрузить и использовать WASM модуль

console.log('🧪 [WASM-WORKER] Worker запущен');

self.onmessage = async (event) => {
	console.log('🧪 [WASM-WORKER] Получено сообщение:', event.data);

	const { command } = event.data;

	if (command === 'test_new_module') {
		// 🧪 НОВЫЙ ТЕСТ: Тестирование нового модуля в Worker
		try {
			console.log('🧪 [WASM-WORKER] Тестирование нового тестового модуля в Worker...');
			const totalStart = performance.now();

			const wasmModule = await import('../assets/pkg/rst_build.js');

			// Инициализация
			if (typeof wasmModule.default === 'function') {
				await wasmModule.default();
			}

			console.log('🧪 [WASM-WORKER] Вызов новой тестовой функции...');

			if (typeof wasmModule.create_test_docx_with_images === 'function') {
				const funcStart = performance.now();

				// Вызываем новую тестовую функцию
				const result = await wasmModule.create_test_docx_with_images(3, 2); // 3 картинки, сложность 2
				const funcTime = Math.round(performance.now() - funcStart);
				const totalTime = Math.round(performance.now() - totalStart);

				console.log('🧪 [WASM-WORKER] Новая функция выполнена за:', funcTime, 'мс');
				console.log('🧪 [WASM-WORKER] Размер DOCX:', result.length, 'байт');

				// Отправляем успешный результат
				self.postMessage({
					success: true,
					functionTime: funcTime,
					totalTime: totalTime,
					docxSize: result.length,
					docxData: result,
					message: 'Новый тестовый модуль успешно работает в Worker!'
				});
			} else {
				throw new Error('Функция create_test_docx_with_images недоступна (нужна пересборка WASM)');
			}

		} catch (error: unknown) {
			console.error('🧪 [WASM-WORKER] Ошибка нового модуля:', error);

			self.postMessage({
				success: false,
				error: (error as Error).message || String(error),
				details: 'Новая функция не найдена - нужна пересборка WASM'
			});
		}
	} else if (command === 'test_heavy_module') {
		// 🔥 ТЯЖЕЛЫЙ ТЕСТ: Тестирование с большой нагрузкой
		try {
			const { imageCount = 50, complexity = 100 } = event.data;
			console.log(`🔥 [WASM-WORKER] Тяжелый тест: ${imageCount} изображений, ${imageCount * complexity * 10} фигур...`);
			const totalStart = performance.now();

			const wasmModule = await import('../assets/pkg/rst_build.js');

			// Инициализация
			if (typeof wasmModule.default === 'function') {
				await wasmModule.default();
			}

			console.log(`🔥 [WASM-WORKER] Вызов тяжелой функции ${imageCount}x${complexity}...`);

			if (typeof wasmModule.create_test_docx_with_images === 'function') {
				const funcStart = performance.now();

				// Вызываем с параметрами из UI
				const result = await wasmModule.create_test_docx_with_images(imageCount, complexity);
				const funcTime = Math.round(performance.now() - funcStart);
				const totalTime = Math.round(performance.now() - totalStart);

				console.log(`🔥 [WASM-WORKER] Тяжелая функция выполнена за: ${funcTime} мс`);
				console.log(`🔥 [WASM-WORKER] Размер DOCX: ${result.length} байт`);

				// Отправляем успешный результат
				self.postMessage({
					success: true,
					functionTime: funcTime,
					totalTime: totalTime,
					docxSize: result.length,
					docxData: result,
					imageCount: imageCount,
					complexity: complexity,
					totalShapes: imageCount * complexity * 10,
					message: `Тяжелый тест успешно выполнен в Worker!`
				});
			} else {
				throw new Error('Функция create_test_docx_with_images недоступна (нужна пересборка WASM)');
			}

		} catch (error: unknown) {
			console.error('🔥 [WASM-WORKER] Ошибка тяжелого теста:', error);

			self.postMessage({
				success: false,
				error: (error as Error).message || String(error),
				details: 'Ошибка выполнения тяжелого теста'
			});
		}
	} else if (command === 'test_wasm') {
		// СТАРЫЙ ТЕСТ: Тестирование существующих функций
		try {
			console.log('🧪 [WASM-WORKER] Попытка загрузки WASM модуля...');
			const loadStart = performance.now();

			const wasmModule = await import('../assets/pkg/rst_build.js');
			const loadTime = Math.round(performance.now() - loadStart);

			console.log('🧪 [WASM-WORKER] WASM модуль загружен за:', loadTime, 'мс');

			// Инициализация WASM
			console.log('🧪 [WASM-WORKER] Попытка инициализации WASM...');
			const initStart = performance.now();

			if (typeof wasmModule.default === 'function') {
				await wasmModule.default();
				console.log('🧪 [WASM-WORKER] WASM инициализирован через default()');
			} else {
				console.log('🧪 [WASM-WORKER] Функция инициализации не найдена, продолжаем...');
			}

			const initTime = Math.round(performance.now() - initStart);

			// Попытка вызова функции
			if (typeof wasmModule.convert_data_to_js_order_byz === 'function') {
				console.log('🧪 [WASM-WORKER] Вызов функции WASM...');
				const funcStart = performance.now();

				const result = wasmModule.convert_data_to_js_order_byz();
				const funcTime = Math.round(performance.now() - funcStart);

				console.log('🧪 [WASM-WORKER] Функция выполнена за:', funcTime, 'мс');
				console.log('🧪 [WASM-WORKER] Размер результата:', result.length, 'символов');

				// Отправляем результат
				self.postMessage({
					success: true,
					loadTime: loadTime,
					initTime: initTime,
					executionTime: funcTime,
					dataSize: result.length,
					resultPreview: result.substring(0, 200),
					message: result.length > 100 ? 'WASM успешно работает в Worker!' : 'WASM работает, но данные не загружены'
				});
			} else {
				throw new Error('Функция convert_data_to_js_order_byz недоступна');
			}

		} catch (error: unknown) {
			console.error('🧪 [WASM-WORKER] Ошибка загрузки/выполнения WASM:', error);

			self.postMessage({
				success: false,
				error: (error as Error).message || String(error),
				details: (error as Error).stack || 'Нет подробностей'
			});
		}
	} else {
		console.error('🧪 [WASM-WORKER] Неизвестная команда:', command);
		self.postMessage({
			success: false,
			error: 'Неизвестная команда: ' + command
		});
	}
};

console.log('🧪 [WASM-WORKER] Worker готов к тестированию WASM');