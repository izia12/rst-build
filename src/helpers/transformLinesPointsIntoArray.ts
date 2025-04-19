import * as THREE from 'three';
export type LinesType = Array<number>
export function transformLinesPointsIntoArray(type: "LINES" | "3DFACES" | "TRIANGLE_FACES", lines: LinesType, color: string): THREE.LineSegments | undefined {

	if (type === "LINES") {
		const validLines: number[] = [];
		const skippedLines: Array<{ index: number, values: number[], reason: string }> = [];

		for (let i = 0; i < lines.length; i += 6) {
			const currentLine = lines.slice(i, i + 6);

			// Проверяем каждую координату
			const invalidCoords = currentLine.map((coord, idx) => {
				if (typeof coord !== 'number') return `coord${idx} is not a number`;
				if (isNaN(coord)) return `coord${idx} is NaN`;
				if (!isFinite(coord)) return `coord${idx} is not finite`;
				return null;
			}).filter(Boolean);

			if (invalidCoords.length > 0) {
				skippedLines.push({
					index: i / 6,
					values: currentLine,
					reason: invalidCoords.join(', ')
				});
				continue;
			}

			// Проверяем, не являются ли все координаты нулевыми
			const isZeroLine = currentLine.every(coord => coord === 0);
			if (isZeroLine) {
				skippedLines.push({
					index: i / 6,
					values: currentLine,
					reason: 'All coordinates are zero'
				});
				continue;
			}

			validLines.push(...currentLine);
		}

		if (validLines.length === 0) {
			//console.warn(`No valid lines found for layer ${color}`);
			return undefined;
		}

		const vertices = new Float32Array(validLines);
		const lineMaterial = new THREE.LineBasicMaterial({
			color,
			linewidth: 1,
			transparent: false,
			depthWrite: true,
			depthTest: true
		});
		const lineGeometry = new THREE.BufferGeometry();
		lineGeometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
		return new THREE.LineSegments(lineGeometry, lineMaterial);
	}
	else if (type === "3DFACES") {
		// console.log('Processing 3DFACE');
		const validFaces: number[] = [];
		const skippedFaces: Array<{ index: number, values: number[], reason: string }> = [];
		for (let i = 0; i < lines.length; i += 12) {
			const currentFace = lines.slice(i, i + 12);

			// Проверяем длину грани
			if (currentFace.length < 12) {
				skippedFaces.push({
					index: i / 12,
					values: currentFace,
					reason: `Incomplete face: got ${currentFace.length} coordinates, expected 12`
				});
				continue;
			}

			// Проверяем каждую координату
			const invalidCoords = currentFace.map((coord, idx) => {
				if (typeof coord !== 'number') return `coord${idx} is not a number`;
				if (isNaN(coord)) return `coord${idx} is NaN`;
				if (!isFinite(coord)) return `coord${idx} is not finite`;
				return null;
			}).filter(Boolean);

			if (invalidCoords.length > 0) {
				skippedFaces.push({
					index: i / 12,
					values: currentFace,
					reason: invalidCoords.join(', ')
				});
				continue;
			}

			// Проверяем, не является ли грань вырожденной (все точки совпадают)
			const points = [
				{ x: currentFace[0], y: currentFace[1], z: currentFace[2] },
				{ x: currentFace[3], y: currentFace[4], z: currentFace[5] },
				{ x: currentFace[6], y: currentFace[7], z: currentFace[8] },
				{ x: currentFace[9], y: currentFace[10], z: currentFace[11] }
			];

			const isDegenerateFace = points.every(p1 =>
				points.every(p2 =>
					Math.abs(p1.x - p2.x) < 0.0001 &&
					Math.abs(p1.y - p2.y) < 0.0001 &&
					Math.abs(p1.z - p2.z) < 0.0001
				)
			);

			if (isDegenerateFace) {
				skippedFaces.push({
					index: i / 12,
					values: currentFace,
					reason: 'Degenerate face (all points are the same)'
				});
				continue;
			}
			// Проверяем, что грань не является линией (все точки не лежат на одной прямой)
			const isLine = points.every((p1, i) => {
				if (i === 0) return true;
				const p0 = points[0];
				const dx = p1.x - p0.x;
				const dy = p1.y - p0.y;
				const dz = p1.z - p0.z;
				return Math.abs(dx) < 0.0001 && Math.abs(dy) < 0.0001 && Math.abs(dz) < 0.0001;
			});

			if (isLine) {
				skippedFaces.push({
					index: i / 12,
					values: currentFace,
					reason: 'Face is actually a line (all points are collinear)'
				});
				continue;
			}

			validFaces.push(...currentFace);
		}

		if (validFaces.length === 0) {
			// console.warn(`No valid faces found for layer ${color}`);
			return undefined;
		}

		const vertices = new Float32Array(validFaces);
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));

		// Создаем индексы для линий, образующих четырехугольник
		const indices: number[] = [];
		for (let i = 0; i < validFaces.length / 12; i++) {
			const baseIndex = i * 4;
			// Соединяем точки в замкнутый четырехугольник
			indices.push(
				baseIndex, baseIndex + 1,     // первая линия
				baseIndex + 1, baseIndex + 2, // вторая линия
				baseIndex + 2, baseIndex + 3, // третья линия
				baseIndex + 3, baseIndex      // замыкающая линия
			);
		}

		geometry.setIndex(indices);

		// Используем LineBasicMaterial вместо MeshPhongMaterial
		const material = new THREE.LineBasicMaterial({
			color,
			linewidth: 1  // К сожалению, в WebGL толщина линии всегда будет 1
		});

		return new THREE.LineSegments(geometry, material);
	}
	else if(type ==="TRIANGLE_FACES"){
		// console.log(lines,"Lines")
		const validFaces: number[] = [];
		const skippedFaces: Array<{ index: number, values: number[], reason: string }> = [];
		for (let i = 0; i < lines.length; i += 9) {
			const currentFace = lines.slice(i, i + 9);

			// Проверяем длину грани
			if (currentFace.length < 9) {
				skippedFaces.push({
					index: i / 9,
					values: currentFace,
					reason: `Incomplete face: got ${currentFace.length} coordinates, expected 12`
				});
				continue;
			}

			// Проверяем каждую координату
			const invalidCoords = currentFace.map((coord, idx) => {
				if (typeof coord !== 'number') return `coord${idx} is not a number`;
				if (isNaN(coord)) return `coord${idx} is NaN`;
				if (!isFinite(coord)) return `coord${idx} is not finite`;
				return null;
			}).filter(Boolean);

			if (invalidCoords.length > 0) {
				skippedFaces.push({
					index: i / 9,
					values: currentFace,
					reason: invalidCoords.join(', ')
				});
				continue;
			}

			// Проверяем, не является ли грань вырожденной (все точки совпадают)
			const points = [
				{ x: currentFace[0], y: currentFace[1], z: currentFace[2] },
				{ x: currentFace[3], y: currentFace[4], z: currentFace[5] },
				{ x: currentFace[6], y: currentFace[7], z: currentFace[8] },
			];

			const isDegenerateFace = points.every(p1 =>
				points.every(p2 =>
					Math.abs(p1.x - p2.x) < 0.0001 &&
					Math.abs(p1.y - p2.y) < 0.0001 &&
					Math.abs(p1.z - p2.z) < 0.0001
				)
			);

			if (isDegenerateFace) {
				skippedFaces.push({
					index: i / 9,
					values: currentFace,
					reason: 'Degenerate face (all points are the same)'
				});
				continue;
			}

			validFaces.push(...currentFace);
		}

		if (validFaces.length === 0) {
			return undefined;
		}
		const vertices = new Float32Array(validFaces);
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));

		// Создаем индексы для линий, образующих четырехугольник
		const indices: number[] = [];
		for (let i = 0; i < validFaces.length / 9; i++) {
			const baseIndex = i * 3;
			// Соединяем точки в замкнутый четырехугольник
			indices.push(
				baseIndex, baseIndex + 1,     // первая линия
				baseIndex + 1, baseIndex + 2, // вторая линия
				baseIndex + 2, baseIndex   // третья замыкающая линия
			);
		}

		geometry.setIndex(indices);

		// Используем LineBasicMaterial вместо MeshPhongMaterial
		const material = new THREE.LineBasicMaterial({
			color,
			linewidth: 1  // К сожалению, в WebGL толщина линии всегда будет 1
		});

		return new THREE.LineSegments(geometry, material);
	}
}