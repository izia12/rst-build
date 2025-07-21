import { useState } from "react";
import { useAppDispatch } from "../../store/store";
import { addChosedItems, deleteChoosedItem } from "../../store/slices/slice.wasm";
import { useToast } from "./Toast";

interface CheckboxProps extends React.InputHTMLAttributes<HTMLInputElement> {
	label?: string;
	id: string,
	maxValues:{
		as1:number,
		as2:number,
		as3:number,
		as4:number,
		asw1:number,
		asw2:number,
	},
	currentStep:{mainStep:number | null, secondaryStep:number|null}
}
export const Checkbox = ({ label, ...props }: CheckboxProps) => {
	const {showToast} = useToast()
	const dispatch = useAppDispatch();
	const [checked, setChecked] = useState(false);
	const isDisabled = (props.currentStep.mainStep===null || props.currentStep.secondaryStep===null) 
	|| (props.maxValues.as1 ===0 && props.maxValues.as2 ===0 && props.maxValues.as3 ===0 && props.maxValues.as4 ===0)

	return (
		<label className="flex items-center space-x-3 cursor-pointer group">
			<div className="relative">
				<input
					disabled={isDisabled}
					type="checkbox"
					// {...props}
					className="absolute opacity-0 w-0 h-0"
				/>
				<div
					className={`
			  w-5 h-5 border rounded-md flex items-center justify-center
			  transition-all duration-200
			  ${checked&&!isDisabled
					? "bg-blue-500 border-blue-500"
					: "bg-white border-gray-300 group-hover:border-blue-400"
				}
			  ${isDisabled ? "opacity-50 cursor-not-allowed" : ""}
			`}
				onClick={() => {
					setChecked(!checked)
					if(props.currentStep.mainStep===null || props.currentStep.secondaryStep===null){
						showToast("Задайте шаги!", "error")
					}
					if (!checked) {
						dispatch(addChosedItems({
							plainNumber:+props.id,
							asValues:{as1:props.maxValues.as1, as2:props.maxValues.as2, as3:props.maxValues.as3, as4:props.maxValues.as4, asw1:props.maxValues.asw1, asw2:props.maxValues.asw2},
							steps:props.currentStep
						}))
					} else {
						dispatch(deleteChoosedItem({
							asValues:{as1:props.maxValues.as1, as2:props.maxValues.as2, as3:props.maxValues.as3, as4:props.maxValues.as4, asw1:props.maxValues.asw1, asw2:props.maxValues.asw2},
							plainNumber:+props.id,
							steps:props.currentStep
						}))
					}
				}}
				>
					<svg
						className={`w-3.5 h-3.5 text-white transition-opacity ${checked && !isDisabled ? "opacity-100" : "opacity-0"}`}
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							strokeLinecap="round"
							strokeLinejoin="round"
							strokeWidth={2}
							d="M5 13l4 4L19 7"
						/>
					</svg>
				</div>
			</div>
			{label && (
				<span className="text-gray-700 text-sm select-none">{label}</span>
			)}
		</label>
	);
};