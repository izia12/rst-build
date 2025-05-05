
import React, { useCallback, useEffect, useRef } from 'react'
import * as THREE from 'three'
//@ts-expect-error this library is exists but ts shows an error
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls"
import { useAppDispatch, useAppSelector } from '../store/store.ts'
import { transformLinesPointsIntoArray } from '../helpers/transformLinesPointsIntoArray.ts'
import { WASMDataType } from '../types/data.types.ts'
import { endPerfomance } from '../store/slices/slice.wasm.ts'

const ThreeScene = () => {
	const dispatch = useAppDispatch()
	const data = useAppSelector(state => state.wasm.wasmData)

	const containerRef = useRef<HTMLDivElement>(null)
	const sceneRef = useRef<THREE.Scene | null>(null)
	const cameraRef = useRef<THREE.PerspectiveCamera | null>(null)
	const rendererRef = useRef<THREE.WebGLRenderer | null>(null)
	const controlsRef = useRef<OrbitControls | null>(null)
	const animationFrameId = useRef<number>()

	// Инициализация Three.js объектов
	useEffect(() => {
		if (!containerRef.current) return

		// Создание сцены
		sceneRef.current = new THREE.Scene()

		// Создание камеры
		const aspect = containerRef.current.clientWidth / containerRef.current.clientHeight
		cameraRef.current = new THREE.PerspectiveCamera(75, aspect, 0.1, 10000)
		cameraRef.current.position.set(200, 200, 200)

		// Создание рендерера
		rendererRef.current = new THREE.WebGLRenderer({ antialias: true })
		rendererRef.current.setSize(
			containerRef.current.clientWidth,
			containerRef.current.clientHeight
		)
		containerRef.current.appendChild(rendererRef.current.domElement)

		// Настройка контролов
		if (cameraRef.current && rendererRef.current) {
			controlsRef.current = new OrbitControls(
				cameraRef.current,
				rendererRef.current.domElement
			)
			controlsRef.current.enableDamping = true
			controlsRef.current.dampingFactor = 0.05
		}

		return () => {
			// Очистка при размонтировании
			if (rendererRef.current) {
				containerRef.current?.removeChild(rendererRef.current.domElement)
				rendererRef.current.dispose()
			}
			controlsRef.current?.dispose()
			if (animationFrameId.current) {
				cancelAnimationFrame(animationFrameId.current)
			}
		}
	}, [])

	// Обработка изменения размера окна
	const handleResize = useCallback(() => {
		if (containerRef.current && cameraRef.current && rendererRef.current) {
			const width = containerRef.current.clientWidth
			const height = containerRef.current.clientHeight

			cameraRef.current.aspect = width / height
			cameraRef.current.updateProjectionMatrix()
			rendererRef.current.setSize(width, height)
		}
	}, [])

	useEffect(() => {
		window.addEventListener('resize', handleResize)
		return () => window.removeEventListener('resize', handleResize)
	}, [handleResize])

	// Анимация
	useEffect(() => {
		const animate = () => {
			if (!sceneRef.current || !cameraRef.current || !rendererRef.current) return

			controlsRef.current?.update()
			rendererRef.current.render(sceneRef.current, cameraRef.current)
			animationFrameId.current = requestAnimationFrame(animate)
		}
		animate()

		return () => {
			if (animationFrameId.current) {
				cancelAnimationFrame(animationFrameId.current)
			}
		}
	}, [])

	// Обработка данных
	const processData = useCallback((data: WASMDataType[]) => {
		if (!sceneRef.current) return
		// Очистка предыдущих объектов
		sceneRef.current.clear()
		const geometryGroups = {
			LINES: [] as number[],
			'3DFACE_TRIANGLES': [] as number[],
			'3DFACES': [] as number[],
		}

		data.forEach(item => {
			const vertices = item.vertices.flatMap(v => [v.x, v.y, v.z])
			if (item.vertices.length === 2) {
				geometryGroups.LINES.push(...vertices)
			} else if (item.vertices.length === 3) {
				geometryGroups['3DFACE_TRIANGLES'].push(...vertices)
			} else if (item.vertices.length === 4) {
				geometryGroups['3DFACES'].push(...vertices)
			}
		})

		Object.entries(geometryGroups).forEach(([type, points]) => {
			if (points.length === 0) return

			const geometryType = type === '3DFACES' ? '3DFACES' :
				type === 'LINES' ? 'LINES' : 'TRIANGLE_FACES'
			const lines = transformLinesPointsIntoArray(
				geometryType,
				points,
				'#ffffff'
			)

			if (lines) sceneRef.current?.add(lines)
		})

		// Центрирование камеры
		if (sceneRef.current.children.length > 0 && cameraRef.current) {
			const group = new THREE.Group()
			sceneRef.current.children.forEach(child => group.add(child.clone()))
			centerCameraOnObject(group)
		}

		// Замер производительности
		const perfEnd = performance.now()
		dispatch(endPerfomance(perfEnd))
	}, [dispatch])

	useEffect(() => {
		if (data.length > 0) {
			processData(data)
		}
	}, [data, processData])

	const centerCameraOnObject = (object: THREE.Object3D) => {
		if (!cameraRef.current || !controlsRef.current) return

		const box = new THREE.Box3().setFromObject(object)
		const center = box.getCenter(new THREE.Vector3())
		const size = box.getSize(new THREE.Vector3())

		controlsRef.current.target.copy(center)

		const maxDim = Math.max(size.x, size.y, size.z)
		const fov = cameraRef.current.fov * (Math.PI / 180)
		const cameraZ = Math.abs(maxDim / 2 / Math.tan(fov / 2))

		cameraRef.current.position.set(
			center.x,
			center.y + maxDim / 2,
			center.z + cameraZ
		)
		cameraRef.current.updateProjectionMatrix()
		controlsRef.current.update()
	}
	return (
		<div>
			<div
				ref={containerRef}
				style={{ minWidth: '100%', height: '50vh' }}
				className='mb-4'
			/>
			<div id="dxf-content-container">
				<pre id="dxf-content" />
			</div>
		</div>
	)
}

export default ThreeScene