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
  const options = task.options.map((option, index) => {
    return (
      <div key={index}>
        <h5>{"  " + index + " :  " + option}</h5>
      </div>
    );
  });
  return (
    <>
      <Card style={{ margin: "1rem" }}></Card>
      <Card style={{ margin: "1rem" }}>
        <CardContent>
          <h1>{task.title}</h1>
          <ReactMarkdown>{task.description}</ReactMarkdown>
          <p>writer: {task.createdBy}</p>
          <p>createdAt: {task.createdAt}</p>
        </CardContent>
      </Card>
      <Card style={{ margin: "1rem" }}>
        <CardContent>
          <div>{options}</div>
        </CardContent>
      </Card>
    </>
  );
}
