# 📚 Документация проекта React 3D WASM

## 🔗 Цепочки вызовов функций

### 1. 🎯 get_horizontal_elements_object_js

**📋 Название:** `get_horizontal_elements_object_js`

**🎯 Предназначение:** Основная функция для извлечения и группировки горизонтальных элементов (плит и стержней) по Z-координатам с вычислением максимальных значений арматуры

**📥 Параметры:** Нет параметров (использует глобальное состояние `GLOBAL_ENTITIES`)

**⚙️ Что делает в процессе:**
1. Извлекает данные из глобального хранилища `GLOBAL_ENTITIES`
2. Группирует плиты (`plates`) и стержни (`rods`) по Z-координатам
3. Сортирует Z-значения в порядке возрастания
4. Для каждого уровня вычисляет максимальные значения арматуры:
   - `maxAs1`, `maxAs2`, `maxAs3`, `maxAs4` (основная арматура)
   - `maxAsw1`, `maxAsw2` (поперечная арматура)
5. Округляет значения до 3 знаков после запятой для устранения погрешностей f32
6. Собирает информацию о материалах для каждого уровня
7. Преобразует данные в JavaScript-объект

**📤 Что отдает на выходе:** 
JavaScript объект с группированными по Z-координатам данными:
```javascript
{
  "z_coordinate": {
    plates: Array<Entity>,
    rods: Array<Entity>,
    Materials: Array<number>,
    maxAs1: number,
    maxAs2: number,
    maxAs3: number,
    maxAs4: number,
    maxAsw1: number,
    maxAsw2: number
  }
}
```

**🔧 Тип функции:** 🎯 **Основная функция** (предназначена для экспорта данных во фронтенд)

**🔗 Полная цепочка вызовов:**
```
UI (App.tsx) → onInputsChanged() → dispatch(fetchWasmJSData()) 
    ↓ [BACKEND РАЗДЕЛИТЕЛЬ]
    → fetchWasmJSData (thunk) → get_horizontal_elements_object_js() → entity_to_js() → getPureWASMJsData()
```

---

## 🛠️ WASM Функции из lib.rs

### 2. 📊 parse_data

**📋 Название:** `parse_data`

**🎯 Предназначение:** Парсинг входных данных из SLI, TXT и XLSX файлов и сохранение в глобальное состояние

**📥 Параметры:**
- `sli_data: &str` - данные SLI файла
- `txt_data: &str` - данные TXT файла  
- `xlsx_data: &[u8]` - бинарные данные XLSX файла

**⚙️ Что делает в процессе:**
1. Парсит SLI данные для извлечения геометрии
2. Обрабатывает TXT данные для дополнительной информации
3. Читает XLSX файл для получения табличных данных
4. Сохраняет все данные в `GLOBAL_ENTITIES`

**📤 Что отдает на выходе:** Void (сохраняет данные в глобальное состояние)

**🔧 Тип функции:** 🎯 **Основная функция** (инициализация данных)

### 3. 🔄 convert_sli_xsl_to_json_string

**📋 Название:** `convert_sli_xsl_to_json_string`

**🎯 Предназначение:** Конвертация обработанных данных в JSON строку для передачи во фронтенд

**📥 Параметры:** Нет (использует `GLOBAL_ENTITIES`)

**⚙️ Что делает в процессе:**
1. Извлекает данные из глобального состояния
2. Сериализует в JSON формат
3. Возвращает строку

**📤 Что отдает на выходе:** JSON строка с данными

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (может переиспользоваться)

### 4. 📄 create_docx

**📋 Название:** `create_docx`

**🎯 Предназначение:** Создание DOCX документа с визуализацией данных

**📥 Параметры:**
- `selected_combinations_json: &str` - JSON с выбранными комбинациями
- `color_palette_json: &str` - JSON с цветовой палитрой

**⚙️ Что делает в процессе:**
1. Десериализует входные JSON данные
2. Создает DOCX документ
3. Добавляет таблицы и изображения
4. Применяет цветовую схему

**📤 Что отдает на выходе:** `Vec<u8>` - бинарные данные DOCX файла

**🔧 Тип функции:** 🎯 **Основная функция** (генерация отчетов)

### 5. 🖼️ create_docx_with_image

**📋 Название:** `create_docx_with_image`

**🎯 Предназначение:** Создание DOCX документа с встроенными изображениями

**📥 Параметры:**
- `selected_combinations_json: &str` - JSON с данными
- `image_data: &[u8]` - бинарные данные изображения

**⚙️ Что делает в процессе:**
1. Обрабатывает изображение
2. Встраивает в DOCX документ
3. Добавляет описания и таблицы

**📤 Что отдает на выходе:** `Vec<u8>` - DOCX с изображениями

**🔧 Тип функции:** 🎯 **Основная функция** (расширенная генерация отчетов)

### 6. 📋 get_table_data_for_frontend

**📋 Название:** `get_table_data_for_frontend`

**🎯 Предназначение:** Подготовка табличных данных для отображения во фронтенде

**📥 Параметры:**
- `diameters: &[u32]` - массив диаметров арматуры
- `floors_json: &str` - JSON с данными этажей

**⚙️ Что делает в процессе:**
1. Обрабатывает данные этажей
2. Фильтрует по диаметрам
3. Формирует таблицы для UI

**📤 Что отдает на выходе:** JSON строка с табличными данными

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (подготовка данных для UI)

### 7. 📊 get_sortament_data

**📋 Название:** `get_sortament_data`

**🎯 Предназначение:** Получение данных сортамента арматуры

**📥 Параметры:** Нет

