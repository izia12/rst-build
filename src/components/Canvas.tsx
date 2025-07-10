import React, { memo, useEffect, useMemo, useRef, useState } from "react";
import { useAppSelector } from "../store/store";
import { Layer, Stage, Shape, Text } from "react-konva";
import init, { get_canvas_statistics_wasm, get_optimized_canvas_data_wasm } from "../assets/pkg/rst_build";
// import  { get_optimized_canvas_data_wasm, get_canvas_statistics_wasm } from "../assets/pkg";

interface CanvasData {
	levels: LevelData[];
	total_shapes: number;
	visible_levels: number;
	total_levels: number;
}

interface LevelData {
	z: number;
	shapes: ShapeData[];
	shape_count: number;
}

interface ShapeData {
	vertices: Array<{ x: number, y: number, z: number }>;
	as_values: {
		as1: number[];
		as2: number[];
		as3: number[];
		as4: number[];
	};
	entity_type: string;
}

interface CanvasStatistics {
	total_levels: number;
	total_entities: number;
	z_range: {
		min: number;
		max: number;
	};
	levels_info: Array<{
		z: number;
		entity_count: number;
	}>;
}

export const Canvas = memo(() => {
	const data = useAppSelector((state) => state.wasm.wasmData);

	// Основные состояния
	const [scale, setScale] = useState(1);
	const [asFn, setAsFn] = useState<"as1" | "as2" | "as3" | "as4">("as1");

	// WASM состояния
	const [useOptimization, setUseOptimization] = useState(false);
	const [optimizedData, setOptimizedData] = useState<CanvasData | null>(null);
	const [canvasStats, setCanvasStats] = useState<CanvasStatistics | null>(null);
	const [isLoading, setIsLoading] = useState(false);

	// Параметры оптимизации
	const [maxShapesPerLevel, setMaxShapesPerLevel] = useState(100);
	const [maxTotalShapes, setMaxTotalShapes] = useState(1000);
	const [zMin, setZMin] = useState<number | null>(null);
	const [zMax, setZMax] = useState<number | null>(null);
    // Добавляем состояния для адаптивных размеров
    const [stageSize, setStageSize] = useState({ width: 800, height: 600 });
    const containerRef = useRef<HTMLDivElement>(null);
    
    // Хук для отслеживания размеров контейнера
    useEffect(() => {
        const updateSize = () => {
            if (containerRef.current) {
                const containerWidth = containerRef.current.offsetWidth;
                const isMobile = window.innerWidth < 768;
                
                // Адаптивные размеры
                const width = Math.min(containerWidth - 32, isMobile ? containerWidth - 16 : 800);
                const height = isMobile ? width * 0.75 : width * 0.75; // Соотношение 4:3
                
                setStageSize({ width, height });
            }
        };
        
        updateSize();
        window.addEventListener('resize', updateSize);
        return () => window.removeEventListener('resize', updateSize);
    }, []);
	// Палитра цветов
	const colorPalette = [
		'#FFFFCC', '#FFEDA0', '#FED976', '#FEB24C', '#FD8D3C', '#FC4E2A', '#E31A1C'
	];

	// Обработка обычных данных
	const { zMap, minAs1, maxAs1 } = useMemo(() => {
		const map = new Map<number, Array<{
			points: number[][];
			as1Value: number;
			center: [number, number];
			area: number;
		}>>();

		let min = Infinity;
		let max = -Infinity;

		data.forEach(item => {
			if (!item.vertices || item.vertices.length === 0) return;

			const firstZ = item.vertices[0].z;
			const allSameZ = item.vertices.every(v => v.z === firstZ);
			if (!allSameZ) return;

			const as1Value = item.row?.[`${asFn}`]?.[0] ?? 0;

			if (as1Value < min) min = as1Value;
			if (as1Value > max) max = as1Value;

			const sumX = item.vertices.reduce((acc, v) => acc + v.x, 0);
			const sumY = item.vertices.reduce((acc, v) => acc + v.y, 0);
			const center: [number, number] = [
				sumX / item.vertices.length,
				sumY / item.vertices.length
			];

			let area = 0;
			for (let i = 0; i < item.vertices.length; i++) {
				const j = (i + 1) % item.vertices.length;
				area += item.vertices[i].x * item.vertices[j].y;
				area -= item.vertices[j].x * item.vertices[i].y;
			}
			area = Math.abs(area / 2);

			if (!map.has(firstZ)) map.set(firstZ, []);

			map.get(firstZ)!.push({
				points: item.vertices.map(v => [v.x, v.y]),
				as1Value,
				center,
				area
			});
		});

		return {
			zMap: map,
			minAs1: min === Infinity ? 0 : min,
			maxAs1: max === -Infinity ? 1 : max
		};
	}, [data, asFn]);

	// Функция определения цвета
	const getColorByAs1 = (value: number) => {
		if (minAs1 === maxAs1) return colorPalette[0];
		const position = (value - minAs1) / (maxAs1 - minAs1);
		const colorIndex = Math.min(6, Math.floor(position * 7));
		return colorPalette[colorIndex];
	};

	// Рассчет границ
	const bounds = useMemo(() => {
		let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;

		const currentData = useOptimization && optimizedData ? optimizedData.levels : Array.from(zMap.values());

		if (useOptimization && optimizedData) {
			for (const level of optimizedData.levels) {
				for (const shape of level.shapes) {
					for (const vertex of shape.vertices) {
						if (vertex.x < minX) minX = vertex.x;
						if (vertex.y < minY) minY = vertex.y;
						if (vertex.x > maxX) maxX = vertex.x;
						if (vertex.y > maxY) maxY = vertex.y;
					}
				}
			}
		} else {
			for (const shapes of zMap.values()) {
				for (const shape of shapes) {
					for (const [x, y] of shape.points) {
						if (x < minX) minX = x;
						if (y < minY) minY = y;
						if (x > maxX) maxX = x;
						if (y > maxY) maxY = y;
					}
				}
			}
		}

		const padding = 10;
		const width = maxX - minX;
		const height = maxY - minY;

		const targetRatio = 800 / 600;
		const actualRatio = width / height;

		let adjustedWidth = width;
		let adjustedHeight = height;

		if (actualRatio > targetRatio) {
			adjustedHeight = width / targetRatio;
		} else {
			adjustedWidth = height * targetRatio;
		}

		return {
			minX: minX - padding,
			minY: minY - padding,
			maxX: minX + adjustedWidth + padding,
			maxY: minY + adjustedHeight + padding,
			width: adjustedWidth + padding * 2,
			height: adjustedHeight + padding * 2
		};
	}, [zMap, optimizedData, useOptimization]);

	// Функция нормализации координат
	const normalize = (x: number, y: number, stageWidth: number, stageHeight: number) => {
		const scaleRatio = Math.min(
			stageWidth / bounds.width,
			stageHeight / bounds.height
		);

		return [
			(x - bounds.minX) * scaleRatio,
			(y - bounds.minY) * scaleRatio
		];
	};

	// Функция для центра
	const getNormalizedCenter = (center: [number, number], stageWidth: number, stageHeight: number) => {
		return normalize(center[0], center[1], stageWidth, stageHeight);
	};

	// WASM функции
	const loadOptimizedData = async () => {
		setIsLoading(true);
		try {
			await init();

			// Исправленный вызов get_optimized_canvas_data_wasm (без JSON.stringify)
			const optimized = get_optimized_canvas_data_wasm(
				maxShapesPerLevel,
				maxTotalShapes,
				zMin,
				zMax
			);

			// Исправленный вызов get_canvas_statistics_wasm (без аргументов)
			const stats = get_canvas_statistics_wasm();

			setOptimizedData(JSON.parse(optimized));
			setCanvasStats(JSON.parse(stats));
		} catch (error) {
			console.error('Ошибка загрузки оптимизированных данных:', error);
		} finally {
			setIsLoading(false);
		}
	};

	// Рендер оптимизированных данных
	const renderOptimizedData = () => {
		if (!optimizedData) return null;

		return optimizedData.levels.map((level) => (
			<div key={level.z} className="mb-6 sm:mb-10">
				<h2 className="text-lg sm:text-xl font-bold mb-2 sm:mb-4 text-gray-800">
					Уровень Z = {level.z} ({level.shape_count} фигур)
				</h2>
				<div ref={containerRef} className="w-full overflow-x-auto">
					<Stage
						width={(800 * scale) / 1.5}
						height={(800 * scale) / 1.5}
						className="border border-gray-300 bg-gray-50 rounded mx-auto"
					>
						<Layer scaleX={scale} scaleY={scale}>
							{level.shapes.map((shape, idx) => {
								const asValue = shape.as_values[asFn][0] || 0;
								const color = getColorByAs1(asValue);

								const centerX = shape.vertices.reduce((sum, v) => sum + v.x, 0) / shape.vertices.length;
								const centerY = shape.vertices.reduce((sum, v) => sum + v.y, 0) / shape.vertices.length;
								const [textX, textY] = normalize(centerX, centerY, 800, 600);

								return (
									<React.Fragment key={`opt-${level.z}-${idx}`}>
										<Shape
											sceneFunc={(ctx, konvaShape) => {
												ctx.beginPath();
												const [startX, startY] = normalize(
													shape.vertices[0].x,
													shape.vertices[0].y,
													800,
													600
												);
												ctx.moveTo(startX, startY);
												for (let i = 1; i < shape.vertices.length; i++) {
													const [x, y] = normalize(
														shape.vertices[i].x,
														shape.vertices[i].y,
														800,
														600
													);
													ctx.lineTo(x, y);
												}
												ctx.closePath();
												ctx.fillStrokeShape(konvaShape);
											}}
											fill={color}
											stroke="#333"
											strokeWidth={0.2}
										/>
										<Text
											x={textX}
											y={textY}
											text={asValue.toFixed(1)}
											fontSize={8}
											fill="black"
											align="center"
											verticalAlign="middle"
											offsetX={10}
											offsetY={4}
											scaleX={1 / scale}
											scaleY={1 / scale}
											listening={false}
										/>
									</React.Fragment>
								);
							})}
						</Layer>
					</Stage>
				</div>
			</div>
		));
	};

	// Обработчики зума
	const handleZoomIn = () => setScale(prev => Math.min(prev * 1.2, 5));
	const handleZoomOut = () => setScale(prev => Math.max(prev / 1.2, 0.5));
	const handleResetZoom = () => setScale(1);

	// Рендер обычных данных
	const renderNormalData = () => {
		return Array.from(zMap.entries()).map(([z, shapes]) => (
			<div key={z} className="mb-10">
				<h2 className="text-xl font-bold mb-4 text-gray-800">Уровень Z = {z}</h2>
				<Stage
					width={(800 * scale) / 1.5}
					height={(800 * scale) / 1.5}
					className="border border-gray-300 bg-gray-50 rounded"
				>
					<Layer scaleX={scale} scaleY={scale}>
						{shapes.map((shape, idx) => {
							const color = getColorByAs1(shape.as1Value);
							const [textX, textY] = getNormalizedCenter(shape.center, 800, 600);
							const fontSize = Math.max(6, Math.min(10, Math.sqrt(shape.area) * 1.2));
							let displayValue = shape.as1Value.toFixed(1);
							if (shape.as1Value > 10) displayValue = shape.as1Value.toFixed(0);

							return (
								<React.Fragment key={`${z}-${idx}`}>
									<Shape
										sceneFunc={(ctx, konvaShape) => {
											ctx.beginPath();
											const [startX, startY] = normalize(
												shape.points[0][0],
												shape.points[0][1],
												800,
												600
											);
											ctx.moveTo(startX, startY);
											for (let i = 1; i < shape.points.length; i++) {
												const [x, y] = normalize(
													shape.points[i][0],
													shape.points[i][1],
													800,
													600
												);
												ctx.lineTo(x, y);
											}
											ctx.closePath();
											ctx.fillStrokeShape(konvaShape);
										}}
										fill={color}
										stroke="#333"
										strokeWidth={0.2}
									/>
									{shape.points.length > 2 && (
										<Text
											x={textX}
											y={textY}
											text={displayValue}
											fontSize={fontSize * 1.4}
											fill="black"
											align="center"
											verticalAlign="middle"
											offsetX={displayValue.length * 2}
											offsetY={fontSize / 2}
											scaleX={1 / scale}
											scaleY={1 / scale}
											listening={false}
										/>
									)}
								</React.Fragment>
							);
						})}
					</Layer>
				</Stage>
			</div>
		));
	};

	if (zMap.size === 0 && !optimizedData) {
		return (
			<div className="flex items-center justify-center h-64 text-gray-500">
				Нет данных для отображения
			</div>
		);
	}

	return (
		<div className="p-4 relative">
			{/* Основная панель управления */}
			<div className="flex flex-col lg:flex-row lg:justify-between gap-4 sticky top-9 z-20 mb-6">
				<div className="flex flex-wrap gap-2">
					<button
						className="bg-sky-900 rounded px-4 py-2 text-white hover:bg-sky-800 transition-colors"
						onClick={handleZoomIn}
					>
						Увеличить (+)
					</button>
					<button
						className="bg-sky-900 rounded px-4 py-2 text-white hover:bg-sky-800 transition-colors"
						onClick={handleZoomOut}
					>
						Уменьшить (-)
					</button>
					<button
						className="bg-sky-900 rounded px-4 py-2 text-white hover:bg-sky-800 transition-colors"
						onClick={handleResetZoom}
					>
						Сбросить масштаб
					</button>
					<span className="flex items-center px-3 py-2 bg-gray-100 rounded text-gray-700">
						Масштаб: {Math.round(scale * 100)}%
					</span>
				</div>

				<div className="flex flex-wrap gap-2">
					{["as1", "as2", "as3", "as4"].map((asType) => (
						<button
							key={asType}
							onClick={() => setAsFn(asType as "as1" | "as2" | "as3" | "as4")}
							className={`px-4 py-2 rounded transition-colors ${asFn === asType
									? "bg-sky-900 text-white"
									: "bg-white text-sky-900 border border-sky-900 hover:bg-sky-50"
								}`}
						>
							{asType}
						</button>
					))}
				</div>
			</div>



			{/* Легенда цветов */}
			<div className="mb-6">
				<h3 className="text-lg font-semibold mb-3 text-gray-800">
					Цветовая палитра (значения {asFn})
				</h3>
				<div className="flex flex-wrap gap-3">
					{colorPalette.map((color, index) => {
						const minVal = minAs1 + index * (maxAs1 - minAs1) / 7;
						const maxVal = minAs1 + (index + 1) * (maxAs1 - minAs1) / 7;

						return (
							<div key={index} className="flex items-center gap-2">
								<div
									className="w-8 h-5 border border-gray-400 rounded"
									style={{ backgroundColor: color }}
								/>
								<span className="text-sm text-gray-700">
									{minVal.toFixed(1)} - {maxVal.toFixed(1)}
								</span>
							</div>
						);
					})}
				</div>
			</div>

			{/* Статус режима */}
			{useOptimization && optimizedData && (
				<div className="mb-4 p-3 bg-green-50 border border-green-200 rounded">
					<p className="text-green-800 font-medium">Режим оптимизации WASM активен</p>
					<p className="text-sm text-green-600">
						Показано {optimizedData.total_shapes} фигур из {optimizedData.visible_levels} уровней
					</p>
				</div>
			)}

			{/* Отображение фигур */}
			<div className="space-y-6">
				{useOptimization && optimizedData ? renderOptimizedData() : renderNormalData()}
			</div>
		</div>
	);
});