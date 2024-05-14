"use client";

import { useRouter, useSearchParams } from "next/navigation";
import { Suspense, useEffect, useRef } from "react";
import { geeqApiClient } from "../../../components/client/GeeqApiClient";

const AuthCallBack = () => {
  const searchParams = useSearchParams();
  const code = searchParams.get("code");
  const router = useRouter();
  const isRequested = useRef(false);

  useEffect(() => {
    if (!isRequested.current) {
      isRequested.current = true;
      geeqApiClient
        .authLoginPost({ code: code }, { withCredentials: true })
        .then(() => {
          router.replace("/");
        });
    }
  }, [geeqApiClient, code, router]);
  return <></>;
};

export default function Page() {
  return (
    <Suspense fallback={<div>callback中...</div>}>
      <AuthCallBack />
    </Suspense>
  );
}