**⚙️ Что делает в процессе:**
1. Извлекает данные сортамента из глобального состояния
2. Форматирует для передачи во фронтенд

**📤 Что отдает на выходе:** Массив данных сортамента

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (доступ к справочным данным)

---

## 🔧 Helper Функции

### 8. 🔄 getPureWASMJsData

**📋 Название:** `getPureWASMJsData`

**🎯 Предназначение:** Преобразование сырых WASM данных в упрощенный формат для UI

**📥 Параметры:**
- `data: WasmDataJsType` - сырые данные от WASM

**⚙️ Что делает в процессе:**
1. Итерирует по всем уровням данных
2. Извлекает только необходимые поля:
   - Количество плит и стержней
   - Максимальные значения арматуры
   - Материалы
   - Добавляет значения по умолчанию (mainStep, secondaryStep, isSelected)

**📤 Что отдает на выходе:** `Promise<PureWASMJsData>` - упрощенные данные

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (трансформация данных)

### 9. 🎨 transformLinesPointsIntoArray

**📋 Название:** `transformLinesPointsIntoArray`

**🎯 Предназначение:** Преобразование массива координат в THREE.js объекты для 3D визуализации

**📥 Параметры:**
- `type: "LINES" | "3DFACES" | "TRIANGLE_FACES"` - тип геометрии
- `lines: LinesType` - массив координат
- `color: string` - цвет линий

**⚙️ Что делает в процессе:**
1. Валидирует координаты (проверка на NaN, Infinity, нули)
2. Фильтрует невалидные линии
3. Создает THREE.js геометрию
4. Применяет материал с цветом

**📤 Что отдает на выходе:** `THREE.LineSegments | undefined` - 3D объект или undefined

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (3D визуализация)

---

## 🔗 Основные цепочки вызовов

### Цепочка 1: Загрузка и обработка данных
```
UI (файловые инпуты) → onInputsChanged() → dispatch(fetchWasmData()) 
    ↓ [BACKEND РАЗДЕЛИТЕЛЬ]
    → fetchWasmData (thunk) → parse_data() → convert_sli_xsl_to_json_string()
```

### Цепочка 2: Получение данных для UI
```
UI (App.tsx) → onInputsChanged() → dispatch(fetchWasmJSData()) 
    ↓ [BACKEND РАЗДЕЛИТЕЛЬ]
    → fetchWasmJSData (thunk) → get_horizontal_elements_object_js() → entity_to_js() → getPureWASMJsData()
```

### Цепочка 3: Генерация документов
```
UI (кнопка экспорта) → dispatch(generateDocumentWithColorPalette()) 
    ↓ [BACKEND РАЗДЕЛИТЕЛЬ]
    → generateDocumentWithColorPalette (thunk) → create_docx_with_selected_combinations()
```

### Цепочка 4: Получение табличных данных
```
UI (таблица) → dispatch(fetchExcelViewData()) 
    ↓ [BACKEND РАЗДЕЛИТЕЛЬ]
    → fetchExcelViewData (thunk) → get_table_data_for_frontend()
```

### Цепочка 5: Получение сортамента
```
UI (селект диаметров) → dispatch(fetchArmDimeters()) 
    ↓ [BACKEND РАЗДЕЛИТЕЛЬ]
    → fetchArmDimeters (thunk) → get_sortament_data()
```

---

## 📊 Статистика функций

**🎯 Основные функции (7):**
- parse_data
- get_horizontal_elements_object_js  
- create_docx
- create_docx_with_image
- create_docx_with_selected_combinations
- get_table_data_for_frontend
- get_sortament_data

**🔧 Вспомогательные функции (3):**
- convert_sli_xsl_to_json_string
- getPureWASMJsData
- transformLinesPointsIntoArray
- entity_to_js

### 10. 🔧 initialize_gpu_renderer

**📋 Название:** `initialize_gpu_renderer`

**🎯 Предназначение:** Инициализация GPU рендерера для ускорения вычислений

**📥 Параметры:** Нет

**⚙️ Что делает в процессе:**
1. Инициализирует GPU контекст
2. Настраивает шейдеры для вычислений
3. Подготавливает буферы данных

**📤 Что отдает на выходе:** Void (инициализирует глобальное состояние GPU)

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (инициализация системы)

### 11. 📊 process_files

**📋 Название:** `process_files`

**🎯 Предназначение:** Обработка множественных файлов данных

**📥 Параметры:**
- `files_data: &str` - JSON с данными файлов

**⚙️ Что делает в процессе:**
1. Парсит JSON с файлами
2. Обрабатывает каждый файл
3. Агрегирует результаты

**📤 Что отдает на выходе:** JSON строка с результатами обработки

**🔧 Тип функции:** 🎯 **Основная функция** (пакетная обработка)

### 12. 🎨 new_draw_polygon

**📋 Название:** `new_draw_polygon`

**🎯 Предназначение:** Отрисовка полигонов с улучшенным алгоритмом

**📥 Параметры:**
- `polygon_data: &str` - JSON с данными полигона

**⚙️ Что делает в процессе:**
1. Парсит данные полигона
2. Применяет алгоритм триангуляции
3. Генерирует вершины для рендеринга

**📤 Что отдает на выходе:** Массив вершин полигона

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (графические вычисления)

### 13. 📏 sort_by_same_z

**📋 Название:** `sort_by_same_z`

**🎯 Предназначение:** Сортировка элементов по одинаковым Z-координатам

**📥 Параметры:**
- `elements_json: &str` - JSON с элементами

**⚙️ Что делает в процессе:**
1. Группирует элементы по Z-координатам
2. Сортирует внутри каждой группы
3. Оптимизирует порядок для рендеринга

