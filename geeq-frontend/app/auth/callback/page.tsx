"use client"

import { useSearchParams } from "next/navigation";
import { NextPage } from "next/types"

export const Page: NextPage = () => {
    const searchParams = useSearchParams();
    const code = searchParams.get("code");
    return <div>{code}</div>
}
export default Page