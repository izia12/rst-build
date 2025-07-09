import { ReactElement, useState } from "react";
import { ExcelViewTable } from "./ExcelViewTable";
import { Button } from "../custom-components/Button";
import Modal from "../custom-components/Modal";
import { fetchExcelViewData } from "../../store/slices/thunks/wasmThanks";
import { useAppDispatch, useAppSelector } from "../../store/store";

export const ExcelView = (): ReactElement => {
	const [isOpenModal, setIsOpenModal] = useState<boolean>(false);
	const dispatch = useAppDispatch();
	const groupUniqueItems = useAppSelector(state => state.wasm.groupUniqueItems);
	const armDiameters = useAppSelector(state => state.wasm.armDiameters);

	const handleOpenModal = async () => {
		// Проверяем, есть ли данные для загрузки
		if (groupUniqueItems.length > 0 && armDiameters.length > 0) {
			// Подготавливаем данные для WASM
			const diameters = new Uint32Array(armDiameters.map(d => d.diameter));
			const floorsJson = JSON.stringify(groupUniqueItems.map(item => ({
				title: item.name,
				level: `Level_${item.id}`,
				max_as1: item.maxAsValues[0]?.as1 || 0,
				max_as2: item.maxAsValues[0]?.as2 || 0,
				max_as3: item.maxAsValues[0]?.as3 || 0,
				max_as4: item.maxAsValues[0]?.as4 || 0,
				steps: [item.steps.mainStep || 200, item.steps.secondaryStep || 200],
				color: item.color
			})));

			// Загружаем данные
			await dispatch(fetchExcelViewData({ diameters, floorsJson }));
		}
		
		setIsOpenModal(true);
	};

	return (
		<>
			<Button 
				onClick={handleOpenModal}
				buttonName="Просмотр Excel"
			/>
			<Modal
			width={1600}
				isOpen={isOpenModal}
				onClose={() => setIsOpenModal(false)}
			>
				<div className="max-w-6xl">
					<h2 className="text-lg font-semibold mb-4">Данные для Excel отчета</h2>
					<ExcelViewTable />
				</div>
			</Modal>
		</>
	);
};