import { Container } from "@mui/material";
import { TaskList } from "../components/root";

export default function Page({ params }: { params: { task: string } }) {
  return (
    <Container>
      <TaskList />
    </Container>
  );
}
