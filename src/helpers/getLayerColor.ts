export function getLayerColor(layer: string, type: string = "LINE"): string {
	let hash = 0;
	for (let i = 0; i < layer.length; i++) {
		hash = layer.charCodeAt(i) + ((hash << 5) - hash);
	}
	const color = Math.abs(hash) % 0xffffff;

	// Для граней делаем цвет немного светлее
	if (type === "3DFACE") {
		return '#' + Math.min(color + 0x333333, 0xffffff).toString(16).padStart(6, '0');
	}

	return '#' + color.toString(16).padStart(6, '0');
} 