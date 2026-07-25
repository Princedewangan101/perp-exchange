"use client";

import { Toaster } from "react-hot-toast";
import { useRealtime } from "@/hooks/useRealtime";
import { useRealtimeEvents } from "@/hooks/useRealtimeEvents";

export default function PagesLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  useRealtime();
  useRealtimeEvents();

  return <>
    {children}
    <Toaster
      position="bottom-left"
      reverseOrder={true}
      toastOptions={{
        style: {
          background: '#333',
          color: '#fff',
        },
      }}
    />
  </>;
}
