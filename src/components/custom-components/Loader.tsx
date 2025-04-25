import { ReactElement } from "react";

export const Loader =():ReactElement=>{
return(
	<div className="absolute inset-0 bg-white bg-opacity-80 flex items-center justify-center z-50">
	<div className="flex flex-col items-center">
		<div className="animate-spin h-12 w-12 border-4 border-blue-500 rounded-full border-t-transparent"></div>
		<span className="mt-3 text-gray-600">Загрузка данных...</span>
	</div>
	</div>
)
}