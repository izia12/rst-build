// toast.tsx
import React, { useState, useEffect, useCallback, createContext, useContext } from 'react';

// Типы для Toast
type ToastType = 'success' | 'error' | 'warning' | 'info';
type ToastPosition = 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';

interface Toast {
	id: string;
	message: string;
	type: ToastType;
	duration?: number;
}

interface ToastContextType {
	showToast: (message: string, type?: ToastType, duration?: number) => void;
}

const ToastContext = createContext<ToastContextType | null>(null);

// Провайдер для Toast
export const ToastProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
	const [toasts, setToasts] = useState<Toast[]>([]);

	const showToast = useCallback((
		message: string,
		type: ToastType = 'info',
		duration: number = 3000
	) => {
		const id = Date.now().toString();
		setToasts((prev) => [...prev, { id, message, type, duration }]);
	}, []);

	const removeToast = useCallback((id: string) => {
		setToasts((prev) => prev.filter(toast => toast.id !== id));
	}, []);

	return (
		<ToastContext.Provider value={{ showToast }}>
			{children}
			<ToastContainer toasts={toasts} onRemove={removeToast} />
		</ToastContext.Provider>
	);
};

// Хук для использования Toast
export const useToast = () => {
	const context = useContext(ToastContext);
	if (!context) {
		throw new Error('useToast must be used within a ToastProvider');
	}
	return context;
};

// Контейнер для Toast
const ToastContainer: React.FC<{
	toasts: Toast[];
	onRemove: (id: string) => void;
	position?: ToastPosition;
}> = ({ toasts, onRemove, position = 'top-right' }) => {
	// Позиционирование
	const positionClasses = {
		'top-right': 'top-4 right-4',
		'top-left': 'top-4 left-4',
		'bottom-right': 'bottom-4 right-4',
		'bottom-left': 'bottom-4 left-4',
	}[position];

	return (
		<div className={`fixed z-50 ${positionClasses}`}>
			{toasts.map((toast) => (
				<ToastItem key={toast.id} toast={toast} onRemove={onRemove} />
			))}
		</div>
	);
};

// Компонент отдельного Toast
const ToastItem: React.FC<{
	toast: Toast;
	onRemove: (id: string) => void;
}> = ({ toast, onRemove }) => {
	// Автоматическое закрытие
	useEffect(() => {
		if (toast.duration && toast.duration > 0) {
			const timer = setTimeout(() => {
				onRemove(toast.id);
			}, toast.duration);

			return () => clearTimeout(timer);
		}
	}, [toast, onRemove]);

	// Иконки для разных типов
	const getIcon = () => {
		switch (toast.type) {
			case 'success': return '✅';
			case 'error': return '❌';
			case 'warning': return '⚠️';
			case 'info': return 'ℹ️';
			default: return null;
		}
	};

	// Цвета для разных типов
	const getColorClasses = () => {
		switch (toast.type) {
			case 'success': return 'bg-green-100 border-green-500 text-green-700';
			case 'error': return 'bg-red-100 border-red-500 text-red-700';
			case 'warning': return 'bg-yellow-100 border-yellow-500 text-yellow-700';
			case 'info': return 'bg-blue-100 border-blue-500 text-blue-700';
			default: return 'bg-gray-100 border-gray-500 text-gray-700';
		}
	};

	return (
		<div className={`
      mb-2 p-4 rounded-lg border-l-4 shadow-lg transition-all
      animate-fadeIn duration-300
      ${getColorClasses()}
    `}>
			<div className="flex items-center">
				<span className="mr-2 text-lg">{getIcon()}</span>
				<span className="flex-grow">{toast.message}</span>
				<button
					onClick={() => onRemove(toast.id)}
					className="ml-4 text-gray-500 hover:text-gray-700"
				>
					✕
				</button>
			</div>
		</div>
	);
};

// Добавляем в глобальные стили анимацию
const style = document.createElement('style');
style.textContent = `
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}
.animate-fadeIn {
  animation: fadeIn 0.3s ease-out forwards;
}
`;
document.head.appendChild(style);