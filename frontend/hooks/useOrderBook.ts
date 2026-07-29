"use client";

import { useEffect, useState } from "react";
import { realtime } from "@/lib/realtime";

export interface OrderBookEntry {
  price: number;
  quantity: number;
}

export interface OrderBookData {
  symbol: string;
  bids: OrderBookEntry[];
  asks: OrderBookEntry[];
}

export function useOrderBook() {
  const [orderBook, setOrderBook] = useState<OrderBookData | null>(null);

  useEffect(() => {
    fetch("http://localhost:5000/api/orderbook")
      .then((r) => r.json())
      .then((data) => {
        if (data?.event_type === "OrderBook") {
          setOrderBook(data as OrderBookData);
        }
      })
      .catch(() => {});

    const handler = (data: unknown) => {
      setOrderBook(data as OrderBookData);
    };

    realtime.on("OrderBook", handler);
    return () => realtime.off("OrderBook", handler);
  }, []);

  return orderBook;
}
