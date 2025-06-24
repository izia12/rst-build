import React from 'react'


type PropsType={
	onClick?:()=>void,
	title?:string
}
export default function LinkButton({title, onClick}:PropsType) {
  return (
	<button onClick={onClick} className='cursor-pointer'>
		{title|| "ok"}
	</button>
  )
}
