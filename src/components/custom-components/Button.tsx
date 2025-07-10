import { ReactElement, ReactNode } from "react";

type ButtonVariant = 'primary' | 'secondary' | 'success' | 'warning' | 'error';
type ButtonSize = 'sm' | 'md' | 'lg';

type ButtonProps = {
	onClick?: () => void | Promise<void>;
	variant?: ButtonVariant;
	size?: ButtonSize;
	disabled?: boolean;
	loading?: boolean;
	children?: ReactNode;
	buttonName?: string;
	icon?: ReactNode;
	className?: string;
	classes?: string; // Добавляем для обратной совместимости
}

export const Button = ({
	onClick,
	variant = 'primary',
	size = 'md',
	disabled = false,
	loading = false,
	children,
	buttonName,
	icon,
	className = '',
	classes = '' // Поддержка старого API
}: ButtonProps): ReactElement => {
	const baseClasses = 'inline-flex items-center justify-center font-medium rounded-lg transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed';
	
	const variantClasses = {
		primary: 'bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500 shadow-sm hover:shadow-md',
		secondary: 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 focus:ring-blue-500 shadow-sm hover:shadow-md',
		success: 'bg-green-600 text-white hover:bg-green-700 focus:ring-green-500 shadow-sm hover:shadow-md',
		warning: 'bg-yellow-600 text-white hover:bg-yellow-700 focus:ring-yellow-500 shadow-sm hover:shadow-md',
		error: 'bg-red-600 text-white hover:bg-red-700 focus:ring-red-500 shadow-sm hover:shadow-md'
	};
	
	const sizeClasses = {
		sm: 'px-3 py-1.5 text-sm',
		md: 'px-4 py-2 text-sm',
		lg: 'px-6 py-3 text-base'
	};
	
	// Объединяем className и classes для обратной совместимости
	const customClasses = `${className} ${classes}`.trim();
	
	return (
		<button
			disabled={disabled || loading}
			onClick={onClick}
			className={`${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${customClasses}`}
		>
			{loading && (
				<svg className="animate-spin -ml-1 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
					<circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
					<path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
				</svg>
			)}
			{icon && !loading && <span className="mr-2">{icon}</span>}
			{children || buttonName || 'Кнопка'}
		</button>
	);
};