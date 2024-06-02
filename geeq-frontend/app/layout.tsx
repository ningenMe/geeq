import { Metadata } from "next";
import { Zen_Kaku_Gothic_New } from "next/font/google";
import GoogleAnalytics from "../components/googleanalytics";

export const metadata: Metadata = {
  title: "geeq",
  description: "エンジニアリングのためのクイズ形式のwebプラットフォーム",
  authors: [{ name: "ningenMe", url: "https://ningenme.net" }],
  keywords: ["geeq", "quiz", "engineering"],
};

export const Font = Zen_Kaku_Gothic_New({
  weight: "400",
  subsets: ["latin"],
});
const style = Object.assign(Font.style, {
  backgroundColor: "#111111",
  color: "#ffffff",
});

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="ja" style={style}>
      <head>
        <GoogleAnalytics />
      </head>
      <body>{children}</body>
    </html>
  );
}
