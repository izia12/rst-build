// Простой тестовый Web Worker для экспериментов
// Проверяет базовую работоспособность Workers

console.log('🧪 [TEST-WORKER] Worker запущен');

self.onmessage = (event) => {
	console.log('🧪 [TEST-WORKER] Получено сообщение:', event.data);

	const { command, data } = event.data;

	if (command === 'test') {
		// Простая обработка
		const result = `Обработано: ${data} (время: ${new Date().toLocaleTimeString()})`;

		// Имитируем небольшую работу
		const start = performance.now();
		let sum = 0;
		for (let i = 0; i < 1000000; i++) {
			sum += i;
		}
		const processingTime = Math.round(performance.now() - start);

		console.log('🧪 [TEST-WORKER] Обработка завершена за:', processingTime, 'мс');

		// Отправляем результат обратно
		self.postMessage({
			success: true,
			result: result,
			processingTime: processingTime,
			sum: sum
		});
	} else {
		console.error('🧪 [TEST-WORKER] Неизвестная команда:', command);
		self.postMessage({
			success: false,
			error: 'Неизвестная команда: ' + command
		});
	}
};

console.log('🧪 [TEST-WORKER] Worker готов к работе');