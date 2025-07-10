import { ReactNode, useEffect } from "react";
import { createPortal } from "react-dom";

interface ModalProps {
	isOpen: boolean;
	onClose: () => void;
	children: ReactNode;
	closeOnOutsideClick?: boolean;
	width?: number;
	button?: React.ReactNode;
	title?: string;
}

const Modal = ({
	isOpen,
	onClose,
	children,
	closeOnOutsideClick = true,
	width = 600,
	button,
	title
}: ModalProps) => {
	useEffect(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if (e.key === "Escape") onClose();
		};

		if (isOpen) {
			window.addEventListener("keydown", handleKeyDown);
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = 'unset';
		}
		
		return () => {
			window.removeEventListener("keydown", handleKeyDown);
			document.body.style.overflow = 'unset';
		};
	}, [isOpen, onClose]);

	if (!isOpen) return null;

	return createPortal(
		<div
			role="dialog"
			aria-modal="true"
			className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm animate-fade-in"
			onClick={closeOnOutsideClick ? onClose : undefined}
		>
			<div
				className="relative max-h-[90vh] w-full rounded-xl bg-white shadow-strong animate-slide-up flex flex-col"
				style={{ width: `${Math.min(width, window.innerWidth - 32)}px` }}
				onClick={(e) => e.stopPropagation()}
			>
				{/* Header */}
				<div className="flex items-center justify-between p-6 border-b border-secondary-200 flex-shrink-0">
					{title && (
						<h2 className="text-xl font-semibold text-secondary-900">{title}</h2>
					)}
					<button
						onClick={onClose}
						className="p-2 text-secondary-400 hover:text-secondary-600 hover:bg-secondary-100 rounded-lg transition-colors"
						aria-label="Закрыть модальное окно"
					>
						<svg className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
						</svg>
					</button>
				</div>

				{/* Content */}
				<div className="flex-1 overflow-y-auto p-6 min-h-0">
					{children}
				</div>

				{/* Footer */}
				{button && (
					<div className="border-t border-secondary-200 p-6 flex-shrink-0">
						{button}
					</div>
				)}
			</div>
		</div>,
		document.body
	);
};

export default Modal;