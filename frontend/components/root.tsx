"use client";

import { useEffect, useState } from "react";
import { TaskQuery } from "./generated";
import { geeqApiClient } from "./client/GeeqApiClient";
import { Card, CardActionArea, CardContent, Typography } from "@mui/material";

export const TaskList = () => {
  const [taskList, setTaskList] = useState<TaskQuery[]>([]);

  useEffect(() => {
    geeqApiClient
      .taskGet({
        withCredentials: true,
      })
      .then((res) => {
        setTaskList(res.data.tasks);
      })
      .catch(() => {
        setTaskList([]);
      });
  }, []);

  const cardList = taskList.map((task) => {
    return (
      <Card key={task.taskId} style={{ margin: "1rem" }}>
        <CardActionArea href={"/task/" + task.taskId}>
          <CardContent>
            <h2>{task.title}</h2>
            <div>writer: {task.createdBy}</div>
            <div>createdAt: {task.createdAt}</div>
          </CardContent>
        </CardActionArea>
      </Card>
    );
  });
  return <>{cardList}</>;
};
