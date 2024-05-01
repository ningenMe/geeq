"use client"

import { useRouter, useSearchParams } from "next/navigation";
import { NextPage } from "next/types"
import { useEffect, useState } from "react";
import { geeqApiClient } from "../../../components/client/GeeqApiClient";

export const Page: NextPage = () => {
    const searchParams = useSearchParams();
    const code = searchParams.get("code");
    const router = useRouter()

    // TODO 2回リクエストしてるのを直す
    useEffect(
        () => {
            geeqApiClient.authOauthPost({code: code}, {withCredentials: true})
            .then(res => {
                router.replace('/auth/me')
            })
        },
        [geeqApiClient]
    );
    return <div>callback中...</div>
}
export default Page