**📤 Что отдает на выходе:** JSON с отсортированными элементами

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (оптимизация данных)

### 14. 📋 get_processed_data_for_frontend

**📋 Название:** `get_processed_data_for_frontend`

**🎯 Предназначение:** Подготовка обработанных данных специально для фронтенда

**📥 Параметры:**
- `filter_params: &str` - JSON с параметрами фильтрации

**⚙️ Что делает в процессе:**
1. Применяет фильтры к данным
2. Форматирует для UI компонентов
3. Оптимизирует размер передаваемых данных

**📤 Что отдает на выходе:** JSON с данными для фронтенда

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (подготовка данных)

### 15. 🏢 create_docx_for_single_floor

**📋 Название:** `create_docx_for_single_floor`

**🎯 Предназначение:** Создание DOCX документа для одного этажа

**📥 Параметры:**
- `floor_data: &str` - JSON с данными этажа
- `combinations_data: &str` - JSON с комбинациями

**⚙️ Что делает в процессе:**
1. Десериализует данные этажа
2. Обрабатывает комбинации арматуры
3. Генерирует документ с планом этажа
4. Добавляет таблицы расчетов

**📤 Что отдает на выходе:** `Vec<u8>` - DOCX файл этажа

**🔧 Тип функции:** 🎯 **Основная функция** (генерация отчетов по этажам)

### 16. 🖼️ create_partial_images

**📋 Название:** `create_partial_images`

**🎯 Предназначение:** Создание частичных изображений для Web Workers

**📥 Параметры:**
- `image_params: &str` - JSON с параметрами изображения

**⚙️ Что делает в процессе:**
1. Разбивает изображение на части
2. Обрабатывает каждую часть в отдельном потоке
3. Оптимизирует для параллельной обработки

**📤 Что отдает на выходе:** Массив частичных изображений

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (многопоточная обработка)

### 17. 🧪 create_test_docx_with_images

**📋 Название:** `create_test_docx_with_images`

**🎯 Предназначение:** Тестовая функция для создания DOCX с изображениями

**📥 Параметры:**
- `test_data: &str` - тестовые данные

**⚙️ Что делает в процессе:**
1. Генерирует тестовые изображения
2. Создает документ для тестирования
3. Проверяет корректность встраивания

**📤 Что отдает на выходе:** `Vec<u8>` - тестовый DOCX

**🔧 Тип функции:** 🧪 **Тестовая функция** (только для разработки)

### 18. 🎨 create_test_image

**📋 Название:** `create_test_image`

**🎯 Предназначение:** Создание тестового изображения

**📥 Параметры:**
- `width: u32` - ширина изображения
- `height: u32` - высота изображения

**⚙️ Что делает в процессе:**
1. Генерирует тестовое изображение заданного размера
2. Применяет тестовые паттерны
3. Кодирует в нужный формат

**📤 Что отдает на выходе:** `Vec<u8>` - бинарные данные изображения

**🔧 Тип функции:** 🧪 **Тестовая функция** (только для разработки)

### 19. ⚡ get_optimized_canvas_data_wasm

**📋 Название:** `get_optimized_canvas_data_wasm`

**🎯 Предназначение:** Получение оптимизированных данных для canvas

**📥 Параметры:**
- `canvas_params: &str` - параметры canvas

**⚙️ Что делает в процессе:**
1. Оптимизирует данные для canvas рендеринга
2. Применяет алгоритмы сжатия
3. Подготавливает для WebGL

**📤 Что отдает на выходе:** Оптимизированные данные canvas

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (оптимизация рендеринга)

### 20. 📊 get_canvas_statistics_wasm

**📋 Название:** `get_canvas_statistics_wasm`

**🎯 Предназначение:** Получение статистики canvas для мониторинга производительности

**📥 Параметры:** Нет

**⚙️ Что делает в процессе:**
1. Собирает метрики производительности canvas
2. Анализирует использование памяти
3. Подсчитывает количество объектов

**📤 Что отдает на выходе:** JSON со статистикой canvas

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (мониторинг производительности)

---

## 🔗 Дополнительные цепочки вызовов

### Цепочка 6: Создание документа для одного этажа
```
UI (кнопка "Экспорт этажа") → dispatch(generateFloorDocument()) 
    ↓ [BACKEND РАЗДЕЛИТЕЛЬ]
    → generateFloorDocument (thunk) → create_docx_for_single_floor()
```

### Цепочка 7: Обработка файлов
```
UI (множественная загрузка) → dispatch(processMultipleFiles()) 
    ↓ [BACKEND РАЗДЕЛИТЕЛЬ]
    → processMultipleFiles (thunk) → process_files()
```

### Цепочка 8: Оптимизация canvas
```
UI (3D viewer) → optimizeCanvasData() 
    ↓ [BACKEND РАЗДЕЛИТЕЛЬ]
    → get_optimized_canvas_data_wasm() → get_canvas_statistics_wasm()
```

---

## 📊 Обновленная статистика функций

**🎯 Основные функции (9):**
- parse_data
- get_horizontal_elements_object_js  
- create_docx
- create_docx_with_image
- create_docx_with_selected_combinations
- get_table_data_for_frontend
- get_sortament_data
- process_files
- create_docx_for_single_floor

**🔧 Вспомогательные функции (9):**
- convert_sli_xsl_to_json_string
- getPureWASMJsData
- transformLinesPointsIntoArray
- entity_to_js
- initialize_gpu_renderer
- new_draw_polygon
- sort_by_same_z
- get_processed_data_for_frontend
- create_partial_images
- get_optimized_canvas_data_wasm
- get_canvas_statistics_wasm

