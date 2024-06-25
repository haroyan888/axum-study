import { useState, useEffect } from "react";
import AddIcon from "@mui/icons-material/Add";
import { Button } from "@mui/material";

import "./App.css";
import TodoCard from "./components/Todo/Todo";
import Todo from "./types/Todo";
import AddFormModal from "./components/Modal/add-form-modal";

async function getTodos() {
  return await fetch("http://localhost:8000/api/todo")
    .then((res) => res.json())
    .then((json) => {
      console.log(json);
      return json;
    })
    .catch(() => {
      alert("データの取得に失敗しました");
      return undefined;
    });
}

function App() {
  // Todoの取得に関する処理
  const [todos, setTodos] = useState([]);
  useEffect(() => {
    (async () => {
      setTodos(await getTodos());
    })();
  }, []);

  // Todoの追加用モーダルに関する処理
  const [isShowAddModal, setShowAddModal] = useState<boolean>(false);

  return (
    <>
      {/* Todoの表示部 */}
      {todos !== undefined
        ? todos.map((element: Todo) => {
            return (
              <div key={element["id"]} className="m-10">
                <TodoCard
                  id={element["id"]}
                  title={element["title"]}
                  description={element["description"]}
                  completed={element["completed"]}
                />
              </div>
            );
          })
        : "Todoはありません"}

      {/* Todoを追加するモーダル */}
      <div className="fixed right-10 bottom-10">
        <Button
          variant="contained"
          onClick={() => setShowAddModal(true)}
          endIcon={<AddIcon />}
        >
          Add
        </Button>
      </div>

      <AddFormModal
        isShowModal={isShowAddModal}
        setShowModal={setShowAddModal}
        afterAddTodo={() => {
          (async () => setTodos(await getTodos()))();
        }}
      />
    </>
  );
}

export default App;
