import { Card, CardContent } from "@mui/material";
import { geeqApiClient } from "../../../components/client/GeeqApiClient";
import React from "react";
import ReactMarkdown from "react-markdown";

export default async function Page({ params }: { params: { taskId: string } }) {
  const task = await geeqApiClient
    .taskTaskIdGet(params.taskId, {})
    .then((res) => {
      return res.data.task;
    });
  //TODO catchの時の処理を追加

  return (
    <>
      <Card style={{ margin: "1rem" }}></Card>
      <Card style={{ margin: "1rem" }}>
        <CardContent>
          <h1>{task.title}</h1>
          <ReactMarkdown>{task.description}</ReactMarkdown>
          <div>writer: {task.createdBy}</div>
          <div>createdAt: {task.createdAt}</div>
        </CardContent>
      </Card>
    </>
  );
}
