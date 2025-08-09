use wgpu::*;
use wgpu::{BufferUsages, BufferDescriptor};
use std::sync::Arc;
use image::{ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;
use web_sys::console;
use futures;
use futures_channel;
use std::sync::Mutex;
use std::collections::HashMap;
use once_cell::sync::Lazy;

// Константы для определения оптимального метода рендеринга
// Увеличиваем пороги, так как GPU эффективен только для больших объемов данных
const GPU_THRESHOLD_IMAGES: usize = 4; // Минимальное количество изображений для GPU
const GPU_THRESHOLD_LINES: usize = 1000; // Минимальное количество линий для GPU (увеличено в 10 раз)
const GPU_THRESHOLD_BUFFER_SIZE: u64 = 100 * 1024 * 1024; // 100MB минимальный размер данных для GPU (увеличено в 2 раза)
const GPU_THRESHOLD_PIXELS: u64 = 2_000_000; // Минимальное количество пикселей для обработки

#[derive(Debug, Clone, Copy)]
pub enum RenderMethod {
    Cpu,
    Gpu,
}

pub struct GpuRenderer {
    device: Device,
    queue: Queue,
    adapter: Adapter,
    line_shader: ShaderModule,
    buffer_pool: HashMap<u64, Vec<Buffer>>, // Пул буферов по размеру
}

impl GpuRenderer {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        console::log_1(&"Creating wgpu instance...".into());
        // Создаем экземпляр wgpu
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::all(),
            flags: InstanceFlags::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
        });
        console::log_1(&"wgpu instance created successfully".into());

        // Получаем адаптер
        console::log_1(&"Requesting GPU adapter...".into());
        let adapter = match instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await {
                Ok(adapter) => {
                    console::log_1(&"GPU adapter obtained successfully".into());
                    adapter
                },
                Err(e) => {
                    let error_msg = format!("Failed to request GPU adapter: {:?}", e);
                    console::log_1(&error_msg.clone().into());
                    return Err(error_msg.into());
                }
            };

        // Получаем устройство и очередь с увеличенными лимитами буферов
        console::log_1(&"Requesting GPU device...".into());
        let mut limits = Limits::default();
        limits.max_buffer_size = 2_147_483_648; // 2GB максимальный размер буфера
        
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                required_features: Features::empty(),
                required_limits: limits,
                label: None,
                memory_hints: Default::default(),
                trace: Default::default(),
            })
            .await?;
        console::log_1(&"GPU device obtained successfully".into());

        // Создаем шейдер модуль один раз при инициализации
        let shader_source = r#"
            struct Params {
                img_width: i32,
                img_height: i32,
                line_color: u32,
                _padding: u32,
            }
            
            @group(0) @binding(0) var<storage, read_write> image_data: array<u32>;
            @group(0) @binding(1) var<storage, read> lines_data: array<f32>;
            @group(0) @binding(2) var<uniform> params: Params;
            
            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let line_index = global_id.x;
                if (line_index >= arrayLength(&lines_data) / 4u) {
                    return;
                }
                
                let x1 = i32(round(lines_data[line_index * 4u]));
                let y1 = i32(round(lines_data[line_index * 4u + 1u]));
                let x2 = i32(round(lines_data[line_index * 4u + 2u]));
                let y2 = i32(round(lines_data[line_index * 4u + 3u]));
                
                // Простой алгоритм рисования линии (Bresenham)
                let dx = abs(x2 - x1);
                let dy = abs(y2 - y1);
                let sx = select(-1, 1, x1 < x2);
                let sy = select(-1, 1, y1 < y2);
                var err = dx - dy;
                
                var x = x1;
                var y = y1;
                
                loop {
                    if (x >= 0 && y >= 0 && x < params.img_width && y < params.img_height) {
                        let pixel_index = u32(y * params.img_width + x);
                        if (pixel_index < arrayLength(&image_data)) {
                            image_data[pixel_index] = params.line_color;
                        }
                    }
                    
                    if (x == x2 && y == y2) {
                        break;
                    }
                    
                    let e2 = 2 * err;
                    if (e2 > -dy) {
                        err -= dy;
                        x += sx;
                    }
                    if (e2 < dx) {
                        err += dx;
                        y += sy;
                    }
                }
            }
        "#;

        let line_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Line Rendering Shader"),
            source: ShaderSource::Wgsl(shader_source.into()),
        });

        Ok(Self {
            device,
            queue,
            adapter,
            line_shader,
            buffer_pool: HashMap::new(),
        })
    }

    /// Определяет оптимальный метод рендеринга на основе объема данных
    pub fn determine_render_method(
        &self,
        images_data: &[(ImageBuffer<Rgba<u8>, Vec<u8>>, Vec<(f32, f32, f32, f32)>, [u8; 4])]
    ) -> RenderMethod {
        let num_images = images_data.len();
        let total_lines: usize = images_data.iter().map(|(_, lines, _)| lines.len()).sum();
        let total_pixels: u64 = images_data.iter().map(|(img, _, _)| (img.width() * img.height()) as u64).sum();
        
        // Вычисляем примерный размер данных
        let estimated_buffer_size = images_data.iter().map(|(img, lines, _)| {
            let img_size = (img.width() * img.height() * 4) as u64; // RGBA
            let lines_size = (lines.len() * 8 * 4) as u64; // 8 float32 на линию
            img_size + lines_size
        }).sum::<u64>();
        
        console::log_1(&format!(
            "Анализ данных: {} изображений, {} линий, {} пикселей, ~{}MB данных",
            num_images,
            total_lines,
            total_pixels,
            estimated_buffer_size / 1_000_000
        ).into());
        
        // Более строгие условия для использования GPU
        // GPU эффективен только при действительно больших объемах данных
        let meets_image_threshold = num_images >= GPU_THRESHOLD_IMAGES;
        let meets_lines_threshold = total_lines >= GPU_THRESHOLD_LINES;
        let meets_buffer_threshold = estimated_buffer_size >= GPU_THRESHOLD_BUFFER_SIZE;
        let meets_pixels_threshold = total_pixels >= GPU_THRESHOLD_PIXELS;
        
        console::log_1(&format!(
            "Пороги: изображения {}/{}, линии {}/{}, буфер {}MB/{}MB, пиксели {}/{}",
            num_images, GPU_THRESHOLD_IMAGES,
            total_lines, GPU_THRESHOLD_LINES,
            estimated_buffer_size / 1_000_000, GPU_THRESHOLD_BUFFER_SIZE / 1_000_000,
            total_pixels, GPU_THRESHOLD_PIXELS
        ).into());
        
        // Требуем выполнения ВСЕХ условий для использования GPU
        if meets_image_threshold && meets_lines_threshold && meets_buffer_threshold && meets_pixels_threshold {
            console::log_1(&"✅ Выбран GPU рендеринг (все пороги превышены)".into());
            RenderMethod::Gpu
        } else {
            console::log_1(&"❌ Выбран CPU рендеринг (не все пороги превышены)".into());
            RenderMethod::Cpu
        }
    }

    pub async fn render_lines_gpu(
        &mut self,
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        lines: &[(f32, f32, f32, f32)],
        color: [u8; 4],
    ) -> Result<(), String> {
        self.render_lines_gpu_single(img, lines, color).await
    }

    // Новый метод для батчевого рендеринга нескольких изображений
    pub async fn render_lines_gpu_batch(
        &mut self,
        images_data: &mut [(&mut ImageBuffer<Rgba<u8>, Vec<u8>>, &[(f32, f32, f32, f32)], [u8; 4])],
    ) -> Result<(), String> {
        // OPTIMIZATION: Детальные замеры времени для диагностики
        web_sys::console::time_with_label("GPU Batch Total");
        console::log_1(&format!("Starting TRUE GPU batch rendering for {} images", images_data.len()).into());
        
        if images_data.is_empty() {
            web_sys::console::time_end_with_label("GPU Batch Total");
            return Ok(());
        }
        
        // Получаем размеры первого изображения (предполагаем, что все одинакового размера)
        let (first_img, _, _) = &images_data[0];
        let img_width = first_img.width();
        let img_height = first_img.height();
        let single_img_size = (img_width * img_height * 4) as u64;
        let batch_count = images_data.len() as u64;
        let total_buffer_size = single_img_size * batch_count;
        
        // OPTIMIZATION: Замеряем время анализа размера буфера
        web_sys::console::time_with_label("Buffer Size Analysis");
        console::log_1(&format!("Batch processing {} images of {}x{}, total buffer size: {} bytes", 
            batch_count, img_width, img_height, total_buffer_size).into());
        
        // Проверяем, не превышает ли размер батча безопасный лимит
        const MAX_SAFE_BUFFER_SIZE: u64 = 1_073_741_824; // 1GB (оставляем запас от 2GB лимита)
        
        if total_buffer_size > MAX_SAFE_BUFFER_SIZE {
            // Разбиваем большой батч на меньшие части
            let max_images_per_batch = (MAX_SAFE_BUFFER_SIZE / single_img_size) as usize;
            console::log_1(&format!("Large batch detected. Splitting {} images into chunks of {} images each", 
                images_data.len(), max_images_per_batch).into());
            web_sys::console::time_end_with_label("Buffer Size Analysis");
            
            for chunk in images_data.chunks_mut(max_images_per_batch) {
                self.render_lines_gpu_batch_internal(chunk).await?;
            }
            web_sys::console::time_end_with_label("GPU Batch Total");
            return Ok(());
        }
        web_sys::console::time_end_with_label("Buffer Size Analysis");
        
        // Если размер батча приемлемый, обрабатываем как обычно
        let result = self.render_lines_gpu_batch_internal(images_data).await;
        web_sys::console::time_end_with_label("GPU Batch Total");
        result
    }
    
    // Внутренний метод для обработки батча без проверки размера
    async fn render_lines_gpu_batch_internal(
        &mut self,
        images_data: &mut [(&mut ImageBuffer<Rgba<u8>, Vec<u8>>, &[(f32, f32, f32, f32)], [u8; 4])],
    ) -> Result<(), String> {
        web_sys::console::time_with_label("GPU Batch Internal");
        
        let (first_img, _, _) = &images_data[0];
        let img_width = first_img.width();
        let img_height = first_img.height();
        let single_img_size = (img_width * img_height * 4) as u64;
        let batch_count = images_data.len() as u64;
        let total_buffer_size = single_img_size * batch_count;
        
        // Получаем буфер из пула или создаем новый
        let combined_buffer = self.get_or_create_buffer(
            total_buffer_size,
            BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            "Combined Batch Buffer"
        );
        
        // OPTIMIZATION: Замеряем время подготовки данных
        web_sys::console::time_with_label("Data Preparation");
        
        // Загружаем все изображения в один буфер
        let mut combined_data = Vec::with_capacity((total_buffer_size / 4) as usize);
        for (img, _, _) in images_data.iter() {
            // Конвертируем RGBA -> ABGR для каждого изображения
            for pixel in img.pixels() {
                let rgba = pixel.0;
                let abgr = ((rgba[3] as u32) << 24) | ((rgba[2] as u32) << 16) | ((rgba[1] as u32) << 8) | (rgba[0] as u32);
                combined_data.push(abgr);
            }
        }
        web_sys::console::time_end_with_label("Data Preparation");
        
        // OPTIMIZATION: Замеряем время записи в буфер
        web_sys::console::time_with_label("Buffer Write");
        
        // Записываем данные в буфер
        self.queue.write_buffer(&combined_buffer, 0, bytemuck::cast_slice(&combined_data));
        web_sys::console::time_end_with_label("Buffer Write");
        
        // Создаем буфер для всех линий
        let mut all_lines_data = Vec::new();
        let mut line_counts = Vec::new();
        
        for (_, lines, color) in images_data.iter() {
            line_counts.push(lines.len() as u32);
            for line in lines.iter() {
                all_lines_data.extend_from_slice(&[
                    line.0, line.1, line.2, line.3,
                    color[0] as f32, color[1] as f32, color[2] as f32, color[3] as f32
                ]);
            }
        }
        
        if !all_lines_data.is_empty() {
            let lines_buffer_size = (all_lines_data.len() * std::mem::size_of::<f32>()) as u64;
            let lines_buffer = self.get_or_create_buffer(
                lines_buffer_size,
                BufferUsages::STORAGE,
                "Batch Lines Buffer"
            );
            
            self.queue.write_buffer(&lines_buffer, 0, bytemuck::cast_slice(&all_lines_data));
            
            // Создаем bind group для батчевой обработки
            let bind_group_layout = self.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Batch Bind Group Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
            
            let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
                label: Some("Batch Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: combined_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: lines_buffer.as_entire_binding(),
                    },
                ],
            });
            
            // OPTIMIZATION: Замеряем время выполнения GPU вычислений
            web_sys::console::time_with_label("GPU Compute");
            
            // Выполняем compute shader для всех изображений одновременно
            let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Batch Compute Encoder"),
            });
            
            {
                let pipeline_layout = self.device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some("Batch Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });
                
                let compute_pipeline = self.device.create_compute_pipeline(&ComputePipelineDescriptor {
                    label: Some("Batch Line Rendering Pipeline"),
                    layout: Some(&pipeline_layout),
                    module: &self.line_shader,
                    entry_point: Some("main"),
                    cache: None,
                    compilation_options: Default::default(),
                });
                
                let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                    label: Some("Batch Compute Pass"),
                    timestamp_writes: None,
                });
                
                compute_pass.set_pipeline(&compute_pipeline);
                compute_pass.set_bind_group(0, &bind_group, &[]);
                
                // Вычисляем workgroups для всех линий сразу
                let total_lines: usize = line_counts.iter().map(|&count| count as usize).sum();
                let workgroup_count = (total_lines + 63) / 64; // 64 threads per workgroup
                
                console::log_1(&format!("Dispatching {} workgroups for {} total lines across {} images", 
                    workgroup_count, total_lines, batch_count).into());
                compute_pass.dispatch_workgroups(workgroup_count as u32, 1, 1);
            }
            
            // Получаем выходной буфер из пула
             let output_buffer = self.get_or_create_buffer(
                 total_buffer_size,
                 BufferUsages::COPY_DST | BufferUsages::MAP_READ,
                 "Batch Output Buffer"
             );
            
            encoder.copy_buffer_to_buffer(&combined_buffer, 0, &output_buffer, 0, total_buffer_size);
            self.queue.submit(std::iter::once(encoder.finish()));
            web_sys::console::time_end_with_label("GPU Compute");
            
            // OPTIMIZATION: Замеряем время чтения результатов
            web_sys::console::time_with_label("GPU Read Results");
            console::log_1(&"Reading batch results from GPU".into());
            
            // Читаем результаты обратно
            let buffer_slice = output_buffer.slice(..);
            let (sender, receiver) = futures_channel::oneshot::channel();
            buffer_slice.map_async(MapMode::Read, move |result| {
                sender.send(result).unwrap();
            });
            
            let result = receiver.await;
            
            match result {
                Ok(Ok(())) => {
                    let data = buffer_slice.get_mapped_range();
                    let pixel_data: &[u32] = bytemuck::cast_slice(&data);
                    
                    // Распределяем результаты обратно по изображениям
                    let pixels_per_image = (img_width * img_height) as usize;
                    for (i, (img, _, _)) in images_data.iter_mut().enumerate() {
                        let start_idx = i * pixels_per_image;
                        let end_idx = start_idx + pixels_per_image;
                        let img_pixel_data = &pixel_data[start_idx..end_idx];
                        
                        let img_data = img.as_mut();
                        let rgba_chunks = img_data.chunks_exact_mut(4);
                        
                        for (rgba_chunk, &pixel) in rgba_chunks.zip(img_pixel_data.iter()) {
                            // Конвертируем обратно из ABGR в RGBA
                            rgba_chunk[0] = (pixel & 0xFF) as u8;         // R
                            rgba_chunk[1] = ((pixel >> 8) & 0xFF) as u8;  // G
                            rgba_chunk[2] = ((pixel >> 16) & 0xFF) as u8; // B
                            rgba_chunk[3] = ((pixel >> 24) & 0xFF) as u8; // A
                        }
                    }
                    
                    drop(data);
                    output_buffer.unmap();
                    web_sys::console::time_end_with_label("GPU Read Results");
                    console::log_1(&"Batch GPU processing completed successfully".into());
                },
                Ok(Err(e)) => {
                    return Err(format!("Batch buffer mapping failed: {:?}", e));
                },
                Err(_) => {
                    return Err("Batch channel closed unexpectedly".into());
                }
            }
            
            // TODO: Возвращаем буферы в пул для переиспользования
            // Временно отключено из-за проблем с lifetime
            // self.return_buffer_to_pool(combined_buffer, total_buffer_size);
            // self.return_buffer_to_pool(output_buffer, total_buffer_size);
            // if !all_lines_data.is_empty() {
            // self.return_buffer_to_pool(lines_buffer, lines_buffer_size);
            // }
        }
        
        web_sys::console::time_end_with_label("GPU Batch Internal");
        Ok(())
    }

    async fn render_lines_gpu_single(
        &mut self,
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        lines: &[(f32, f32, f32, f32)],
        color: [u8; 4],
    ) -> Result<(), String> {
        console::log_1(&format!("GPU rendering {} lines on {}x{} image", lines.len(), img.width(), img.height()).into());
        
        // Создаем буфер для данных изображения (как u32 для RGBA пикселей)
        let img_width = img.width() as usize;
        let img_height = img.height() as usize;
        let pixel_count = img_width * img_height;
        let buffer_size = (pixel_count * std::mem::size_of::<u32>()) as u64;
        
        console::log_1(&format!("Creating image buffer: {}x{} = {} pixels", img_width, img_height, pixel_count).into());
        
        // Создаем буфер для изображения
        let buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Image Buffer"),
            size: buffer_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Конвертируем RGBA u8 данные в u32 пиксели
        let img_data = img.as_raw();
        let mut pixel_data: Vec<u32> = Vec::with_capacity(pixel_count);
        for chunk in img_data.chunks_exact(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3] as u32;
            let pixel = (a << 24) | (b << 16) | (g << 8) | r; // ABGR format
            pixel_data.push(pixel);
        }
        
        console::log_1(&"Image data converted to u32".into());
        
        // Записываем данные изображения в буфер
        self.queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&pixel_data));

        // Создаем буфер для линий
        let lines_data: Vec<f32> = lines.iter()
            .flat_map(|(x1, y1, x2, y2)| vec![*x1, *y1, *x2, *y2])
            .collect();
        
        let lines_buffer_size = (lines_data.len() * std::mem::size_of::<f32>()) as u64;
        let lines_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Lines Buffer"),
            size: lines_buffer_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.queue.write_buffer(&lines_buffer, 0, bytemuck::cast_slice(&lines_data));

        let img_width = img.width() as i32;
        let img_height = img.height() as i32;
        
        // Конвертируем цвет в u32 формат (ABGR)
        let color_u32 = ((color[3] as u32) << 24) | ((color[2] as u32) << 16) | ((color[1] as u32) << 8) | (color[0] as u32);
        
        // Создаем uniform буфер для параметров
        let params_data = [img_width, img_height, color_u32 as i32, 0i32]; // padding для выравнивания
        let params_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Params Buffer"),
            size: (params_data.len() * std::mem::size_of::<i32>()) as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        self.queue.write_buffer(&params_buffer, 0, bytemuck::cast_slice(&params_data));

        // Создаем bind group layout
        let bind_group_layout = self.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Compute Bind Group Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Создаем bind group
        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Compute Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: lines_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        // Создаем compute pipeline
        let pipeline_layout = self.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = self.device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &self.line_shader,
            entry_point: Some("main"),
            cache: None,
            compilation_options: Default::default(),
        });

        // Выполняем compute shader
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Compute Encoder"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            
            let workgroup_count = (lines.len() as u32 + 63) / 64; // Округляем вверх
            console::log_1(&format!("Dispatching {} workgroups for {} lines", workgroup_count, lines.len()).into());
            compute_pass.dispatch_workgroups(workgroup_count, 1, 1);
            console::log_1(&"Compute dispatch completed".into());
        }

        // ОПТИМИЗАЦИЯ: Создаем буфер для чтения результата с минимальным размером
        let output_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Output Buffer"),
            size: buffer_size,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        console::log_1(&"Copying buffer and submitting commands".into());
        encoder.copy_buffer_to_buffer(&buffer, 0, &output_buffer, 0, buffer_size);
        
        // ОПТИМИЗАЦИЯ: Отправляем команды без ожидания
        self.queue.submit(std::iter::once(encoder.finish()));
        console::log_1(&"Commands submitted to GPU queue".into());

        console::log_1(&"Reading result back from GPU".into());
        // Читаем результат обратно - ОПТИМИЗИРОВАННАЯ ВЕРСИЯ
        let buffer_slice = output_buffer.slice(..);
        let (sender, receiver) = futures_channel::oneshot::channel();
        buffer_slice.map_async(MapMode::Read, move |result| {
            sender.send(result).unwrap();
        });
        
        // Убираем синхронное ожидание - используем только асинхронное
        let result = receiver.await;
        
        match result {
            Ok(Ok(())) => {
                let data = buffer_slice.get_mapped_range();
                let pixel_data: &[u32] = bytemuck::cast_slice(&data);
                
                // ОПТИМИЗИРОВАННАЯ конвертация: используем chunks_exact для векторизации
                let img_data = img.as_mut();
                let rgba_chunks = img_data.chunks_exact_mut(4);
                
                for (rgba_chunk, &pixel) in rgba_chunks.zip(pixel_data.iter()) {
                    // Быстрая конвертация ABGR -> RGBA
                    rgba_chunk[0] = ((pixel >> 16) & 0xFF) as u8; // R
                    rgba_chunk[1] = ((pixel >> 8) & 0xFF) as u8;  // G  
                    rgba_chunk[2] = (pixel & 0xFF) as u8;         // B
                    rgba_chunk[3] = ((pixel >> 24) & 0xFF) as u8; // A
                }
                
                drop(data);
                output_buffer.unmap();
                console::log_1(&"GPU data read completed".into());
            },
            Ok(Err(e)) => {
                return Err(format!("Buffer mapping failed: {:?}", e).into());
            },
            Err(_) => {
                return Err("Channel closed unexpectedly".into());
            }
        }

        Ok(())
    }

    pub fn is_available(&self) -> bool {
        true
    }

    // Методы управления пулом буферов
    fn get_or_create_buffer(&mut self, size: u64, usage: BufferUsages, label: &str) -> Buffer {
        // Проверяем, не превышает ли размер буфера максимальный лимит
        const MAX_BUFFER_SIZE: u64 = 1_073_741_824; // 1GB - безопасный лимит (запас от 2GB)
        
        if size > MAX_BUFFER_SIZE {
            console::log_1(&format!("Warning: Requested buffer size ({} bytes) exceeds safe limit ({} bytes). Consider splitting the batch.", size, MAX_BUFFER_SIZE).into());
        }
        
        if let Some(buffers) = self.buffer_pool.get_mut(&size) {
            if let Some(buffer) = buffers.pop() {
                console::log_1(&format!("Reusing buffer from pool: {} bytes", size).into());
                return buffer;
            }
        }
        
        console::log_1(&format!("Creating new buffer: {} bytes", size).into());
        self.device.create_buffer(&BufferDescriptor {
            label: Some(label),
            size,
            usage,
            mapped_at_creation: false,
        })
    }
    
    fn return_buffer_to_pool(&mut self, buffer: Buffer, size: u64) {
        let buffers = self.buffer_pool.entry(size).or_insert_with(Vec::new);
        if buffers.len() < 10 { // Ограничиваем размер пула
            buffers.push(buffer);
            console::log_1(&format!("Buffer returned to pool: {} bytes", size).into());
        }
    }
}

// Глобальный экземпляр GPU рендерера
static mut GPU_RENDERER: Option<GpuRenderer> = None;

pub async fn init_gpu_renderer() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if GPU_RENDERER.is_none() {
            console::log_1(&"Attempting to initialize GPU renderer...".into());
            match GpuRenderer::new().await {
                Ok(renderer) => {
                    GPU_RENDERER = Some(renderer);
                    console::log_1(&"GPU renderer initialized successfully".into());
                },
                Err(e) => {
                    console::log_1(&format!("Failed to initialize GPU renderer: {}", e).into());
                    return Err(e);
                }
            }
        } else {
            console::log_1(&"GPU renderer already initialized".into());
        }
    }
    Ok(())
}

pub fn get_gpu_renderer() -> Option<&'static mut GpuRenderer> {
    unsafe { GPU_RENDERER.as_mut() }
}

pub fn get_gpu_renderer_ref() -> Option<&'static GpuRenderer> {
    unsafe { GPU_RENDERER.as_ref() }
}

pub fn is_gpu_available() -> bool {
    unsafe { GPU_RENDERER.is_some() }
}