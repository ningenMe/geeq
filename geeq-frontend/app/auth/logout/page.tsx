"use client";

import { useRouter } from "next/navigation";
import { Suspense, useEffect } from "react";
import { geeqApiClient } from "../../../components/client/GeeqApiClient";

const Logout = () => {
  const router = useRouter();

  useEffect(() => {
    geeqApiClient.authLogoutPost({ withCredentials: true }).then(() => {
      router.replace("/");
    });
  }, [geeqApiClient, router]);
  return <></>;
};

export default function Page() {
  return (
    <Suspense fallback={<div>logout中...</div>}>
      <Logout />
    </Suspense>
  );
}
