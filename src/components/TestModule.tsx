import React, { useState, useEffect } from 'react';
import init from '../assets/pkg/rst_build';
import  TestModuleWrapper  from '../assets/pkg/rst_build';


interface TestModuleComponentProps {
  className?: string;
}

const TestModuleComponent: React.FC<TestModuleComponentProps> = ({ className }) => {
  const [testModule, setTestModule] = useState<typeof TestModuleWrapper | null>(null);
  const [imageData, setImageData] = useState<string | null>(null);
  const [boundsInfo, setBoundsInfo] = useState<string>('');
  const [objectsCount, setObjectsCount] = useState<number>(0);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [wasmInitialized, setWasmInitialized] = useState<boolean>(false);

  useEffect(() => {
    const initWasm = async () => {
      try {
        await init();
        setWasmInitialized(true);

      } catch (error) {
        console.error('Ошибка инициализации WASM:', error);
      }
    };
    initWasm();
  }, []);

  const initializeTestModule = () => {
    if (!wasmInitialized) {
      console.error('WASM модуль не инициализирован');
      return;
    }
    try {
      const module = new TestModuleWrapper();
      module.generate_test_entities();
      setTestModule(module);
      
      // Получаем информацию о границах
      const bounds = module.get_bounds_info();
      setBoundsInfo(bounds);
      
      // Получаем количество объектов
      const count = module.get_object_count();
      setObjectsCount(count);
      
      
    } catch (error) {
      console.error('Ошибка при инициализации тестового модуля:', error);
    }
  };

  const generateTestImage = async () => {
    if (!testModule) {
      console.error('Тестовый модуль не инициализирован');
      return;
    }

    setIsLoading(true);
    try {
      
      
      // Генерируем изображение
      const imageBytes = testModule.draw_test_image();
      
      if (imageBytes && imageBytes.length > 0) {
        // Преобразуем байты в blob и создаем URL
        const blob = new Blob([imageBytes], { type: 'image/png' });
        const imageUrl = URL.createObjectURL(blob);
        setImageData(imageUrl);

      } else {
        console.error('Получен пустой массив байтов изображения');
      }
    } catch (error) {
      console.error('Ошибка при генерации тестового изображения:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const addCustomRectangle = () => {
    if (!testModule) {
      console.error('Тестовый модуль не инициализирован');
      return;
    }

    try {
      // Добавляем случайный прямоугольник
      const x1 = Math.random() * 10;
      const y1 = Math.random() * 10;
      const x2 = x1 + 2 + Math.random() * 3;
      const y2 = y1 + 2 + Math.random() * 3;
      
      testModule.add_custom_rectangle(x1, y1, x2, y2);
      
      // Обновляем информацию
      const bounds = testModule.get_bounds_info();
      setBoundsInfo(bounds);
      
      const count = testModule.get_object_count();
      setObjectsCount(count);
      
      
    } catch (error) {
      console.error('Ошибка при добавлении прямоугольника:', error);
    }
  };

  const generateSimpleDocument = async () => {
    if (!wasmInitialized) {
      console.error('WASM модуль не инициализирован');
      return;
    }
    setIsLoading(true);
    try {
      
      
      // Создаем новый экземпляр модуля
      const module = new TestModuleWrapper();
      
      // Генерируем простое изображение и создаем документ
      const docBytes = module.draw_simple_test_image();
      
      if (docBytes && docBytes.length > 0) {
        // Создаем blob для скачивания
        const blob = new Blob([docBytes], { type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' });
        const url = URL.createObjectURL(blob);
        
        // Создаем ссылку для скачивания
        const link = document.createElement('a');
        link.href = url;
        link.download = 'test_document.docx';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        
        // Освобождаем URL
        URL.revokeObjectURL(url);
        

      } else {
        console.error('Получен пустой массив байтов документа');
      }
    } catch (error) {
      console.error('Ошибка при генерации простого документа:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const clearTestData = () => {
    if (!testModule) {
      console.error('Тестовый модуль не инициализирован');
      return;
    }

    try {
      testModule.clear_data();
      setImageData(null);
      setBoundsInfo('');
      setObjectsCount(0);
      
    } catch (error) {
      console.error('Ошибка при очистке тестовых данных:', error);
    }
  };

  return (
    <div className={`p-6 bg-white rounded-lg shadow-lg ${className || ''}`}>
      <h2 className="text-2xl font-bold mb-4 text-gray-800">Тестовый модуль для четырехугольников</h2>
      
      <div className="space-y-4">
        {/* Кнопки управления */}
        <div className="flex flex-wrap gap-3">
          <button
            onClick={generateSimpleDocument}
            disabled={isLoading || !wasmInitialized}
            className="px-6 py-3 bg-purple-600 text-white rounded-lg hover:bg-purple-700 disabled:bg-gray-400 transition-colors font-semibold"
          >
            {isLoading ? 'Генерация документа...' : (wasmInitialized ? 'Генерация документа' : 'Загрузка WASM...')}
          </button>
          
          <button
            onClick={initializeTestModule}
            disabled={!wasmInitialized}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-400 transition-colors"
          >
            {wasmInitialized ? 'Инициализировать тестовые данные' : 'Загрузка WASM...'}
          </button>
          
          <button
            onClick={generateTestImage}
            disabled={!testModule || isLoading || !wasmInitialized}
            className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 disabled:bg-gray-400 transition-colors"
          >
            {isLoading ? 'Генерация...' : 'Сгенерировать изображение'}
          </button>
          
          <button
            onClick={addCustomRectangle}
            disabled={!testModule}
            className="px-4 py-2 bg-orange-500 text-white rounded hover:bg-orange-600 disabled:bg-gray-400 transition-colors"
          >
            Добавить случайный прямоугольник
          </button>
          
          <button
            onClick={clearTestData}
            disabled={!testModule}
            className="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 disabled:bg-gray-400 transition-colors"
          >
            Очистить данные
          </button>
        </div>

        {/* Информация о данных */}
        {testModule && (
          <div className="bg-gray-100 p-4 rounded">
            <h3 className="font-semibold mb-2">Информация о тестовых данных:</h3>
            <p className="text-sm text-gray-700 mb-2">Количество объектов: {objectsCount}</p>
            {boundsInfo && (
              <pre className="text-sm text-gray-700 whitespace-pre-wrap">{boundsInfo}</pre>
            )}
          </div>
        )}

        {/* Отображение изображения */}
        {imageData && (
          <div className="border rounded p-4">
            <h3 className="font-semibold mb-2">Сгенерированное изображение:</h3>
            <img 
              src={imageData} 
              alt="Тестовое изображение" 
              className="max-w-full h-auto border rounded"
              style={{ maxHeight: '600px' }}
            />
          </div>
        )}

        {/* Инструкции */}
        <div className="bg-blue-50 p-4 rounded">
          <h3 className="font-semibold mb-2 text-blue-800">Инструкции:</h3>
          <ol className="text-sm text-blue-700 space-y-1">
            <li>1. Нажмите "Инициализировать тестовые данные" для создания 5 тестовых четырехугольников</li>
            <li>2. Нажмите "Сгенерировать изображение" для отрисовки фигур</li>
            <li>3. Используйте "Добавить случайный прямоугольник" для экспериментов</li>
            <li>4. Изучайте информацию о границах и масштабировании</li>
            <li>5. Очищайте данные для новых экспериментов</li>
          </ol>
        </div>
      </div>
    </div>
  );
};

export default TestModuleComponent;