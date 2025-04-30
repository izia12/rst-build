import React, { useRef } from 'react';
import { Document, Packer, Paragraph, ImageRun } from 'docx';
import { saveAs } from 'file-saver';

interface Point {
    x: number;
    y: number;
}

const FileUploadAndDocxGenerator = () => {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    // Пример данных точек
    const points: Point[] = [
        { x: 10, y: 10 },
        { x: 200, y: 10 },
        { x: 150, y: 100 },
        { x: 50, y: 150 },
        { x: 20, y: 200 },
    ];

    const generateImage = async () => {
        if (!canvasRef.current) return;

        const canvas = canvasRef.current;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        // Устанавливаем размеры холста
        canvas.width = 400;
        canvas.height = 400;

        // Рисуем фон
        ctx.fillStyle = 'lightblue';
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Рисуем линии между точками
        ctx.beginPath();
        ctx.strokeStyle = 'black';
        ctx.lineWidth = 2;
        points.forEach((point, index) => {
            if (index === 0) {
                ctx.moveTo(point.x, point.y);
            } else {
                ctx.lineTo(point.x, point.y);
            }
        });
        ctx.stroke();

        // Генерация изображения из canvas в формате PNG
        const imageDataUrl = canvas.toDataURL('image/png');

        // Генерируем .docx с изображением
        await generateDocx(imageDataUrl);
    };

    const generateDocx = async (imageDataUrl: string) => {
        const imageBlob = await fetch(imageDataUrl).then(res => res.blob());
        const reader = new FileReader();

        reader.onloadend = async () => {
            const doc = new Document({
                sections: [
                    {
                        properties: {},
                        children: [
                            new Paragraph('Заголовок документа'),
                            new Paragraph('Это простой пример .docx файла с динамически сгенерированным изображением.'),
                            new Paragraph({
                                children: [
                                    new ImageRun({
                                        data: await fetch(imageDataUrl).then(res => res.arrayBuffer()),
                                        transformation: {
                                            width: 400,
                                            height: 400,
                                        },
                                        type: 'png',
                                    }),
                                ],
                            }),
                        ],
                    },
                ],
            });

            try {
                const blob = await Packer.toBlob(doc);
                saveAs(blob, 'document.docx');
            } catch (error) {
                console.error('Ошибка при генерации .docx файла:', error);
            }
        };
        reader.readAsArrayBuffer(imageBlob);
    };

    return (
        <div className="p-4">
            <button
                onClick={generateImage}
                className="mt-4 bg-blue-500 text-white p-2 rounded"
            >
                Создать документ с изображением
            </button>

            {/* Canvas, который не отображается на экране */}
            <canvas ref={canvasRef} style={{ display: 'none' }} />
        </div>
    );
};

export default FileUploadAndDocxGenerator;