**🧪 Тестовые функции (2):**
- create_test_docx_with_images
- create_test_image

---

## 🔧 Дополнительные Helper Функции

### 21. 🎨 getLayerColor

**📋 Название:** `getLayerColor`

**🎯 Предназначение:** Генерация уникального цвета для слоя на основе его имени

**📥 Параметры:**
- `layer: string` - имя слоя
- `type: string = "LINE"` - тип объекта (LINE или 3DFACE)

**⚙️ Что делает в процессе:**
1. Вычисляет хеш от имени слоя
2. Преобразует хеш в цвет
3. Для 3DFACE делает цвет светлее
4. Возвращает цвет в формате HEX

**📤 Что отдает на выходе:** `string` - цвет в формате #RRGGBB

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (генерация цветов)

### 22. 🗺️ getLayersMap

**📋 Название:** `getLayersMap`

**🎯 Предназначение:** Создание карты слоев с группировкой геометрии по цветам

**📥 Параметры:**
- `data: MainDataType` - основные данные проекта

**⚙️ Что делает в процессе:**
1. Итерирует по всем данным
2. Группирует геометрию по слоям
3. Разделяет на линии, грани и треугольные грани
4. Присваивает цвета из предопределенной палитры

**📤 Что отдает на выходе:** `MappedShapesByColor` - карта геометрии по цветам

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (организация данных)

### 23. 📐 getLinesVerticesArray

**📋 Название:** `getLinesVerticesArray`

**🎯 Предназначение:** Преобразование массива вершин в плоский массив координат для линий

**📥 Параметры:**
- `lines: VerticesType[]` - массив вершин линий

**⚙️ Что делает в процессе:**
1. Фильтрует вершины с валидными координатами
2. Преобразует в плоский массив [x, y, z, x, y, z, ...]
3. Удаляет null/undefined значения

**📤 Что отдает на выходе:** `Array<number>` - плоский массив координат

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (преобразование данных)

### 24. 🔺 getFacesVerticesArray

**📋 Название:** `getFacesVerticesArray`

**🎯 Предназначение:** Преобразование массива вершин в плоский массив координат для граней

**📥 Параметры:**
- `faces: VerticesType[]` - массив вершин граней

**⚙️ Что делает в процессе:**
1. Фильтрует вершины с валидными координатами
2. Преобразует в плоский массив для граней
3. Обеспечивает правильный порядок вершин

**📤 Что отдает на выходе:** `Array<number>` - плоский массив координат граней

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (преобразование данных)

### 25. 📊 getVerticesArray

**📋 Название:** `getVerticesArray`

**🎯 Предназначение:** Базовая функция преобразования вершин в массив координат

**📥 Параметры:**
- `vertices: VerticesType[]` - массив вершин

**⚙️ Что делает в процессе:**
1. Фильтрует валидные вершины
2. Извлекает x, y, z координаты
3. Формирует плоский массив

**📤 Что отдает на выходе:** `Array<number>` - массив координат

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (базовое преобразование)

### 26. 🎲 getRandomColor

**📋 Название:** `getRandomColor`

**🎯 Предназначение:** Генерация случайного цвета

**📥 Параметры:** Нет

**⚙️ Что делает в процессе:**
1. Генерирует случайные RGB значения
2. Преобразует в HEX формат
3. Обеспечивает читаемость цвета

**📤 Что отдает на выходе:** `string` - случайный цвет в формате #RRGGBB

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (генерация цветов)

### 27. 📁 loadDXF

**📋 Название:** `loadDXF`

**🎯 Предназначение:** Загрузка и парсинг DXF файлов

**📥 Параметры:**
- `file: File` - DXF файл для загрузки

**⚙️ Что делает в процессе:**
1. Читает содержимое DXF файла
2. Парсит структуру DXF
3. Извлекает геометрические данные
4. Преобразует в внутренний формат

**📤 Что отдает на выходе:** `Promise<ParsedDXFData>` - распарсенные данные DXF

**🔧 Тип функции:** 🎯 **Основная функция** (загрузка файлов)

### 28. 🔄 transformMainWDT_ToOrder_Z

**📋 Название:** `transformMainWDT_ToOrder_Z`

**🎯 Предназначение:** Трансформация основных данных с сортировкой по Z-координате

**📥 Параметры:**
- `mainData: MainDataType` - основные данные
- `orderZ: boolean = true` - флаг сортировки по Z

**⚙️ Что делает в процессе:**
1. Анализирует Z-координаты всех элементов
2. Сортирует элементы по Z-уровням
3. Группирует по этажам
4. Оптимизирует порядок для рендеринга

**📤 Что отдает на выходе:** `TransformedData` - трансформированные данные

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (трансформация данных)

### 29. 📋 getPreparedDataFromUniqueGroups

**📋 Название:** `getPreparedDataFromUniqueGroups`

**🎯 Предназначение:** Подготовка данных из уникальных групп элементов

**📥 Параметры:**
- `uniqueGroups: UniqueGroupsType` - уникальные группы

**⚙️ Что делает в процессе:**
1. Анализирует уникальные группы
2. Извлекает общие характеристики
3. Подготавливает данные для UI
4. Оптимизирует структуру данных

**📤 Что отдает на выходе:** `PreparedGroupData` - подготовленные данные групп

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (подготовка данных)

### 30. 🏢 getPreparedUniqueFloors

**📋 Название:** `getPreparedUniqueFloors`

**🎯 Предназначение:** Подготовка данных уникальных этажей

**📥 Параметры:**
- `floorsData: FloorsDataType` - данные этажей

