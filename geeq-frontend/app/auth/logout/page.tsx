"use client";
import { useRouter } from "next/navigation";
import { geeqApiClient } from "../../../components/client/GeeqApiClient";

export default function Page() {
  const router = useRouter();
  geeqApiClient
    .authLogoutPost({ withCredentials: true })
    .catch(() => {})
    .finally(() => {
      router.push("/");
    });

  return <></>;
}
