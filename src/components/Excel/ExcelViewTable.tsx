import { ReactElement } from "react";

export const ExcelViewTable=():ReactElement=>{
	
	return(
		<>
		<table className=" divide-y divide-gray-200 bg-white " style={{ maxWidth: "550px" }}>
				<thead className="bg-gray-50">
					<tr>
						<th></th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Функция</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Целевая площадь</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Основной шаг</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Доп. шаг</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Основная арматура</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Доп. арматура</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Общая площадь</th>
						<th className="px-4 py-2 text-left text-sm font-medium text-gray-500">Отклонение (%)</th>
					</tr>
				</thead>
				<tbody>
					
				</tbody>
			</table>
		</>
	)
}