**⚙️ Что делает в процессе:**
1. Анализирует данные этажей
2. Выделяет уникальные характеристики
3. Группирует по типам конструкций
4. Подготавливает для отображения

**📤 Что отдает на выходе:** `UniqueFloorsData` - данные уникальных этажей

**🔧 Тип функции:** 🔧 **Вспомогательная функция** (анализ этажей)

---

## 🔗 Полные цепочки вызовов от UI до Backend

### Реальные UI кнопки и их цепочки вызовов

### 1. Кнопка "ZIP архив (параллельно)" - DocumentGenerator.tsx
**UI Click** → **Frontend Handler** → **Redux Thunk** → **Web Worker** → **WASM Backend** → **Rust Backend**
1. **UI**: Клик на кнопку `onClick={handleGenerateDocumentsParallel}`
2. **Frontend**: `handleGenerateDocumentsParallel()` в DocumentGenerator.tsx
3. **Redux**: `dispatch(generateDocumentsParallelThunk(selectedFloors))`
4. **Thunk**: Создание Web Worker для каждого этажа
5. **Web Worker**: Параллельная обработка этажей
6. **WASM**: `generate_document_wasm(floorData)` для каждого этажа
7. **Rust**: Генерация DOCX документов в параллельных потоках
8. **Frontend**: Сборка ZIP архива из готовых документов
9. **UI**: Автоматическое скачивание ZIP файла

### 2. Кнопка "Получить Excel" - CreateNewGroupPlanesButton.tsx
**UI Click** → **Frontend Handler** → **Redux Thunk** → **WASM Backend** → **Rust Backend**
1. **UI**: Клик на кнопку `onClick={saveXlsx}`
2. **Frontend**: `saveXlsx()` в CreateNewGroupPlanesButton.tsx
3. **Data Preparation**: 
   - `getPreparedUniqueFloor(filteredItems)`
   - `getPreparedDataFromUniqueGroups(choosedUniques)`
   - Формирование `summaryData` с диаметрами и JSON
4. **WASM Init**: `await init()` - инициализация WASM модуля
5. **Redux**: `dispatch(fetchExcelViewData({diameters, floorsJson}))`
6. **WASM**: `create_custom_sortament_report(diameters, floorsJson)`
7. **Rust**: Генерация Excel отчета с сортаментом
8. **Frontend**: `saveFile(combinedData)` - создание Blob и скачивание

### 3. Кнопка "Получить документ" - DocumentGenerator.tsx
**UI Click** → **Frontend Handler** → **Redux Thunk** → **WASM Backend** → **Rust Backend**
1. **UI**: Клик на кнопку `onClick={handleGenerateDocument}`
2. **Frontend**: `handleGenerateDocument()` в DocumentGenerator.tsx
3. **Redux**: `dispatch(generateDocumentThunk(selectedFloor))`
4. **Thunk**: Подготовка данных этажа для генерации
5. **WASM**: `generate_single_document_wasm(floorData)`
6. **Rust**: Генерация DOCX документа для одного этажа
7. **Frontend**: Обработка результата и скачивание файла

### 4. Кнопки управления Canvas - Canvas.tsx
**UI Click** → **Frontend Handler** → **State Update** → **Canvas Rerender**

#### Зум IN:
1. **UI**: Клик на кнопку `onClick={handleZoomIn}`
2. **Frontend**: `handleZoomIn()` → `setScale(prev => Math.min(prev * 1.2, 5))`
3. **State**: Обновление scale в useState
4. **Canvas**: Перерисовка с новым масштабом

#### Зум OUT:
1. **UI**: Клик на кнопку `onClick={handleZoomOut}`
2. **Frontend**: `handleZoomOut()` → `setScale(prev => Math.max(prev / 1.2, 0.5))`
3. **State**: Обновление scale в useState
4. **Canvas**: Перерисовка с новым масштабом

#### Сброс зума:
1. **UI**: Клик на кнопку `onClick={handleResetZoom}`
2. **Frontend**: `handleResetZoom()` → `setScale(1)`
3. **State**: Сброс scale к значению 1
4. **Canvas**: Перерисовка с исходным масштабом

### 5. Кнопка "Создать унификацию" - CreateUniqueItem.tsx
**UI Click** → **Frontend Handler** → **Redux Action** → **State Update**
1. **UI**: Заполнение формы (название, цвет) и клик `onClick={dispatch(addToGroupUniqueItem(...))}`
2. **Frontend**: Сбор данных формы и выбранных этажей
3. **Data Preparation**:
   - Генерация UUID для группы
   - Получение `choosedPlains` из Redux state
   - Формирование объекта унификации с `planes`, `maxAsValues`, `steps`
4. **Redux**: `dispatch(addToGroupUniqueItem(unificationData))`
5. **State**: Добавление новой группы унификации в `groupUniqueItems`
6. **UI**: Закрытие модального окна и обновление списка

### 6. Кнопка "Задать арматуры" - ArmSettings.tsx
**UI Click** → **Frontend Handler** → **Modal Open** → **State Update**
1. **UI**: Клик на кнопку `onClick={setIsOpen(!isOpen)}`
2. **Frontend**: `setIsOpen()` - переключение состояния модального окна
3. **State**: Обновление `isOpen` в useState
4. **UI**: Открытие модального окна с компонентом `AdditionInfoArm`

