import { ReactElement } from "react";


type propsType = {
	onClick?:()=>void
	classes?:string,
	buttonName?:string,
	disabled?:boolean
	
}
export const Button = (props:propsType):ReactElement=>{
	return (
		<button 
		disabled={props.disabled || false}
		onClick={props.onClick}
			// onClick={() => setIsOpen(true)}
			className={`rounded bg-blue-500 px-4 py-2 text-white hover:bg-blue-600 ${props.classes} ${props.disabled?"disabled:opacity-75":""}`}
		>
			{props.buttonName ||`Выбрать унификацию`}
		</button>
	)
}