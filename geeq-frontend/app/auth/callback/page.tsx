"use client"

import { useRouter, useSearchParams } from "next/navigation";
import { NextPage } from "next/types"
import { useEffect, useRef, useState } from "react";
import { geeqApiClient } from "../../../components/client/GeeqApiClient";

export const Page: NextPage = () => {
    const searchParams = useSearchParams();
    const code = searchParams.get("code");
    const router = useRouter();
    const isRequested = useRef(false);

    useEffect(() => {
        if (!isRequested.current) {
            isRequested.current = true;
            geeqApiClient.authOauthPost({ code: code }, { withCredentials: true })
                .then(res => {
                    router.replace('/auth/me');
                });
        }
    }, [geeqApiClient, code, router]);
    return <div>callback中...</div>
}
export default Page