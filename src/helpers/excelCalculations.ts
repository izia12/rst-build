import { ExcelView } from '../types/data.types';

/**
 * Вычисляет минимальное отклонение для заданной функции
 * @param functionName - название функции
 * @param floorData - данные этажа
 * @returns минимальное отклонение или null если данных нет
 */
export const getMinDeviationForFunction = (functionName: string, floorData: ExcelView): number | null => {
	const armature = floorData.values.find(a => a.function_name === functionName);
	if (!armature || armature.combinations.length === 0) return null;
	
	return Math.min(...armature.combinations.map(c => Math.abs(c.deviation)));
};

/**
 * Проверяет, должен ли checkbox быть отмечен по умолчанию (минимальное отклонение)
 * @param functionName - название функции
 * @param deviation - отклонение комбинации
 * @param floorData - данные этажа
 * @returns true если это минимальное отклонение
 */
export const shouldBeCheckedByDefault = (functionName: string, deviation: number, floorData: ExcelView): boolean => {
	const minDeviation = getMinDeviationForFunction(functionName, floorData);
	return minDeviation !== null && Math.abs(deviation) === minDeviation;
};

/**
 * Генерирует уникальный ключ для комбинации
 * @param level - уровень этажа
 * @param functionName - название функции
 * @param combinationIndex - индекс комбинации
 * @returns уникальный ключ
 */
export const generateCombinationKey = (level: string, functionName: string, combinationIndex: number): string => {
	return `${level}-${functionName}-${combinationIndex}`;
};

/**
 * Инициализирует набор выбранных комбинаций по умолчанию (с минимальными отклонениями)
 * @param excelData - массив данных Excel
 * @returns Set с ключами выбранных по умолчанию комбинаций
 */
export const initializeDefaultSelectedCombinations = (excelData: ExcelView[]): Set<string> => {
	const defaultSelected = new Set<string>();
	
	excelData.forEach((floorData) => {
		floorData.values.forEach((armature) => {
			armature.combinations.forEach((combination, index) => {
				if (shouldBeCheckedByDefault(armature.function_name, combination.deviation, floorData)) {
					const key = generateCombinationKey(floorData.level, armature.function_name, index);
					defaultSelected.add(key);
				}
			});
		});
	});
	
	return defaultSelected;
};