### 7. Кнопка "получить комбинации" - ArmSettings.tsx (внутри модала)
**UI Click** → **Frontend Handler** → **WASM Backend** → **Rust Backend**
1. **UI**: Клик на кнопку `onClick={saveXlsx}` внутри модала
2. **Frontend**: `saveXlsx()` в ArmSettings.tsx
3. **WASM Init**: `await init()` - инициализация WASM модуля
4. **WASM**: `get_excell_report_for_arms()` - без параметров
5. **Rust**: Генерация Excel отчета для арматуры
6. **Frontend**: `saveFile(combinedData)` - создание Blob и скачивание Excel

### 8. Кнопка "Создать унификацию" - CreateNewGroupPlanesButton.tsx
**UI Click** → **Frontend Handler** → **Modal Control** → **State Update**
1. **UI**: Клик на кнопку после проверки `checkAllStepsAreEqual()`
2. **Frontend**: Проверка одинаковости шагов выбранных этажей
3. **Validation**: Если шаги не равны - показ toast ошибки
4. **State**: `setOpenForCreateUI(!openForCreateUI)` - переключение UI создания
5. **UI**: Открытие/закрытие интерфейса создания унификации

## 📋 Полная схема UI кнопок проекта

### Кнопки генерации документов:
1. **"ZIP архив (параллельно)"** → `DocumentGenerator.tsx` → Параллельная генерация DOCX → ZIP скачивание
2. **"Получить документ"** → `DocumentGenerator.tsx` → Одиночная генерация DOCX → Файл скачивание
3. **"Получить Excel"** → `CreateNewGroupPlanesButton.tsx` → WASM сортамент → Excel скачивание
4. **"получить комбинации"** → `ArmSettings.tsx` → WASM арматура → Excel скачивание

### Кнопки управления данными:
5. **"Создать унификацию"** → `CreateUniqueItem.tsx` → Redux добавление → State обновление
6. **"Создать унификацию"** → `CreateNewGroupPlanesButton.tsx` → UI переключение → Modal контроль
7. **"Задать арматуры"** → `ArmSettings.tsx` → Modal открытие → UI показ

### Кнопки управления Canvas:
8. **"Зум IN"** → `Canvas.tsx` → Scale увеличение → Canvas перерисовка
9. **"Зум OUT"** → `Canvas.tsx` → Scale уменьшение → Canvas перерисовка  
10. **"Сброс зума"** → `Canvas.tsx` → Scale сброс → Canvas перерисовка

### Архитектурные паттерны цепочек:
- **UI → WASM → Rust → Download**: Кнопки генерации файлов (1,2,3,4)
- **UI → Redux → State**: Кнопки управления данными (5,6)
- **UI → useState → Render**: Кнопки UI контроля (7,8,9,10)

## 📊 ВИЗУАЛЬНЫЕ ДИАГРАММЫ

### 🎨 Архитектурные диаграммы
- **[Общая архитектура системы](./image/flow/architecture-overview.svg)** - Полный обзор всех слоев системы
- **[ZIP архив - цепочка вызовов](./image/flow/zip-button-flow.svg)** - Детальная диаграмма параллельной обработки
- **[Excel - цепочка вызовов](./image/flow/excel-button-flow.svg)** - Процесс генерации Excel файлов
- **[Document - цепочка вызовов](./image/flow/document-button-flow.svg)** - Создание DOCX документов

> 💡 **Примечание:** Диаграммы созданы в формате SVG и показывают полные цепочки вызовов от UI до Rust с указанием технологий, используемых на каждом уровне.

## 🔗 ПОЛНЫЕ ЦЕПОЧКИ ВЫЗОВОВ ФУНКЦИЙ

### 1. 📦 Кнопка "ZIP архив (параллельно)"

```
🖱️ UI КЛИК
    ↓
📄 DocumentGenerator.tsx → handleGenerateDocumentsParallel()
    ↓
🔧 WASM init() → инициализация модуля
    ↓
📊 excelViewData.forEach() → сбор данных комбинаций
    ↓
👷 new Worker(parallel-document-worker.ts) → создание веб-воркеров
    ↓
📨 worker.postMessage() → отправка данных в воркер
    ↓
🔄 parallel-document-worker.ts → self.onmessage()
    ↓
🦀 [WASM] create_docx_for_single_floor() → lib.rs:534
    ↓
🦀 [RUST] create_docx_for_selected_floors() → docx_generator.rs:200
    ↓
🦀 [RUST] sort_by_z() → группировка по этажам
    ↓
🦀 [RUST] calculate_layout() → расчет размещения
    ↓
🦀 [RUST] draw_image_as1() → drawItem.rs:374
    ↓
🦀 [RUST] draw_image() → drawItem.rs:650
    ↓
🦀 [RUST] draw_image_with_floor() → drawItem.rs:654
    ↓
🦀 [RUST] ImageBuffer::from_fn() → создание изображения
    ↓
🦀 [RUST] Docx::new() → создание DOCX документа
    ↓
📤 worker.postMessage(result) → возврат результата
    ↓
📦 JSZip() → создание ZIP архива
    ↓
💾 saveAs() → скачивание файла
```

### 2. 📊 Кнопка "Получить Excel"

```
🖱️ UI КЛИК
    ↓
🔘 CreateNewGroupPlanesButton.tsx → onClick()
    ↓
📄 saveXlsx() → функция сохранения Excel
    ↓
🔧 WASM init() → инициализация модуля
    ↓
🔄 Redux dispatch(fetchExcelViewData()) → wasmThanks.ts:47
    ↓
🦀 [WASM] get_table_data_for_frontend() → lib.rs:380
    ↓
🦀 [RUST] serde_json::from_str() → парсинг JSON данных
    ↓
🦀 [RUST] CustomSortament::from_js_data() → создание сортамента
    ↓
🦀 [RUST] generate_excel_data_for_js() → custom_sortament.rs:534
    ↓
🦀 [RUST] find_combinations_for_area_with_limits() → custom_sortament.rs:444
    ↓
🦀 [RUST] get_available_diameters() → получение диаметров
    ↓
🦀 [RUST] get_area() → расчет площади арматуры
    ↓
🦀 [RUST] sort_by() → сортировка по отклонению
    ↓
🦀 [RUST] serde_json::to_string() → сериализация результата
    ↓
🔄 Redux state.excelViewData = action.payload → обновление состояния
    ↓
🦀 [WASM] create_custom_sortament_report() → создание Excel файла
    ↓
💾 saveFile() → скачивание Excel файла
```

