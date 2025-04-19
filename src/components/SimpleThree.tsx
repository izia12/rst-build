import React, { useRef, useEffect, useCallback } from 'react';
import * as THREE from 'three';

const InteractiveCubes = () => {
  const containerRef = useRef<HTMLDivElement>(null);
  const sceneRef = useRef<THREE.Scene>(new THREE.Scene());
  const cameraRef = useRef<THREE.PerspectiveCamera>();
  const rendererRef = useRef<THREE.WebGLRenderer>();
  const cubesRef = useRef<THREE.Mesh[]>([]);
  const raycasterRef = useRef<THREE.Raycaster>(new THREE.Raycaster());
  const mouseRef = useRef<THREE.Vector2>(new THREE.Vector2());

  // Создаем 10 случайных кубов
  const createCubes = useCallback(() => {
    const cubes = [];
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    
    for(let i = 0; i < 10; i++) {
      const material = new THREE.MeshBasicMaterial({
        color: new THREE.Color(Math.random(), Math.random(), Math.random()),
        name: `cube-${i}`
      });
      
      const cube = new THREE.Mesh(geometry, material);
      cube.position.set(
        Math.random() * 10 - 5,
        Math.random() * 10 - 5,
        Math.random() * 10 - 5
      );
      cube.userData.id = i; // Добавляем кастомные данные
      cubes.push(cube);
    }
    return cubes;
  }, []);

  // Инициализация Three.js
  useEffect(() => {
    if (!containerRef.current) return;

    // Настройка рендерера
    rendererRef.current = new THREE.WebGLRenderer({ antialias: true });
    rendererRef.current.setSize(800, 600);
    containerRef.current.appendChild(rendererRef.current.domElement);

    // Настройка камеры
    cameraRef.current = new THREE.PerspectiveCamera(
      75,
      800 / 600,
      0.1,
      1000
    );
    cameraRef.current.position.z = 15;

    // Добавляем кубы на сцену
    cubesRef.current = createCubes();
    cubesRef.current.forEach(cube => sceneRef.current.add(cube));

    // Анимация
    const animate = () => {
      requestAnimationFrame(animate);
      // Вращение кубов
      cubesRef.current.forEach(cube => {
        cube.rotation.x += 0.01;
        cube.rotation.y += 0.01;
      });

      rendererRef.current?.render(sceneRef.current, cameraRef.current!);
    };
    animate();

    // Очистка
    return () => {
      if (rendererRef.current) {
        containerRef.current?.removeChild(rendererRef.current.domElement);
        rendererRef.current.dispose();
      }
    };
  }, [createCubes]);

  // Обработчик клика
  const handleClick = useCallback((event: MouseEvent) => {
    if (!rendererRef.current || !cameraRef.current) return;

    // 1. Нормализация координат мыши
    const rect = rendererRef.current.domElement.getBoundingClientRect();
    mouseRef.current.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
    mouseRef.current.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

    // 2. Обновление Raycaster
    raycasterRef.current.setFromCamera(mouseRef.current, cameraRef.current);

    // 3. Поиск пересечений
    const intersects = raycasterRef.current.intersectObjects(cubesRef.current);

    // 4. Обработка пересечений
    if (intersects.length > 0) {
      const clickedCube = intersects[0].object as THREE.Mesh;
      const material = clickedCube.material as THREE.MeshBasicMaterial;
      
      // Меняем цвет и выводим информацию
      material.color.setHex(Math.random() * 0xffffff);
      console.log('Clicked cube ID:', clickedCube.userData.id);
    }
  }, []);

  // Добавляем обработчик событий
  useEffect(() => {
    if (!rendererRef.current) return;
    
    const canvas = rendererRef.current.domElement;
    canvas.addEventListener('click', handleClick);
    
    return () => canvas.removeEventListener('click', handleClick);
  }, [handleClick]);

  return <div ref={containerRef} />;
};

export default InteractiveCubes;