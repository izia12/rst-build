export async function loadDXF(event: React.ChangeEvent<HTMLInputElement>) {
	const file = (event.target as HTMLInputElement).files?.[0];
	return new Promise((resolve, reject) => {
		if (!file) {
			return reject(new Error("Файл не выбран."));
		}

		const reader = new FileReader();
		reader.onload = function (e) {
			resolve(e.target?.result);
		};
		reader.onerror = function (e) {
			reject(new Error("Ошибка чтения файла: " + e.target?.error?.code));
		};
		reader.readAsText(file);
	});
}