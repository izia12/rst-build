import React, {useEffect, useRef} from 'react';
import {Line, LineCanvasType} from "./drawLine/drawLine.ts";

const DiagramGenerator = () => {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    const data1:LineCanvasType[] =[
        {startX:10, startY:10, endX:10, endY:(window.innerHeight+25), color:"black", lineWidth:1},
    ]
    const data2 =[
        {startX:10, startY:10, endX:(window.innerWidth+25), endY:10, color:"black", lineWidth:1},
    ]
    console.log(window.innerHeight)
    function getRandomInteger(min: number, max: number): number {
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }
    useEffect(()=>{
        const ctx  = (canvasRef.current as HTMLCanvasElement).getContext('2d') as CanvasRenderingContext2D
        if(ctx){
            for (let i=0; i<Math.trunc(window.innerWidth/25); i++){
                new Line(
                    ctx,
                    data1[0].startX+i*25,
                    data1[0].startY,
                    data1[0].startX+i*25,
                    data1[0].endY,
                    {x:(data1[0].startX+i*25)-2, y:data1[0].startY+25, text:i.toString()},
                    data1[0].color,
                    data1[0].lineWidth,
                ).draw()

            }
            for (let i = 0; i < Math.trunc((window.innerHeight)/25); i++) {
                const line = new Line(
                    ctx,
                    data2[0].startX,
                    data2[0].startY+i*25,
                    data2[0].endX,
                    data2[0].startY+i*25,
                    {x:data2[0].startX, y: data2[0].startY+i*25, text:i.toString()},
                    data2[0].color,
                    data2[0].lineWidth,
                )
                line.draw()
                // line.drawText(data1[0].startX+i*25, data1[0].startY+i*25, i)
                for (let k=0; k<Math.trunc(window.innerWidth/25); k++){
                    const number = getRandomInteger(0,200)
                    let color = ""
                    if(number>=0&&number<34){
                        color = "#FFFFE0"
                    }
                    if(number>=34&&number<67){
                        color = "#FFFF00"
                    }
                    if (number>=67&&number<100){
                        color= "#FFA500"
                    }
                    if(number>=100&&number<133){
                        color = "#FF8C00"
                    }
                    if(number>=133&&number<167){
                        color = "#FF4500"
                    }
                    if(number>=167&&number<=200){
                        color = "#800000"
                    }
                    // console.log(`Before drawText: (${data1[0].startX + k * 25}, ${data1[0].startY + i * 25}) - Color: ${color}`);
                    line.drawText(data1[0].startX+k*25, data1[0].startY+i*25, number.toString(), color)
                }
            }
        }
    },[])
    console.log(Math.trunc((window.innerHeight)/18))
    const drawDiagram = () => {
        if (!canvasRef.current) return;

        const canvas = canvasRef.current;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        ctx.clearRect(0, 0, canvas.width, canvas.height); // Очищаем канвас

        // Рисуем прямоугольник (объект)
        ctx.beginPath();
        ctx.rect(100, 100, 250, 100);
        ctx.stroke(); // Обводим контур

        // Рисуем стрелки
        const arrows = [
            { startX: 70, startY: 140, endX: 100, endY: 140, color: 'blue' },   // AS3
            { startX: 70, startY: 160, endX: 100, endY: 160, color: 'blue' },
            { startX: 70, startY: 180, endX: 100, endY: 180, color: 'blue' },
            { startX: 300, startY: 125, endX: 400, endY: 125, color: 'green' }, // AS1
            { startX: 300, startY: 140, endX: 400, endY: 140, color: 'green' },
            { startX: 300, startY: 160, endX: 400, endY: 160, color: 'green' },
        ];

        arrows.forEach(arrow => {
            ctx.strokeStyle = arrow.color;
            drawArrow(ctx, arrow.startX, arrow.startY, arrow.endX, arrow.endY);
        });

        // Рисуем оси
        ctx.beginPath();
        ctx.moveTo(150, 75); // Положение начала оси Z
        ctx.lineTo(150, 50); // Конец оси Z
        ctx.strokeStyle = 'red';
        ctx.stroke();

        ctx.fillText('Z', 155, 50);
        ctx.fillText('Y', 250, 75);
        ctx.fillText('X', 350, 95);

        ctx.fillText('AS1', 425, 130);
        ctx.fillText('AS2', 425, 150);
        ctx.fillText('AS3', 62, 140);
        ctx.fillText('AS4', 62, 160);
    };

    const drawArrow = (ctx: CanvasRenderingContext2D, fromX: number, fromY: number, toX: number, toY: number) => {
        const headLength = 10; // Длина стрелки
        const dx = toX - fromX;
        const dy = toY - fromY;
        const angle = Math.atan2(dy, dx);

        ctx.beginPath();
        ctx.moveTo(fromX, fromY);
        ctx.lineTo(toX, toY);
        ctx.lineTo(toX - headLength * Math.cos(angle - Math.PI / 6), toY - headLength * Math.sin(angle - Math.PI / 6));
        ctx.moveTo(toX, toY);
        ctx.lineTo(toX - headLength * Math.cos(angle + Math.PI / 6), toY - headLength * Math.sin(angle + Math.PI / 6));
        ctx.stroke();
    };

    return (
        <div>
            <canvas ref={canvasRef} width={window.innerWidth} height={window.innerHeight} style={{ border: '1px solid black' }} />
            <button onClick={drawDiagram} style={{ marginTop: '10px' }}>Создать диаграмму</button>
        </div>
    );
};

export default DiagramGenerator;