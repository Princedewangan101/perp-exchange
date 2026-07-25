"use client";

import { useEffect, useState } from "react";
import { realtime } from "@/lib/realtime";

export function useRealtime() {
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const token = localStorage.getItem("auth_token");
    if (!token) return;

    realtime.connect(token);
    setConnected(true);

    return () => {
      realtime.disconnect();
      setConnected(false);
    };
  }, []);

  return { connected };
}
