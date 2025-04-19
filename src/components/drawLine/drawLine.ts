export type LineCanvasType = {
    startX:number
    startY:number
    endX: number,
    endY: number,
    color: string,
    lineWidth: number
}
export class Line {
    private ctx: CanvasRenderingContext2D;
    private startX: number;
    private startY: number;
    private endX: number;
    private endY: number;
    private color: string;
    private fillText:{
        x:number,
        y:number,
        text:string,
    };
    private lineWidth: number;

    constructor(
        ctx: CanvasRenderingContext2D,
        startX: number,
        startY: number,
        endX: number,
        endY: number,
        fillText:{
            x:number,
            y:number,
            text:string,
        },
        color: string = 'black',
        lineWidth: number = 1
) {
        this.ctx = ctx;
        this.startX = startX;
        this.startY = startY;
        this.endX = endX;
        this.endY = endY;
        this.color = color;
        this.fillText = fillText;
        this.lineWidth = lineWidth;
    }

public draw(): void {
        this.ctx.beginPath(); // Начинаем новый путь
        this.ctx.moveTo(this.startX, this.startY); // Перемещаем к начальной точке
        this.ctx.lineTo(this.endX, this.endY); // Рисуем линию до конечной точки
        this.ctx.strokeStyle = this.color; // Устанавливаем цвет
        this.ctx.lineWidth = this.lineWidth; // Устанавливаем ширину
        this.ctx.fillText(this.fillText.text, this.fillText.x, this.fillText.y);
        this.ctx.font="10px Arial"
        this.ctx.stroke(); // Рисуем линию

    }
    public drawText(X:number, Y:number, text:string, color:string | CanvasGradient | CanvasPattern):void{
        // console.log(`Drawing text "${text}" at (${X}, ${Y}) with color: ${color}`); // Лог для отладки

        this.ctx.fillStyle = color;   // Устанавливаем цвет фона
        this.ctx.fillRect(X, Y, 25, 25); // Рисуем прямоугольник

        this.ctx.font = "10px Arial"; // Устанавливаем стиль шрифта
        this.ctx.fillStyle = "black";  // Цвет текста
        this.ctx.fillText(text, X, Y); // Рисуем текст
    }
}

// Получаем элемент canvas и его контекст
// const canvas = document.getElementById('myCanvas') as HTMLCanvasElement;
// const ctx = canvas.getContext('2d');
//
// if (ctx) {
//     // Создаем экземпляр класса Line и рисуем линию
//     const line = new Line(ctx, 50, 50, 250, 250, 'blue', 5);
//     line.draw();
// }