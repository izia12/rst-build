
self.onmessage = async (event) => {
	const { workerId, startIndex, imageCount, complexity } = event.data;

	try {
		const totalStart = performance.now();

		const wasmModule = await import('../assets/pkg/rst_build.js');
		if (typeof wasmModule.default === 'function') {
			await wasmModule.default();
		}

		if (typeof wasmModule.create_partial_images === 'function') {
			const funcStart = performance.now();
			const imagesArray = await wasmModule.create_partial_images(startIndex, imageCount, complexity);
			const funcTime = Math.round(performance.now() - funcStart);
			const images: Uint8Array[] = [];
			let totalSize = 0;

			for (let i = 0; i < imagesArray.length; i++) {
				const uint8Array = imagesArray[i] as Uint8Array;
				const imageBytes = new Uint8Array(uint8Array);
				images.push(imageBytes);
				totalSize += imageBytes.length;
			}

			self.postMessage({
				success: true,
				workerId: workerId,
				functionTime: funcTime,
				totalTime: Math.round(performance.now() - totalStart),
				imageCount: images.length,
				totalSize: totalSize,
				images: images,
				startIndex: startIndex
			});
		} else {
			throw new Error('Функция create_partial_images недоступна');
		}

	} catch (error: unknown) {
		self.postMessage({
			success: false,
			workerId: workerId,
			error: (error as Error).message || String(error)
		});
	}
};