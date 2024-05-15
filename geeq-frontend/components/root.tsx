"use client";

import { useRouter } from "next/navigation";
import { GITHUB_CLIENT_ID } from "../components/constant";
import { geeqApiClient } from "./client/GeeqApiClient";
import { useState, useEffect } from "react";

const redirect_uri = process.env.NEXT_PUBLIC_FRONT_ORIGIN + "/auth/callback";
const github_oauth_url = `https://github.com/login/oauth/authorize?client_id=${GITHUB_CLIENT_ID}&redirect_uri=${redirect_uri}`;

export const LoginButton = () => {
  const router = useRouter();

  return (
    <button
      onClick={() => {
        router.push(github_oauth_url);
      }}
    >
      LOGIN
    </button>
  );
};

export const LogoutButton = () => {
  return (
    <button
      onClick={() => {
        geeqApiClient.authLogoutPost({ withCredentials: true });
      }}
    >
      LOGOUT
    </button>
  );
};

export const Me = () => {
  const [loginUserId, setLoginUserId] = useState<string | null>(null);

  useEffect(() => {
    geeqApiClient.authMeGet({ withCredentials: true }).then((res) => {
      setLoginUserId(res.data.userId);
    });
  }, [geeqApiClient]);

  return <a>{loginUserId ? loginUserId + " Logined" : "You Are Guest User"}</a>;
};
