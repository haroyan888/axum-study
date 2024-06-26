import { useState, useEffect } from "react";

import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";

import DetailTodoModal from "@/components/Modal/detail-modal"
import Todo from "@/types/Todo";

export default function TodoCard({
	todo,
	afterEditTodo
} : {
	todo:Todo,
	afterEditTodo: () => void
}) {
  // 完了したかどうかをローカルの状態で保存
  const [completedLoc, setCompleted] = useState<boolean>(todo.completed);
	// Todoの詳細モーダルの開閉状態を表す
	const [isShowDetailTodo, setShowDetailTodo] = useState<boolean>(false);
  useEffect(() => {
    console.log("Completed useEffect");
    fetch("http://localhost:8000/api/todo/search/" + todo.id, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        completed: completedLoc,
      }),
    });
  }, [completedLoc]);
  return (
    <>
      <Card
        sx={{ minWidth: 275 }}
        style={{
          backgroundColor: completedLoc ? "#43a047" : "#eeeeee",
        }}
      >
        <CardContent>
          <Typography variant="h5" component="div">
            {todo.title}
          </Typography>
          <Typography variant="body2">{todo.description}</Typography>
        </CardContent>
        <CardActions>
          <Button
            size="small"
            style={{
              color: completedLoc ? "#ef5350" : "#66bb6a",
            }}
            onClick={() => setCompleted(!completedLoc)}
          >
            {completedLoc ? "まだ" : "できた"}
          </Button>
          <Button variant="contained" onClick={() => setShowDetailTodo(true)}>
            詳細
          </Button>
        </CardActions>
      </Card>
      <DetailTodoModal 
        todo={todo}
        isShowModal={isShowDetailTodo}
        setShowModal={setShowDetailTodo}
        afterEditTodo={afterEditTodo}
      />
    </>
  );
}
