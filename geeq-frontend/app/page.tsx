"use client"

import { NextPage } from "next";
import { geeqApiClient } from "../components/client/GeeqApiClient";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";

export default function Page() {
  // TODO 環境変数に入れる
  const github_client_id = "Iv1.17c5ebb1ad2d2832";
  // TODO local/本番で使い分ける
  const redirect_url = "http://localhost:3000/auth/callback"
  const github_oauth_url = `https://github.com/login/oauth/authorize?client_id=${github_client_id}&redirect_url=${redirect_url}`;
  const [loginUserId, setLoginUserId] = useState<string | null>(null);
  const router = useRouter();

  useEffect(() => {
    geeqApiClient.authMeGet({ withCredentials: true })
      .then(res => {
        setLoginUserId(res.data.userId);
      });
  }, [geeqApiClient]);
  return (
    <>
      <button
        onClick={() => {
          router.replace(github_oauth_url)
        }}
      >
        LOGIN
      </button>
      <button onClick={() => {
        geeqApiClient.authLogoutPost({ withCredentials: true})
        setLoginUserId(null);
      }}>
        LOGOUT
      </button>
      <h1>Hello, Geeq</h1>
      <a>{loginUserId ? loginUserId+" Logined" : "You Are Guest User"}</a>
    </>
  )
}