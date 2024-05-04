"use client"

import { NextPage } from "next";
import { geeqApiClient } from "../components/client/GeeqApiClient";
import { useEffect, useState } from "react";

export const Page: NextPage = () => {
  // TODO 環境変数に入れる
  const github_client_id = "Iv1.17c5ebb1ad2d2832";
  // TODO local/本番で使い分ける
  const redirect_url = "http://localhost:3000/auth/callback"
  const github_oauth_url = `https://github.com/login/oauth/authorize?client_id=${github_client_id}&redirect_url=${redirect_url}`;
  const [loginUserId, setLoginUserId] = useState<string | null>(null);

  useEffect(() => {
    geeqApiClient.authMeGet({ withCredentials: true })
      .then(res => {
        setLoginUserId(res.data.userId);
      });
  }, [geeqApiClient]);

  return (
    <>
      <a
        className='App-link'
        href={github_oauth_url}
      >
        LOGIN with Github
      </a>
      <a>
        {loginUserId} logined
      </a>
      <h1>Hello, Next.js!</h1>
    </>
  )
}

export default Page;