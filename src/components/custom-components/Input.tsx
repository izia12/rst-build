interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
	label?: string;
	error?: string;
}

export const Input = ({ label, error, ...props }: InputProps) => {
	return (
		<div className="w-full">
			{label && (
				<label className="block mb-2 text-sm font-medium text-gray-700">
					{label}
				</label>
			)}
			<input
				{...props}
				className={`
			w-full px-4 py-2 border rounded-lg transition-all
			focus:outline-none focus:ring-2 
			${error
						? "border-red-500 focus:ring-red-200"
						: "border-gray-300 focus:border-blue-500 focus:ring-blue-200"}
			disabled:bg-gray-100 disabled:cursor-not-allowed
			${props.className || ""}
		  `}
			/>
			{error && (
				<p className="mt-1 text-sm text-red-600">{error}</p>
			)}
		</div>
	);
};