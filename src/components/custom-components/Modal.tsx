import { ReactNode, useEffect } from "react";
import { createPortal } from "react-dom";

interface ModalProps {
	isOpen: boolean;
	onClose: () => void;
	children: ReactNode;
	closeOnOutsideClick?: boolean;
	width?: number,
	button?: React.ReactNode
}

const Modal = ({
	isOpen,
	onClose,
	children,
	closeOnOutsideClick = true,
	width = 600,
	button
}: ModalProps) => {
	// Обработка закрытия по ESC
	useEffect(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if (e.key === "Escape") onClose();
		};

		if (isOpen) window.addEventListener("keydown", handleKeyDown);
		return () => window.removeEventListener("keydown", handleKeyDown);
	}, [isOpen, onClose]);

	// Создаем портал только при isOpen = true
	if (!isOpen) return null;

	return createPortal(
		<div
			role="dialog"
			aria-modal="true"
			className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
			onClick={closeOnOutsideClick ? onClose : undefined}
		>
			<div
				className="relative max-h-[90vh] w-full overflow-y-auto rounded-lg bg-white p-6 shadow-xl transform-gpu will-change-transform flex flex-col"
				style={{ width: `${width}px` }}
				onClick={(e) => e.stopPropagation()}
			>

				{/* Кнопка закрытия */}
				<button
					onClick={onClose}
					className="absolute right-4 top-4 text-gray-500 hover:text-gray-700"
					aria-label="Close modal"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						className="h-6 w-6"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							strokeLinecap="round"
							strokeLinejoin="round"
							strokeWidth={2}
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>

				{/* Контент модалки */}
				<div className="flex-grow overflow-y-auto pr-4">{children}</div>
				{button && <div>{button}</div>}
			</div>
		</div>,
		document.body
	);
};

export default Modal;