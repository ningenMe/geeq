"use client";

import { usePathname, useSearchParams } from "next/navigation";
import Script from "next/script";
import { Suspense, useEffect } from "react";

export const GA_TAG_ID = process.env.NEXT_PUBLIC_GA_ID as string;

export const pageview = (path: string) => {
  window.gtag("config", GA_TAG_ID, {
    page_path: path,
  });
};

const InnerGoogleAnalytics = () => {
  const pathname = usePathname();
  const searchParams = useSearchParams();

  useEffect(() => {
    if (GA_TAG_ID == "") {
      return;
    }
    const url = pathname + searchParams.toString();
    pageview(url);
  }, [pathname, searchParams]);

  return (
    <>
      <Script
        strategy="lazyOnload"
        src={`https://www.googletagmanager.com/gtag/js?id=${GA_TAG_ID}`}
      />
      <Script id="gtag-init" strategy="afterInteractive">
        {`
          window.dataLayer = window.dataLayer || [];
          function gtag(){dataLayer.push(arguments);}
          gtag('js', new Date());
          gtag('config', '${GA_TAG_ID}', {
            page_path: window.location.pathname,
          });
        `}
      </Script>
    </>
  );
};

export const GoogleAnalytics = () => {
  return (
    <Suspense>
      <InnerGoogleAnalytics />
    </Suspense>
  );
};
