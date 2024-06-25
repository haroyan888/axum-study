import { useState, useEffect } from "react";

import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";

import Todo from "@/types/Todo";

export default function TodoCard({ id, title, description, completed }: Todo) {
  const [completedLoc, setCompleted] = useState<boolean>(completed);
  useEffect(() => {
    fetch("http://localhost:8000/api/todo/search/" + id, {
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
    <Card
      sx={{ minWidth: 275 }}
      style={{
        backgroundColor: completedLoc ? "#43a047" : "#eeeeee",
      }}
    >
      <CardContent>
        <Typography variant="h5" component="div">
          {title}
        </Typography>
        <Typography variant="body2">{description}</Typography>
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
      </CardActions>
    </Card>
  );
}
