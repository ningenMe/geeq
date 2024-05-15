import { Metadata } from "next";

export const metadata: Metadata = {
  title: "geeq",
  description: "エンジニアリングのためのクイズ形式のwebプラットフォーム",
  authors: [{ name: "ningenMe", url: "https://ningenme.net" }],
  keywords: ["geeq", "quiz", "engineering"],
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="ja">
      <body>{children}</body>
    </html>
  );
}
