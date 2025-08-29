import React from 'react';
import { useAppSelector } from '../store/store';
import { useAppDispatch } from '../store/store';
import { generateDocumentForSelectedCombinations, generateDocumentWithColorPalette } from '../store/slices/thunks/wasmThanks';
import { Button } from './custom-components/Button';
import { SelectedCombination, SelectedCombinationsData } from '../types/data.types';

export const DocumentGenerator: React.FC = () => {
    const dispatch = useAppDispatch();
    const { excelViewData, documentGeneration } = useAppSelector(state => state.wasm);
    
    const handleGenerateDocument = async () => {
        // Собираем выбранные комбинации с их данными
        const selectedCombinations: SelectedCombination[] = [];
        const selectedFloors: string[] = [];
        
        excelViewData.forEach(floor => {
            floor.values.forEach(armatureCombination => {
                armatureCombination.combinations.forEach(item => {
                    if (item.is_default_checked) {
                        // Добавляем выбранную комбинацию
                        selectedCombinations.push({
                            floor_level: floor.level,
                            function_name: armatureCombination.function_name,
                            as_target_value: armatureCombination.as_target_value,
                            combination: {
                                ...item,
                                result_scale: item.result_scale || `[${item.total_area.toFixed(3)}см2:Ø${item.main_diameter}${item.additional_diameter ? '+Ø' + item.additional_diameter : ''}]`
                            }
                        });
                        

                        // Добавляем этаж в список если его еще нет
                        if (!selectedFloors.includes(floor.level)) {
                            selectedFloors.push(floor.level);
                        }
                    }
                });
            });
        });
        
        if (selectedCombinations.length === 0) {
            alert('Не выбрано ни одной комбинации для генерации документа');
            return;
        }
        
        const selectedData: SelectedCombinationsData = {
            combinations: selectedCombinations,
            floors: selectedFloors
        };
        

		
        try {
            // Используем новую функцию с цветовой палитрой
            const result = await dispatch(generateDocumentWithColorPalette(selectedData)).unwrap();
            
            // Создаем blob и скачиваем файл
            const blob = new Blob([result], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
            const url = URL.createObjectURL(blob);
            const link = document.createElement('a');
            link.href = url;
            link.download = `document_${new Date().toISOString().slice(0, 10)}.docx`;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
            URL.revokeObjectURL(url);
        } catch (error) {
            console.error('Ошибка при генерации документа:', error);
            alert('Ошибка при генерации документа');
        }
    };
    
    return (
		<>
        <Button
            onClick={handleGenerateDocument}
            disabled={!excelViewData || excelViewData.length === 0 || documentGeneration.loading}
            className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
        >
            {documentGeneration.loading ? 'Генерация...' : 'Получить документ'}
        </Button>
        {documentGeneration.loading &&
            <div className="mt-2">
                <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-500"></div>
            </div>
        }
		</>
    );
};