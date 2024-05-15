import { LoginButton, LogoutButton, Me } from "../components/root";

export default function Page() {
  return (
    <>
      <LoginButton />
      <LogoutButton />
      <h1>Hello, Geeq</h1>
      <Me />
    </>
  );
}
