"use client";

import { geeqApiClient } from "../components/client/GeeqApiClient";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { GITHUB_CLIENT_ID } from "../components/constant";

export default function Page() {
  // TODO local/本番で使い分ける
  const redirect_uri = process.env.NEXT_PUBLIC_FRONT_ORIGIN + "/auth/callback";
  const github_oauth_url = `https://github.com/login/oauth/authorize?client_id=${GITHUB_CLIENT_ID}&redirect_uri=${redirect_uri}`;
  const [loginUserId, setLoginUserId] = useState<string | null>(null);
  const router = useRouter();

  useEffect(() => {
    geeqApiClient.authMeGet({ withCredentials: true }).then((res) => {
      setLoginUserId(res.data.userId);
    });
  }, [geeqApiClient]);
  return (
    <>
      <LoginButton />
      <LogoutButton />
      <h1>Hello, Geeq</h1>
      <h1>redirect_uri = {redirect_uri}</h1>
      <a>{loginUserId ? loginUserId + " Logined" : "You Are Guest User"}</a>
    </>
  );
}
