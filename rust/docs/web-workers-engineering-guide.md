# 🎓 ИНЖЕНЕРНОЕ РУКОВОДСТВО: WEB WORKERS + WASM ПАРАЛЛЕЛИЗМ

## 📋 ОГЛАВЛЕНИЕ

1. [Архитектура системы](#архитектура-системы)
2. [Поток данных](#поток-данных)
3. [Жизненный цикл воркера](#жизненный-цикл-воркера)
4. [Временная диаграмма](#временная-диаграмма)
5. [Обязательные компоненты](#обязательные-компоненты)
6. [Файловая структура](#файловая-структура)

---

## 🏗️ АРХИТЕКТУРА СИСТЕМЫ

### Общая схема компонентов

```mermaid
graph TD
    subgraph DOCGEN["DocumentGenerator.tsx - КООРДИНАТОР"]
        BTN["Button onClick() - строка 200+"]
        LOOP["for 50 воркеров"]
        CREATE["new Worker(parallel-image-worker.ts)"]
        POST["worker.postMessage({workerId, startIndex})"]
        PROMISE["Promise.all(воркеры)"]
    end
    
    subgraph WORKER["parallel-image-worker.ts - ИСПОЛНИТЕЛЬ"]
        ONMSG["self.onmessage - строка 6"]
        IMPORT["await import('../assets/pkg/rst_build.js')"]
        CALL["wasmModule.create_partial_images()"]
        CONVERT["for loop: Uint8Array conversion"]
        SEND["self.postMessage({success, images})"]
    end
    
    subgraph RUST["lib.rs - WASM МОДУЛЬ"]
        BIND["#[wasm_bindgen] create_partial_images()"]
        IMG["create_test_image() generation"]
        RETURN["js_sys::Array return"]
    end
    
    BTN --> LOOP
    LOOP --> CREATE
    CREATE --> POST
    POST --> ONMSG
    ONMSG --> IMPORT
    IMPORT --> CALL
    CALL --> BIND
    BIND --> IMG
    IMG --> RETURN
    RETURN --> CONVERT
    CONVERT --> SEND
    SEND --> PROMISE
```

---

## 🔄 ПОТОК ДАННЫХ

### Детальная схема обмена сообщениями

```mermaid
sequenceDiagram
    participant UI as DocumentGenerator.tsx
    participant WM as WASM Module
    participant W1 as Worker 1
    participant RS as lib.rs

    Note over UI: КОНКРЕТНЫЙ КОД ВЗАИМОДЕЙСТВИЯ
    
    UI->>UI: onClick() в строке 200+
    Note right of UI: const worker = new Worker(new URL('../workers/parallel-image-worker.ts'))
    
    UI->>W1: worker.postMessage({workerId, startIndex, imageCount})
    Note right of UI: Код: event.data из строки 6 parallel-image-worker.ts
    
    W1->>WM: await import('../assets/pkg/rst_build.js')
    Note right of W1: Строка 10: const wasmModule = await import
    
    W1->>RS: wasmModule.create_partial_images(startIndex, imageCount, complexity)
    Note right of W1: Строка 16: typeof wasmModule.create_partial_images === 'function'
    
    RS->>RS: create_partial_images() execution
    Note right of RS: WASM bindgen функция в lib.rs
    
    RS-->>W1: js_sys::Array с изображениями
    Note right of RS: return images array
    
    W1->>W1: Convert WASM to Uint8Array[]
    Note right of W1: Строки 25-35: for loop conversion
    
    W1->>UI: self.postMessage({success: true, images: []})
    Note right of W1: Строка 36-45: результат обратно в UI
    
    UI->>UI: Promise.all() синхронизация
    Note right of UI: Ожидание всех 50 воркеров
```

---

## ⚙️ ЖИЗНЕННЫЙ ЦИКЛ ВОРКЕРА

### Состояния и переходы воркера

```mermaid
stateDiagram
    [*] --> Creating
    Creating --> Initializing
    Initializing --> Ready
    Ready --> Loading
    Loading --> Processing
    Processing --> Converting
    Converting --> Sending
    Sending --> Terminating
    Terminating --> [*]
  
    Creating : new Worker()
    Initializing : Worker thread started
    Ready : self.onmessage registered
    Loading : Received postMessage
    Processing : WASM module loaded
    Converting : create_partial_images() done
    Sending : Data converted to JS
    Terminating : terminate() called
```

---

## ⏱️ ВРЕМЕННАЯ ДИАГРАММА

### Параллельное выполнение vs Последовательное

```mermaid
gantt
    title Сравнение производительности
    dateFormat YYYY-MM-DD
    axisFormat %d
  
    section Последовательное
    Изображения 0-99    :done, seq1, 2024-01-01, 47d
    Изображения 100-199 :done, seq2, after seq1, 47d
    Изображения 200-299 :done, seq3, after seq2, 47d
  
    section Параллельное
    Worker 1 0-99       :done, par1, 2024-01-01, 47d
    Worker 2 100-199    :done, par2, 2024-01-01, 47d
    Worker 3 200-299    :done, par3, 2024-01-01, 47d
```

**📊 РЕЗУЛЬТАТ:** Ускорение в **4.45 раза** (236с → 53с)

---

## 🔴 ОБЯЗАТЕЛЬНЫЕ КОМПОНЕНТЫ

```mermaid
graph TB
    ROOT[WEB WORKERS СИСТЕМА]
  
    subgraph API["ОБЯЗАТЕЛЬНЫЕ API"]
        WORKER[new Worker - Создание потока]
        POST[postMessage - Канал связи]
        ON[onmessage - Event-driven модель]
        TERM[terminate - Освобождение памяти]
    end
  
    subgraph FILES["ОБЯЗАТЕЛЬНЫЕ ФАЙЛЫ"]
        DOC[DocumentGenerator.tsx - Координатор]
        PAR[parallel-image-worker.ts - Исполнитель]
        LIB[lib.rs - WASM функции]
    end
  
    subgraph TYPES["КРИТИЧЕСКИЕ ТИПЫ"]
        ARR[js_sys::Array - WASM совместимость]
        UINT[Uint8Array - Бинарные данные]
        PROM[Promise - Асинхронность]
    end
  
    ROOT --> API
    ROOT --> FILES
    ROOT --> TYPES
```

---

## 📁 ФАЙЛОВАЯ СТРУКТУРА

### Организация проекта

```mermaid
graph TD
    ROOT[wasm_3d/]
  
    subgraph FRONTEND["FRONTEND TypeScript React"]
        SRC[src/]
        COMP[components/]
        WORK[workers/]
        PKG[assets/pkg/]
      
        DOCGEN[DocumentGenerator.tsx - Координатор]
        WORKER[parallel-image-worker.ts - Исполнитель]
        WASMJS[rst_build.js - WASM JS биндинги]
        WASMBIN[rst_build_bg.wasm - WASM бинарник]
    end
  
    subgraph BACKEND["BACKEND Rust WASM"]
        RUST[rust/]
        SRCRUST[src/]
        CARGO[Cargo.toml - Конфигурация]
        DOCS[docs/]
      
        LIB[lib.rs - WASM функции]
        GUIDE[web-workers-engineering-guide.md]
    end
  
    ROOT --> FRONTEND
    ROOT --> BACKEND
  
    SRC --> COMP
    SRC --> WORK
    SRC --> PKG
    COMP --> DOCGEN
    WORK --> WORKER
    PKG --> WASMJS
    PKG --> WASMBIN
  
    RUST --> SRCRUST
    RUST --> CARGO
    RUST --> DOCS
    SRCRUST --> LIB
    DOCS --> GUIDE
  
    LIB -.-> WASMJS
    LIB -.-> WASMBIN
```

---

## 💻 ДЕТАЛЬНЫЙ КОД С ПРИВЯЗКОЙ К ФАЙЛАМ

### 🎯 ТОЧКА ВХОДА: DocumentGenerator.tsx (строка 200+)

```typescript
// ФАЙЛ: c:\workProject\react-3d\wasm_3d\src\components\DocumentGenerator.tsx
// СТРОКИ: 200+

const worker = new Worker(
    new URL('../workers/parallel-image-worker.ts', import.meta.url),
    { type: 'module' }
);

worker.postMessage({ 
    workerId: i + 1, 
    startIndex: i * 100, 
    imageCount: 100, 
    complexity: 100 
});

worker.onmessage = (event) => {
    const totalTime = performance.now() - startTime;
    console.log(`🧪 Worker ${event.data.workerId} завершен`);
    worker.terminate();
};
```

### ⚙️ ВОРКЕР: parallel-image-worker.ts

```typescript
// ФАЙЛ: c:\workProject\react-3d\wasm_3d\src\workers\parallel-image-worker.ts
// СТРОКИ: 1-56 (весь файл)

// СТРОКА 6: Точка входа воркера
self.onmessage = async (event) => {
    const { workerId, startIndex, imageCount, complexity } = event.data;

    try {
        const totalStart = performance.now();

        // СТРОКА 10: Импорт WASM модуля
        const wasmModule = await import('../assets/pkg/rst_build.js');
        if (typeof wasmModule.default === 'function') {
            await wasmModule.default();
        }

        // СТРОКА 16: Проверка функции и вызов
        if (typeof wasmModule.create_partial_images === 'function') {
            const funcStart = performance.now();
            const imagesArray = await wasmModule.create_partial_images(startIndex, imageCount, complexity);
            const funcTime = Math.round(performance.now() - funcStart);

            // СТРОКА 22: ТОЛЬКО ВРЕМЯ!
            console.log(`${funcTime}мс`);

            // СТРОКИ 25-35: Конвертация WASM данных в JS
            const images = [];
            let totalSize = 0;

            for (let i = 0; i < imagesArray.length; i++) {
                const uint8Array = imagesArray[i] as Uint8Array;
                const imageBytes = new Uint8Array(uint8Array);
                images.push(imageBytes);
                totalSize += imageBytes.length;
            }

            // СТРОКА 36-45: Отправка результата обратно
            self.postMessage({
                success: true,
                workerId: workerId,
                functionTime: funcTime,
                totalTime: Math.round(performance.now() - totalStart),
                imageCount: images.length,
                totalSize: totalSize,
                images: images,
                startIndex: startIndex
            });
        }
    } catch (error: unknown) {
        // СТРОКА 50+: Обработка ошибок
        self.postMessage({
            success: false,
            workerId: workerId,
            error: (error as Error).message || String(error)
        });
    }
};
```

### 🦀 RUST WASM: lib.rs

```rust
// ФАЙЛ: c:\workProject\react-3d\wasm_3d\rust\src\lib.rs
// ИМПОРТЫ И ОБЪЯВЛЕНИЯ

use wasm_bindgen::prelude::*;
use web_sys::console;
use docx_rs::{Docx, Paragraph, Pic, Run};
use image::{ImageBuffer, ImageOutputFormat, Rgb, RgbImage};

// СТРОКА 15+: Логирование для отладки
#[wasm_bindgen]
pub fn log_data(x: f64, y: f64) {
    console::log_1(&format!("Координаты: x = {}, y = {}", x, y).into());
}

// ОСНОВНАЯ ФУНКЦИЯ ДЛЯ ПАРАЛЛЕЛЬНОЙ ГЕНЕРАЦИИ
#[wasm_bindgen]
pub fn create_partial_images(
    start_index: usize, 
    count: usize, 
    complexity: usize
) -> js_sys::Array {
    let images = js_sys::Array::new();
    
    for i in 0..count {
        let image_data = create_test_image(start_index + i, complexity);
        let uint8_array = js_sys::Uint8Array::from(&image_data[..]);
        images.push(&uint8_array);
    }
    
    images
}

// ГЕНЕРАЦИЯ ОДНОГО ИЗОБРАЖЕНИЯ
pub fn create_test_image(index: usize, complexity: usize) -> Vec<u8> {
    let width = 680;
    let height = 900;
    let mut img = ImageBuffer::from_fn(width, height, |_, _| Rgb([255u8, 255u8, 255u8]));
    
    // Генерация контента изображения
    // ... код рисования ...
    
    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Png)
        .unwrap();
    buffer
}
```

---

## 🔗 КОНКРЕТНЫЕ СВЯЗИ МЕЖДУ КОДОМ

```mermaid
flowchart TD
    subgraph FILE1["DocumentGenerator.tsx"]
        CODE1["Button onClick()\nстрока 200+"]
        CODE2["new Worker()\nparallel-image-worker.ts"]
        CODE3["worker.postMessage()\n{workerId, startIndex}"]
        CODE4["Promise.all()\nожидание 50 воркеров"]
    end
    
    subgraph FILE2["parallel-image-worker.ts"]
        CODE5["self.onmessage\nстрока 6"]
        CODE6["import('../assets/pkg/rst_build.js')\nстрока 10"]
        CODE7["wasmModule.create_partial_images()\nстрока 16"]
        CODE8["self.postMessage()\nстрока 36-45"]
    end
    
    subgraph FILE3["lib.rs"]
        CODE9["#[wasm_bindgen]\ncreate_partial_images()"]
        CODE10["create_test_image()\nгенерация PNG"]
        CODE11["js_sys::Array\nreturn данные"]
    end
    
    CODE1 --> CODE2
    CODE2 --> CODE3
    CODE3 --> CODE5
    CODE5 --> CODE6
    CODE6 --> CODE7
    CODE7 --> CODE9
    CODE9 --> CODE10
    CODE10 --> CODE11
    CODE11 --> CODE8
    CODE8 --> CODE4
```

---

## 🔧 ТЕХНИЧЕСКИЕ ДЕТАЛИ РЕАЛИЗАЦИИ

### 📍 КОНКРЕТНЫЕ ВЫЗОВЫ ФУНКЦИЙ

```mermaid
graph TB
    subgraph REACT["React Component"]
        BTN1["onClick() event\nDocumentGenerator.tsx:200+"]
        LOOP1["for (let i = 0; i < 50; i++)"]
        WORKER1["new Worker(parallel-image-worker.ts)"]
        POST1["worker.postMessage({workerId: i+1, startIndex: i*100})"]
    end
    
    subgraph WORKER_THREAD["Worker Thread"]
        HANDLER["self.onmessage = async (event)\nparallel-image-worker.ts:6"]
        DESTRUCT["const {workerId, startIndex} = event.data"]
        IMPORT1["await import('../assets/pkg/rst_build.js')"]
        DEFAULT1["await wasmModule.default()"]
        CHECK1["typeof wasmModule.create_partial_images === 'function'"]
        CALL1["wasmModule.create_partial_images(startIndex, 100, 100)"]
    end
    
    subgraph RUST_WASM["Rust WASM"]
        ENTRY["#[wasm_bindgen] create_partial_images()\nlib.rs"]
        ARRAY1["let images = js_sys::Array::new()"]
        FOR1["for i in 0..count"]
        CREATE1["create_test_image(start_index + i, complexity)"]
        UINT8["js_sys::Uint8Array::from(&image_data[..])"]
        PUSH1["images.push(&uint8_array)"]
        RETURN1["return images"]
    end
    
    subgraph BACK_TO_WORKER["Back to Worker"]
        CONVERT1["for (let i = 0; i < imagesArray.length; i++)"]
        NEW1["new Uint8Array(uint8Array)"]
        MSG1["self.postMessage({success: true, images: []})"]
    end
    
    BTN1 --> LOOP1
    LOOP1 --> WORKER1
    WORKER1 --> POST1
    POST1 --> HANDLER
    HANDLER --> DESTRUCT
    DESTRUCT --> IMPORT1
    IMPORT1 --> DEFAULT1
    DEFAULT1 --> CHECK1
    CHECK1 --> CALL1
    CALL1 --> ENTRY
    ENTRY --> ARRAY1
    ARRAY1 --> FOR1
    FOR1 --> CREATE1
    CREATE1 --> UINT8
    UINT8 --> PUSH1
    PUSH1 --> RETURN1
    RETURN1 --> CONVERT1
    CONVERT1 --> NEW1
    NEW1 --> MSG1
```

```mermaid
classDiagram
    class DocumentGenerator {
        +onClick() void
        +createWorkers() Worker[]
        +distributeWork() Promise[]
        +collectResults() Array
    }
  
    class ParallelImageWorker {
        +onmessage(event) void
        +loadWASM() Module
        +processImages() Uint8Array[]
        +sendResults() void
    }
  
    class WASMModule {
        +create_partial_images() js_sys_Array
        +create_test_image() Vec_u8
    }
  
    DocumentGenerator --> ParallelImageWorker
    ParallelImageWorker --> WASMModule
```

---

## 📈 МЕТРИКИ ПРОИЗВОДИТЕЛЬНОСТИ

### Измеряемые параметры

```mermaid
graph LR
    subgraph PERF["ПРОИЗВОДИТЕЛЬНОСТЬ"]
        W1[Воркер 1 47с]
        W10[Воркер 10 46с]
        W20[Воркер 20 48с]
        W30[Воркер 30 45с]
        W40[Воркер 40 50с]
        W50[Воркер 50 47с]
        MAIN[Основной поток 236с]
    end
```

**🎯 КЛЮЧЕВЫЕ ПОКАЗАТЕЛИ:**

- **Параллельные воркеры:** 47-50 секунд каждый
- **Основной поток:** 236 секунд общее время
- **Ускорение:** 4.45x раза
- **Эффективность:** 89% (4.45/5 воркеров)

---

## 🔗 ВЗАИМОДЕЙСТВИЕ КОМПОНЕНТОВ

### API вызовы и контракты данных

```mermaid
graph LR
    subgraph CONTRACTS["КОНТРАКТЫ ДАННЫХ"]
        INPUT[Input Contract workerId startIndex imageCount complexity]
        OUTPUT[Output Contract success workerId imageCount images totalSize]
        ERROR[Error Contract success false workerId error]
    end
  
    subgraph API["API ГРАНИЦЫ"]
        JSRUST[JS to RUST wasm-bindgen Structured Clone]
        MAINWORKER[Main to Worker postMessage Transferable Objects]
    end
  
    INPUT --> MAINWORKER
    MAINWORKER --> JSRUST
    JSRUST --> OUTPUT
    JSRUST --> ERROR
    OUTPUT --> MAINWORKER
    ERROR --> MAINWORKER
```

---

## 🎯 ЗАКЛЮЧЕНИЕ

### Ключевые принципы успешной реализации

1. **🔴 ОБЯЗАТЕЛЬНЫЕ требования Web Workers API**

   - Отдельные файлы для воркеров
   - self.onmessage как единственная точка входа
   - postMessage для всей коммуникации
   - terminate() для освобождения памяти
2. **⚡ Эффективное использование параллелизма**

   - Правильное разделение задач
   - Балансировка нагрузки между воркерами
   - Минимизация накладных расходов на координацию
3. **🔧 Интеграция с WASM**

   - #[wasm_bindgen] аннотации
   - Правильные типы данных (js_sys::Array)
   - Эффективная конвертация данных
4. **📊 Мониторинг производительности**

   - Измерение времени выполнения
   - Анализ узких мест
   - Оптимизация критических путей

**🏆 РЕЗУЛЬТАТ:** Система обеспечивает **4.45x ускорение** при обработке 5000 изображений благодаря правильной архитектуре параллельных вычислений!
