import { ReactElement, useState } from "react";
import { ExcelViewTable } from "./ExcelViewTable";
import { Button } from "../custom-components/Button";
import Modal from "../custom-components/Modal";
import { fetchExcelViewData } from "../../store/slices/thunks/wasmThanks";
import { useAppSelector } from "../../store/store";

export const ExcelView = ():ReactElement=>{
	const [isOpenModal, setIsOpenModal] = useState<boolean>(false)
	const excelData = useAppSelector(state=>state.wasm.excelViewData)
	console.log(excelData);
	
	return(
		<>
			<Button 
				onClick={()=>{
					setIsOpenModal(true)
				
				}}
				buttonName="просмотр Excel"
			/>
			<Modal
				isOpen={isOpenModal}
				onClose={()=>setIsOpenModal(!isOpenModal)}
			>
				<ExcelViewTable/>
			</Modal>
		</>
	)
}