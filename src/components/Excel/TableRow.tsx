import { ReactElement } from "react";
import { useDispatch } from "react-redux";
import { PreparedExcelView } from "../../types/data.types";
import { toggleCombinationChecked } from "../../store/slices/slice.wasm";

interface TableRowProps {
	floorData: PreparedExcelView;
	floorIndex: number;
}

export const TableRow = ({ floorData, floorIndex }: TableRowProps): ReactElement => {
	const dispatch = useDispatch();

	// Подсчитываем общее количество строк для этого этажа
	const totalRows = floorData.values.reduce((sum, armature) => sum + armature.combinations.length, 0);

	// Обработчик клика по checkbox'у
	const handleCheckboxChange = (armatureIndex: number, combinationIndex: number) => {
		dispatch(toggleCombinationChecked({
			floorIndex,
			armatureIndex,
			combinationIndex
		}));
	};
	let currentRowIndex = 0;

	return (
		<>
			{floorData.values.map((armature, armatureIndex) => (
				armature.combinations.map((combination, combinationIndex) => {
					const isFirstRowOfFloor = currentRowIndex === 0;
					const isFirstRowOfFunction = combinationIndex === 0;
					const key = `${floorData.level}-${armature.function_name}-${combinationIndex}`;
					const isChecked = combination.is_default_checked;
					const isMinDeviation = combination.is_min_deviation;
					currentRowIndex++;

					return (
						<tr
							key={key}
							className={`${floorIndex % 2 === 0 ? "bg-white" : "bg-gray-50"
								} ${isChecked ? 'ring-2 ring-green-300 bg-green-50' : ''} hover:bg-blue-50 transition-colors`}
						>
							{/* Checkbox для выбора */}
							<td className="px-4 py-2 text-center">
								<input
									type="checkbox"
									checked={isChecked}
									onChange={() => handleCheckboxChange(armatureIndex, combinationIndex)}
									className={`w-4 h-4 rounded border-gray-300 cursor-pointer ${isChecked
											? 'text-green-600 focus:ring-green-500'
											: 'text-gray-400 hover:text-gray-600'
										}`}
									title={isMinDeviation ? 'Оптимальный вариант (минимальное отклонение)' : 'Кликните для выбора'}
								/>
							</td>

							{/* Этаж - показываем только в первой строке этажа */}
							{isFirstRowOfFloor && (
								<td
									rowSpan={totalRows}
									className="px-4 py-2 text-sm text-gray-900 border-r border-gray-200 align-top font-medium"
								>
									{floorData.level}
								</td>
							)}
							{/* Название - показываем только в первой строке этажа */}
							{isFirstRowOfFloor && (
								<td
									rowSpan={totalRows}
									className="px-4 py-2 text-sm text-gray-900 border-r border-gray-200 align-top"
								>
									{floorData.title || '-'}
								</td>
							)}

							{/* Функция - показываем только в первой строке функции */}
							{isFirstRowOfFunction && (
								<td
									rowSpan={armature.combinations.length}
									className="px-4 py-2 text-sm text-gray-900 border-r border-gray-200 align-top font-medium"
								>
									{armature.function_name.toUpperCase()}
								</td>
							)}

							{/* Целевая площадь - показываем только в первой строке функции */}
							{isFirstRowOfFunction && (
								<td
									rowSpan={armature.combinations.length}
									className="px-4 py-2 text-sm text-gray-900 border-r border-gray-200 align-top"
								>
									{armature.as_target_value.toFixed(2)}
								</td>
							)}

							{/* Основной диаметр */}
							<td className="px-4 py-2 text-sm text-gray-900">
								⌀{combination.main_diameter}
							</td>

							{/* Дополнительный диаметр */}
							<td className="px-4 py-2 text-sm text-gray-900">
								{combination.additional_diameter > 0 ? `⌀${combination.additional_diameter}` : '-'}
							</td>

							{/* Общая площадь */}
							<td className="px-4 py-2 text-sm text-gray-900">
								{combination.total_area.toFixed(2)}
							</td>

							{/* Отклонение */}
							<td className={`px-4 py-2 text-sm font-medium ${isChecked
									? 'text-green-700 bg-green-100'
									: Math.abs(combination.deviation) <= 5
										? 'text-green-600'
										: Math.abs(combination.deviation) <= 10
											? 'text-yellow-600'
											: 'text-red-600'
								}`}>
								{combination.deviation > 0 ? '+' : ''}{combination.deviation.toFixed(1)}%
								{isMinDeviation && (
									<span className="ml-2 text-xs bg-blue-200 text-blue-800 px-1 rounded">
										МИН
									</span>
								)}
								{isChecked && (
									<span className="ml-2 text-xs bg-green-200 text-green-800 px-1 rounded">
										✓
									</span>
								)}
							</td>
						</tr>
					);
				})
			))}
		</>
	);
};