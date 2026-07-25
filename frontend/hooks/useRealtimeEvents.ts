"use client";

import { useEffect } from "react";
import toast from "react-hot-toast";
import { realtime } from "@/lib/realtime";

export function useRealtimeEvents() {
  useEffect(() => {
    const handler = (data: unknown) => {
      const fill = data as Record<string, unknown>;
      toast.success(`Order filled — Qty: ${fill.quantity}`, { duration: 4000 });
    };

    realtime.on("OrderFilled", handler);
    return () => realtime.off("OrderFilled", handler);
  }, []);
}
