import { ReactElement } from "react";
import { Checkbox } from "./custom-components/Checkbox";

export const UniqeItems = ():ReactElement=>{
	
	
	return(
		<div>
			<div className="flex">
				<Checkbox/>
				<div>Z Name</div>
				<div>Number of Plate Elements</div>
				<div>Number of rod elements</div>
			</div>
		</div>
	)
}