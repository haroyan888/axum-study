import SendIcon from "@mui/icons-material/Send";
import { Button, TextField } from "@mui/material";

import { postFetch } from "../../functions/fetch";
import Modal from "./modal";
import { FormEventHandler } from "react";

export default function AddFormModal({
  isShowModal,
  setShowModal,
  afterAddTodo,
}: {
  isShowModal: boolean;
  setShowModal: (flag: boolean) => void;
  afterAddTodo: () => void;
}) {
  const addFormEventHandler: FormEventHandler<HTMLFormElement> = async (
    event
  ) => {
    event.preventDefault();
    const form = new FormData(event.currentTarget);
    const title = form.get("title");
    const description = form.get("description");

    // データ送信
    const res = await postFetch(
      "/todo",
      JSON.stringify({
        title: title,
        description: description,
      })
    );

    if (!res.ok) {
      alert("追加に失敗しました");
    }

    setShowModal(false);
    afterAddTodo();
  };

  return (
    <>
      <Modal
        title="Todoの追加"
        isShowModal={isShowModal}
        closeModal={setShowModal}
      >
        <form onSubmit={addFormEventHandler}>
          <TextField
            required
            id="add-title"
            name="title"
            label="タイトル"
            variant="standard"
            margin="normal"
          />
          <TextField
            id="add-description"
            name="description"
            label="説明"
            multiline
            margin="normal"
          />
          <Button type="submit" endIcon={<SendIcon />}>
            Send
          </Button>
        </form>
      </Modal>
    </>
  );
}