### 3. 📄 Кнопка "Получить документ"

```
🖱️ UI КЛИК
    ↓
📄 DocumentGenerator.tsx → handleGenerateDocument()
    ↓
📊 excelViewData.forEach() → сбор выбранных комбинаций
    ↓
🔍 item.is_default_checked → фильтрация выбранных
    ↓
📦 SelectedCombinationsData → подготовка данных
    ↓
🔄 Redux dispatch(generateDocumentWithColorPalette()) → wasmThanks.ts:63
    ↓
🔧 WASM init() → инициализация модуля
    ↓
🦀 [WASM] create_docx_with_selected_combinations() → lib.rs:396
    ↓
🦀 [RUST] serde_json::from_str() → парсинг JSON комбинаций
    ↓
🦀 [RUST] SelectedCombinationsData → десериализация данных
    ↓
🦀 [RUST] HashMap::new() → группировка по этажам и функциям
    ↓
🦀 [RUST] GLOBAL_ENTITIES.with() → получение глобальных данных
    ↓
🦀 [RUST] create_docx_for_selected_floors() → docx_generator.rs:200
    ↓
🦀 [RUST] sort_by_z() → группировка сущностей по Z-координате
    ↓
🦀 [RUST] Docx::new() → создание DOCX документа
    ↓
🦀 [RUST] page_margin() → настройка полей страницы
    ↓
🦀 [RUST] page_size() → настройка размера страницы
    ↓
🦀 [RUST] page_orient() → альбомная ориентация
    ↓
🦀 [RUST] calculate_layout() → расчет размещения изображений
    ↓
🦀 [RUST] draw_image_with_floor() → drawItem.rs:654
    ↓
🦀 [RUST] calculate_image_bounds_with_config() → расчет границ
    ↓
🦀 [RUST] generate_color_palette() → генерация цветовой палитры
    ↓
🦀 [RUST] ImageBuffer::from_fn() → создание изображения
    ↓
🦀 [RUST] add_paragraph() → добавление параграфов в DOCX
    ↓
🦀 [RUST] add_image() → добавление изображений в DOCX
    ↓
📤 Vec<u8> → возврат байтов документа
    ↓
💾 Blob() → создание blob объекта
    ↓
💾 URL.createObjectURL() → создание URL для скачивания
    ↓
💾 link.download → скачивание DOCX файла
```

## 🎯 АРХИТЕКТУРНЫЕ ПАТТЕРНЫ

### Паттерн 1: UI → Frontend → WASM → Rust → Download
- **Применяется для:** Генерация файлов (ZIP, Excel, DOCX)
- **Особенности:** Тяжелые вычисления в Rust, асинхронная обработка

### Паттерн 2: UI → Redux → State → Re-render  
- **Применяется для:** Управление состоянием приложения
- **Особенности:** Реактивные обновления интерфейса

### Паттерн 3: UI → useState → Component Update
- **Применяется для:** Локальных изменений компонентов
- **Особенности:** Быстрые UI обновления без глобального состояния

## 🔧 ТЕХНИЧЕСКИЕ ДЕТАЛИ

### WASM Интеграция
- **Инициализация:** `init()` перед каждым вызовом
- **Передача данных:** JSON сериализация/десериализация
- **Обработка ошибок:** `Result<T, JsValue>` паттерн

### Rust Backend
- **Глобальное состояние:** `GLOBAL_ENTITIES` для кэширования
- **Параллелизация:** Web Workers для тяжелых операций
- **Генерация файлов:** Специализированные библиотеки (docx-rs, image)

### Frontend Integration
- **Redux Thunks:** Асинхронные операции с WASM
- **Error Handling:** Try-catch с пользовательскими уведомлениями
- **File Downloads:** Blob API для скачивания файлов

### Системные цепочки вызовов

### Цепочка 1: Загрузка и парсинг данных проекта
```
UI (App.tsx - загрузка файлов) 
    ↓ [FRONTEND]
    → handleFileUpload() → dispatch(fetchWasmData({sliData, txtData, xlsxData}))
    ↓ [REDUX THUNK]
    → wasmThanks.ts → fetchWasmData() → init() 
    ↓ [WASM BACKEND]
    → parse_data(sliData, txtData, xlsxData) → convert_sli_xsl_to_json_string()
    ↓ [RUST BACKEND]
    → parse_sli_data() → parse_txt_data() → parse_xlsx_data() → merge_data_sources() → serialize_to_json()
```

### Цепочка 2: Получение горизонтальных элементов
```
UI (App.tsx - после загрузки) 
    ↓ [FRONTEND]
    → dispatch(fetchWasmJSData())
    ↓ [REDUX THUNK]
    → wasmThanks.ts → fetchWasmJSData() → init()
    ↓ [WASM BACKEND]
    → get_horizontal_elements_object_js()
    ↓ [RUST BACKEND]
    → extract_horizontal_elements() → filter_by_type() → transform_to_js_format() → serialize_elements()
    ↓ [FRONTEND HELPER]
    → getPureWASMJsData(result) → transformLinesPointsIntoArray() → getLayersMap() → getLayerColor()
```

