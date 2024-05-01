import { NextPage } from "next";

export const Page: NextPage = () => {
  // TODO 環境変数に入れる
  const github_client_id = "Iv1.17c5ebb1ad2d2832";
  const github_oauth_url = `https://github.com/login/oauth/authorize?client_id=${github_client_id}&scope=user:read`;

  return (
    <>
      <a
        className='App-link'
        href={github_oauth_url}
      >
        LOGIN with Github
      </a>
      <h1>Hello, Next.js!</h1>
    </>
  )
}

export default Page;