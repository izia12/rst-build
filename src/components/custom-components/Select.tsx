interface SelectProps extends React.SelectHTMLAttributes<HTMLSelectElement> {
	label?: string;
	error?: string;
	options: { value: string; label: string }[];
  }
  
  export const Select = ({ label, error, options, ...props }: SelectProps) => {
	return (
	  <div className="w-full">
		{label && (
		  <label className="block mb-2 text-sm font-medium text-gray-700">
			{label}
		  </label>
		)}
		<div className="relative">
		  <select
			{...props}
			className={`
			  w-full px-4 py-2 pr-8 border rounded-lg appearance-none
			  focus:outline-none focus:ring-2 transition-all
			  ${error 
				? "border-red-500 focus:ring-red-200" 
				: "border-gray-300 focus:border-blue-500 focus:ring-blue-200"}
			  disabled:bg-gray-100 disabled:cursor-not-allowed
			  ${props.className || ""}
			`}
		  >
			{options.map((option) => (
			  <option key={option.value} value={option.value}>
				{option.label}
			  </option>
			))}
		  </select>
		  <div className="absolute inset-y-0 right-3 flex items-center pointer-events-none">
			<svg
			  className="w-5 h-5 text-gray-400"
			  fill="none"
			  stroke="currentColor"
			  viewBox="0 0 24 24"
			>
			  <path
				strokeLinecap="round"
				strokeLinejoin="round"
				strokeWidth={2}
				d="M19 9l-7 7-7-7"
			  />
			</svg>
		  </div>
		</div>
		{error && <p className="mt-1 text-sm text-red-600">{error}</p>}
	  </div>
	);
  };