### Цепочка 3: Генерация Excel данных
```
UI (CreateNewGroupPlanesButton.tsx - клик кнопки)
    ↓ [FRONTEND]
    → handleCreateGroup() → dispatch(fetchExcelViewData({diameters, floorsJson}))
    ↓ [REDUX THUNK]
    → wasmThanks.ts → fetchExcelViewData() → init()
    ↓ [WASM BACKEND]
    → get_table_data_for_frontend(diameters, floorsJson)
    ↓ [RUST BACKEND]
    → parse_floor_data() → calculate_diameters() → generate_table_rows() → format_excel_data() → serialize_table()
```

### Цепочка 4: Генерация DOCX документа
```
UI (DocumentGenerator.tsx - генерация документа)
    ↓ [FRONTEND]
    → handleGenerateDocument() → dispatch(generateDocumentWithColorPalette(selectedData))
    ↓ [REDUX THUNK]
    → wasmThanks.ts → generateDocumentWithColorPalette() → init()
    ↓ [WASM BACKEND]
    → create_docx_with_selected_combinations(selectedCombinationsJson)
    ↓ [RUST BACKEND]
    → parse_combinations() → create_document_structure() → add_tables() → add_images() → generate_docx_bytes()
```

### Цепочка 5: Параллельная генерация документов этажей
```
UI (Multi-floor generation)
    ↓ [FRONTEND]
    → createFloorWorkers() → new Worker(floor-document-worker.ts)
    ↓ [WEB WORKER]
    → floor-document-worker.ts → parse_data(sliData, txtData, xlsxData)
    ↓ [WASM BACKEND]
    → create_docx_for_single_floor(floorLevel, "", selectedCombinationsJson)
    ↓ [RUST BACKEND]
    → filter_floor_data() → process_single_floor() → create_floor_document() → generate_floor_docx()
```

### Цепочка 6: Получение данных сортамента
```
UI (Sortament component)
    ↓ [FRONTEND]
    → loadSortamentData() → dispatch(fetchArmDimeters())
    ↓ [REDUX THUNK]
    → wasmThanks.ts → fetchArmDimeters() → init()
    ↓ [WASM BACKEND]
    → get_sortament_data()
    ↓ [RUST BACKEND]
    → load_sortament_table() → parse_diameter_data() → calculate_areas() → format_arm_diameters()
```

### Цепочка 7: Обработка файлов через process_files
```
UI (File processor)
    ↓ [FRONTEND]
    → handleFileProcessing() → processFiles()
    ↓ [WASM BACKEND]
    → process_files(fileData)
    ↓ [RUST BACKEND]
    → validate_files() → extract_metadata() → process_geometry() → optimize_data() → return_processed_result()
```

### Цепочка 8: Инициализация GPU рендерера
```
UI (3D Viewer initialization)
    ↓ [FRONTEND]
    → init3DViewer() → setupGPURenderer()
    ↓ [WASM BACKEND]
    → initialize_gpu_renderer()
    ↓ [RUST BACKEND]
    → setup_webgl_context() → compile_shaders() → create_buffers() → initialize_matrices()
```

### Цепочка 9: Загрузка DXF файла
```
UI (DXF upload) 
    ↓ [FRONTEND]
    → handleDXFUpload() → loadDXF(file)
    ↓ [FRONTEND HELPER]
    → readFileContent() → parseDXFStructure() → extractGeometry()
    ↓ [FRONTEND PROCESSING]
    → getLayersMap() → getLayerColor() → getLinesVerticesArray() → transformLinesPointsIntoArray()
```

### Цепочка 10: Подготовка данных для 3D визуализации
```
UI (3D viewer) 
    ↓ [FRONTEND]
    → prepare3DData() → transformMainWDT_ToOrder_Z()
    ↓ [FRONTEND PROCESSING]
    → getPreparedDataFromUniqueGroups() → getVerticesArray() → getFacesVerticesArray()
    ↓ [FRONTEND RENDERING]
    → createGeometryBuffers() → setupMaterials() → renderScene()
```

### Цепочка 11: Анализ этажей
```
UI (Floor analysis)
    ↓ [FRONTEND]
    → analyzeFloors() → getPreparedUniqueFloors()
    ↓ [FRONTEND PROCESSING]
    → getPreparedDataFromUniqueGroups() → groupByFloorLevel() → calculateStatistics()
```

---

## 📊 Финальная статистика функций

**🎯 Основные функции (11):**
- parse_data
- get_horizontal_elements_object_js  
- create_docx
- create_docx_with_image
- create_docx_with_selected_combinations
- get_table_data_for_frontend
- get_sortament_data
- process_files
- create_docx_for_single_floor
- loadDXF
- getPureWASMJsData

**🔧 Вспомогательные функции (17):**
- convert_sli_xsl_to_json_string
- transformLinesPointsIntoArray
- entity_to_js
- initialize_gpu_renderer
- new_draw_polygon
- sort_by_same_z
- get_processed_data_for_frontend
- create_partial_images
- get_optimized_canvas_data_wasm
- get_canvas_statistics_wasm
- getLayerColor
- getLayersMap
- getLinesVerticesArray
- getFacesVerticesArray
- getVerticesArray
- getRandomColor
- transformMainWDT_ToOrder_Z
- getPreparedDataFromUniqueGroups
- getPreparedUniqueFloors

**🧪 Тестовые функции (2):**
- create_test_docx_with_images
- create_test_image

**📈 Общее количество функций:** 30