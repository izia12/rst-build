import React, { useMemo, memo } from "react";
import * as d3 from "d3";
import { useAppSelector } from "../store/store";
import { RowData } from "../types/data.types";

// Добавляем тип для фигур с дополнительными данными
type ProcessedShape = {
  type: "3DFACE" | "3DFACE_TRIANGLE" | "LINE";
  points: number[][];
  rowData: RowData; // Добавляем данные для аннотаций
};

type ProcessedLayer = {
  z: number;
  shapes: ProcessedShape[];
};

export default function Quadrilaterals() {
  const data = useAppSelector(state => state.wasm.wasmData);
  
  const layers = useMemo(() => {
    const layersMap = new Map<number, ProcessedLayer>();
    
    data.forEach((item, index) => {
      const vz = item.vertices[0]?.z;
      if (vz === undefined || !item.vertices.every(v => v.z === vz)) return;

      const points = item.vertices.map(v => [v.x, v.y]);
      
      // Добавляем rowData из первой строки (можно настроить логику)
      const rowData = item.row?.[0]; 

      if (!layersMap.has(vz)) {
        layersMap.set(vz, {
          z: vz,
          shapes: []
        });
      }
      
      layersMap.get(vz)!.shapes.push({
        type: item.entity_type,
        points,
        rowData // Сохраняем данные для аннотаций
      });
    });

    // Сортируем слои по z (от меньшего к большему)
    return Array.from(layersMap.values())
      .sort((a, b) => a.z - b.z);
  }, [data]);

  return (
    <div className="layers-container">
      {layers.map(layer => (
        <Layer key={layer.z} layer={layer} />
      ))}
    </div>
  );
}

const Layer = memo(({ layer }: { layer: ProcessedLayer }) => {
  const svgRef = React.useRef<SVGSVGElement>(null);
  const [bounds, setBounds] = React.useState({ width: 0, height: 0 });

  React.useEffect(() => {
    if (!svgRef.current) return;
    
    const allPoints = layer.shapes.flatMap(s => s.points);
    const xExtent = d3.extent(allPoints, p => p[0]);
    const yExtent = d3.extent(allPoints, p => p[1]);
    
    setBounds({
      width: xExtent[1] - xExtent[0] || 1,
      height: yExtent[1] - yExtent[0] || 1
    });

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    const margin = 20;
    const width = 500;
    const height = 500;

    const xScale = d3.scaleLinear()
      .domain([xExtent[0], xExtent[1]])
      .range([margin, width - margin]);

    const yScale = d3.scaleLinear()
      .domain([yExtent[0], yExtent[1]])
      .range([height - margin, margin]);

    layer.shapes.forEach(shape => {
      const points = shape.points.map(p => 
        `${xScale(p[0])},${yScale(p[1])}`
      ).join(" ");
      
      // Рисуем фигуру
      const element = svg.append(shape.type === "LINE" ? "polyline" : "polygon")
        .attr("points", points)
        .attr("fill", "none")
        .attr("stroke", "#2c3e50")
        .attr("stroke-width", 0.5)
        .attr("stroke-linejoin", "round");

      // Добавляем аннотации если есть данные
      if (shape.rowData) {
        // Вычисляем центр фигуры
        const centroid = d3.polygonCentroid(shape.points.map(p => [
          xScale(p[0]), 
          yScale(p[1])
        ]));
        
        // Находим максимальное значение из as1
        const maxAs1 = Math.max(...shape.rowData.as1);
        
        // Добавляем текст
        element.append("text")
          .attr("x", centroid[0])
          .attr("y", centroid[1])
          .attr("text-anchor", "middle")
          .attr("dominant-baseline", "central")
          .attr("font-size", "8px")
          .attr("fill", "#e74c3c")
          .text(`${maxAs1.toFixed(2)}`);
      }
    });

  }, [layer]);

  return (
    <div className="layer">
      <h3>Z-Level: {layer.z.toFixed(1)}</h3>
      <svg
        ref={svgRef}
        width={500}
        height={500}
        viewBox={`0 0 500 500`}
        preserveAspectRatio="xMidYMid meet"
      />
    </div>
  );
});