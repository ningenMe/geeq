"use client"

import { useRouter, useSearchParams } from "next/navigation";
import { NextPage } from "next/types"
import { useEffect, useRef } from "react";
import { geeqApiClient } from "../../../components/client/GeeqApiClient";

export const Page: NextPage = () => {
    const searchParams = useSearchParams();
    const code = searchParams.get("code");
    const router = useRouter();
    const isRequested = useRef(false);

    useEffect(() => {
        if (!isRequested.current) {
            isRequested.current = true;
            geeqApiClient.authLoginPost({ code: code }, { withCredentials: true })
                .then(() => {
                    router.replace('/');
                });
        }
    }, [geeqApiClient, code, router]);
    return <div>callback中...</div>
}
export default Page