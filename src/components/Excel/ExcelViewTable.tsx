import { ReactElement } from "react";
import { useAppSelector } from "../../store/store";
import { TableRow } from "./TableRow";

export const ExcelViewTable = (): ReactElement => {
	const excelData = useAppSelector(state => state.wasm.excelViewData);
	const loading = useAppSelector(state => state.wasm.loading);

	if (loading) {
		return <div className="p-4">Загрузка данных...</div>;
	}

	if (!excelData || excelData.length === 0) {
		return <div className="p-4">Нет данных для отображения</div>;
	}

	return (
		<div className="space-y-4">
			{/* Инструкция для пользователя */}
			<div className="bg-blue-50 border border-blue-200 rounded-lg p-3">
				<div className="flex items-center space-x-2">
					<span className="text-blue-600">ℹ️</span>
					<span className="text-sm text-blue-800">
						<strong>Инструкция:</strong> По умолчанию выбраны варианты с минимальным отклонением. 
						Вы можете изменить выбор, кликая на checkbox'ы.
					</span>
				</div>
			</div>

			{/* Таблица */}
			<div className="overflow-x-auto max-h-96 overflow-y-auto border rounded-lg">
				<table className="min-w-full divide-y divide-gray-200 bg-white">
					<thead className="bg-gray-50 sticky top-0">
						<tr>
							<th className="px-4 py-2 text-center text-sm font-medium text-gray-500">
								<span className="text-green-600">✓</span> Выбор
							</th>
							<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Этаж</th>
							<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Название</th>
							<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Функция</th>
							<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Целевая площадь</th>
							<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Основной ⌀</th>
							<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Доп. ⌀</th>
							<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Общая площадь</th>
							<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Отклонение (%)</th>
						</tr>
					</thead>
					<tbody className="bg-white divide-y divide-gray-200">
						{excelData.map((floor, floorIndex) => (
							<TableRow 
								key={`${floor.level}-${floorIndex}`} 
								floorData={floor} 
								floorIndex={floorIndex}
							/>
						))}
					</tbody>
				</table>
			</div>
		</div>
	);
};