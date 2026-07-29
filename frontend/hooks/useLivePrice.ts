"use client";

import { useEffect, useState } from "react";
import { realtime } from "@/lib/realtime";

export interface LivePriceData {
  symbol: string;
  price: number;
  time: number;
}

export function useLivePrice() {
  const [price, setPrice] = useState<LivePriceData | null>(null);

  useEffect(() => {
    const handler = (data: unknown) => {
      setPrice(data as LivePriceData);
    };

    realtime.on("LivePrice", handler);
    return () => realtime.off("LivePrice", handler);
  }, []);

  return price;
}
