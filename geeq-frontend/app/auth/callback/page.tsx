"use client"

import { useRouter, useSearchParams } from "next/navigation";
import { NextPage } from "next/types"
import { useEffect, useState } from "react";
import { geeqApiClient } from "../../../components/client/GeeqApiClient";

export const Page: NextPage = () => {
    const searchParams = useSearchParams();
    const code = searchParams.get("code");
    const router = useRouter()

    useEffect(
        () => {
            geeqApiClient.authOauthPost({code: code})
            .then(res => {
                router.replace('/auth/me')
            })
        },
        [geeqApiClient]
    );
    return <div>callback中...</div>
}
export default Page