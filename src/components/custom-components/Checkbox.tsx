import { useState } from "react";
import { useAppDispatch, useAppSelector } from "../../store/store";
import { addChosedItems, deleteChoosedItem } from "../../store/slices/slice.wasm";

interface CheckboxProps extends React.InputHTMLAttributes<HTMLInputElement> {
	label?: string;
	id: string,
}
export const Checkbox = ({ label, ...props }: CheckboxProps) => {
	const choosedPlains = useAppSelector(state => state.wasm.choosedPlainsFromList);
	const dispatch = useAppDispatch();
	const [checked, setChecked] = useState(false);
	return (
		<label className="flex items-center space-x-3 cursor-pointer group">
			<div className="relative">
				<input
					type="checkbox"
					{...props}
					className="absolute opacity-0 w-0 h-0"
				/>
				<div
					className={`
			  w-5 h-5 border rounded-md flex items-center justify-center
			  transition-all duration-200
			  ${checked
					? "bg-blue-500 border-blue-500"
					: "bg-white border-gray-300 group-hover:border-blue-400"
				}
			  ${props.disabled ? "opacity-50 cursor-not-allowed" : ""}
			`}
				onClick={() => {
					setChecked(!checked)
					console.log(choosedPlains, checked);
					if (!checked) {
						dispatch(addChosedItems(+props.id))
					} else {
						dispatch(deleteChoosedItem(+props.id))
					}
				}}
				>
					<svg
						className={`w-3.5 h-3.5 text-white transition-opacity ${checked ? "opacity-100" : "opacity-0"
							}`}
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