import React, { useMemo, useState } from "react";
import { useAppSelector } from "../store/store";
import { Layer, Stage, Shape, Text } from "react-konva";

export default function Canvas() {
  const data = useAppSelector((state) => state.wasm.wasmData);
  const [scale, setScale] = useState(1);
  
  // Палитра цветов
  const colorPalette = [
    '#FFFFCC', '#FFEDA0', '#FED976', '#FEB24C', '#FD8D3C', '#FC4E2A', '#E31A1C'
  ];

  // Фильтрация и подготовка данных
  const { zMap, minAs1, maxAs1 } = useMemo(() => {
    const map = new Map<number, Array<{
      points: number[][];
      as1Value: number;
      center: [number, number];
      area: number; // Добавим площадь для определения размера текста
    }>>();
    
    let min = Infinity;
    let max = -Infinity;
    
    data.forEach(item => {
      if (!item.vertices || item.vertices.length === 0) return;
      
      const firstZ = item.vertices[0].z;
      const allSameZ = item.vertices.every(v => v.z === firstZ);
      if (!allSameZ) return;
      
      const as1Value = item.row?.as1?.[0] ?? 0;
      
      if (as1Value < min) min = as1Value;
      if (as1Value > max) max = as1Value;
      
      // Вычисляем центр
      const sumX = item.vertices.reduce((acc, v) => acc + v.x, 0);
      const sumY = item.vertices.reduce((acc, v) => acc + v.y, 0);
      const center: [number, number] = [
        sumX / item.vertices.length,
        sumY / item.vertices.length
      ];
      
      // Вычисляем площадь для определения размера текста
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
  }, [data]);

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
	
	// Добавим минимальный отступ для сохранения пропорций
	const padding = 10;
	const width = maxX - minX;
	const height = maxY - minY;
	
	// Сохраняем пропорции холста (800x600)
	const targetRatio = 800 / 600;
	const actualRatio = width / height;
	
	let adjustedWidth = width;
	let adjustedHeight = height;
	
	if (actualRatio > targetRatio) {
	  // Шире, чем нужно - увеличиваем высоту
	  adjustedHeight = width / targetRatio;
	} else {
	  // Уже, чем нужно - увеличиваем ширину
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
  }, [zMap]);

  // Функция нормализации координат
  const normalize = (x: number, y: number, stageWidth: number, stageHeight: number) => {
	// Используем пропорциональное масштабирование с сохранением пропорций
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

  // Обработчики зума
  const handleZoomIn = () => setScale(prev => Math.min(prev * 1.2, 5));
  const handleZoomOut = () => setScale(prev => Math.max(prev / 1.2, 0.5));
  const handleResetZoom = () => setScale(1);

  // Рендер
  if (zMap.size === 0) {
    return <div>Нет данных для отображения</div>;
  }

  return (
    <div style={{ padding: "20px" }}>
      {/* Панель управления масштабом */}
      <div style={{ marginBottom: "10px", display: "flex", gap: "10px" }}>
        <button onClick={handleZoomIn}>Увеличить (+)</button>
        <button onClick={handleZoomOut}>Уменьшить (-)</button>
        <button onClick={handleResetZoom}>Сбросить масштаб</button>
        <span>Масштаб: {Math.round(scale * 100)}%</span>
      </div>

      {/* Легенда цветов */}
      <div style={{ marginBottom: "20px" }}>
        <h3>Цветовая палитра (значения as1)</h3>
        <div style={{ display: "flex", flexWrap: "wrap" }}>
          {colorPalette.map((color, index) => {
            const minVal = minAs1 + index * (maxAs1 - minAs1) / 7;
            const maxVal = minAs1 + (index + 1) * (maxAs1 - minAs1) / 7;
            
            return (
              <div key={index} style={{ margin: "5px", display: "flex", alignItems: "center" }}>
                <div style={{
                  width: "30px",
                  height: "20px",
                  backgroundColor: color,
                  border: "1px solid #333",
                  marginRight: "5px"
                }}/>
                <span>{minVal.toFixed(1)} - {maxVal.toFixed(1)}</span>
              </div>
            );
          })}
        </div>
      </div>

      {/* Отображение фигур */}
      {Array.from(zMap.entries()).map(([z, shapes]) => (
        <div key={z} style={{ marginBottom: "40px" }}>
          <h2>Уровень Z = {z}</h2>
          <Stage 
            width={(800 * scale)/1.5} 
            height={(800 * scale)/1.5} 
            style={{ border: "1px solid #ddd", background: "#f8f8f8" }}
          >
            <Layer scaleX={scale} scaleY={scale}>
              {shapes.map((shape, idx) => {
                const color = getColorByAs1(shape.as1Value);
                const [textX, textY] = getNormalizedCenter(shape.center, 800, 600);
                // console.log(shape.points);
				
                // Определяем размер текста в зависимости от площади фигуры
                const fontSize = Math.max(6, Math.min(10, Math.sqrt(shape.area) *1.2));
                
                // Форматируем значение для отображения
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
                      strokeWidth={0.5}
                    //   opacity={0.85}
                    />
                    
                    {/* Оптимизированный текст */}
                    {shape.points.length>2&&<Text
                      x={textX}
                      y={textY}
                      text={displayValue}
                      fontSize={fontSize*1.4}
                      fill="black"
                      align="center"
                      verticalAlign="middle"
                      offsetX={displayValue.length * 2} // Автоподстройка под длину текста
                      offsetY={fontSize / 2}
                      scaleX={1/scale}
                      scaleY={1/scale}
                      listening={false}
                    //   fontStyle="bold"
                    />}
                  </React.Fragment>
                );
              })}
            </Layer>
          </Stage>
        </div>
      ))}
    </div>
  );
};