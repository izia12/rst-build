import { ReactElement } from "react";

type LoaderProps = {
	size?: 'sm' | 'md' | 'lg';
	text?: string;
	fullScreen?: boolean;
}

export const Loader = ({
	size = 'md',
	text = 'Загрузка данных...',
	fullScreen = true
}: LoaderProps): ReactElement => {
	const sizeClasses = {
		sm: 'h-6 w-6',
		md: 'h-12 w-12',
		lg: 'h-16 w-16'
	};

	const spinner = (
		<div className="flex flex-col items-center space-y-4">
			<div className="relative">
				<div className={`${sizeClasses[size]} border-4 border-primary-200 rounded-full animate-spin`}></div>
				<div className={`${sizeClasses[size]} border-4 border-primary-600 rounded-full animate-spin absolute top-0 left-0`} style={{ borderRightColor: 'transparent', borderTopColor: 'transparent' }}></div>
			</div>
			{text && (
				<div className="text-center">
					<p className="text-secondary-600 font-medium">{text}</p>
					<div className="flex space-x-1 mt-2 justify-center">
						<div className="w-2 h-2 bg-primary-500 rounded-full animate-pulse"></div>
						<div className="w-2 h-2 bg-primary-500 rounded-full animate-pulse" style={{ animationDelay: '0.1s' }}></div>
						<div className="w-2 h-2 bg-primary-500 rounded-full animate-pulse" style={{ animationDelay: '0.2s' }}></div>
					</div>
				</div>
			)}
		</div>
	);

	if (!fullScreen) {
		return spinner;
	}

	return (
		<div className="fixed inset-0 bg-white/90 flex items-center justify-center z-[9999]">
			{spinner}
		</div>
	);
};