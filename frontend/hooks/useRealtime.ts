"use client";

import { useEffect, useState } from "react";
import { realtime } from "@/lib/realtime";

export function useRealtime() {
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const token = localStorage.getItem("auth_token");
    if (!token) {
      console.warn("[WS_FE]: no auth_token found, skipping WebSocket connection");
      return;
    }

    realtime.connect(token);
    setConnected(true);
    console.log("[WS_FE]: connected with token");

    return () => {
      realtime.disconnect();
      setConnected(false);
    };
  }, []);

  return { connected };
}
