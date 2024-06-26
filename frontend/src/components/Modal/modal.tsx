import ClearIcon from '@mui/icons-material/Clear';
import { IconButton } from "@mui/material";

import "./modal.css"

type ModalProps = {
	title: string,
	isShowModal: boolean,
	closeModal: (flag: boolean) => void
	children: React.ReactNode
}

export default function Modal ({
	title,
	isShowModal,
	closeModal,
	children
} : ModalProps) {
	return (
		<>	
		{isShowModal ? (
			<>
				<div className="overlay" onClick={() => closeModal(false)} />
				<div className="modal-container">
					<div className="modal-content">
						<div className="modal-content_header flex items-center justify-between mb-3">
							<h1>{ title }</h1>
							<IconButton 
								onClick={() => closeModal(false)}
								className="right-0"
							>
								<ClearIcon />
							</IconButton>
						</div>
						<div className="modal-content_body m-3">
							{ children }
						</div>
					</div>
				</div>
			</>
			): null}
		</>
	)
}