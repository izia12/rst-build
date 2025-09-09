import React from 'react'


type PropsType={
	onClick?:()=>void,
	title?:string
}
export default function LinkButton({title, onClick}:PropsType): JSX.Element {
  return (
	<button onClick={onClick} className='cursor-pointer'>
		{title|| "ok"}
	</button>
  )
}
