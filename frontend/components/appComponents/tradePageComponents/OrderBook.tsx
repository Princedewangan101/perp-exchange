"use client";
import React from "react";
import { useOrderBook } from "@/hooks/useOrderBook";

const OrderBook = ({ symbol }: { symbol: string }) => {
  const liveOrderBook = useOrderBook();
  const [isOpen, setIsOpen] = React.useState(true);
  const [locked, setLocked] = React.useState(false);
  const [snapshot, setSnapshot] = React.useState(liveOrderBook);

  React.useEffect(() => {
    if (!locked && liveOrderBook) {
      setSnapshot(liveOrderBook);
    }
  }, [liveOrderBook, locked]);

  const orderBook = snapshot;
  const bids = orderBook?.bids ?? [];
  const asks = orderBook?.asks ?? [];

  const asksWithTotal = React.useMemo(() => {
    let cum = 0;
    const cumArr = asks.map((a) => {
      cum += a.quantity;
      return cum;
    });
    return [...asks].reverse().map((a, i) => ({
      ...a,
      total: cumArr[cumArr.length - 1 - i],
    }));
  }, [asks]);

  const bidsWithTotal = React.useMemo(() => {
    let cum = 0;
    return bids.map((b) => {
      cum += b.quantity;
      return { ...b, total: cum };
    });
  }, [bids]);

  const maxTotal = React.useMemo(
    () =>
      Math.max(
        ...asksWithTotal.map((a) => a.total),
        ...bidsWithTotal.map((b) => b.total),
        0,
      ),
    [asksWithTotal, bidsWithTotal],
  );

  const maxQty = React.useMemo(
    () =>
      Math.max(
        ...asks.map((a) => a.quantity),
        ...bids.map((b) => b.quantity),
        0,
      ),
    [asks, bids],
  );

  const coin = symbol.slice(0, -5);

  if (!isOpen) {
    return (
      <button
        onClick={() => setIsOpen(true)}
        className="bg-zinc-950 rounded self-stretch flex items-center px-1 hover:bg-zinc-900"
      >
        <svg width="16" height="16" viewBox="0 0 15 15" fill="none">
          <path d="M6.15803 3.13508C6.35949 2.94621 6.67591 2.95642 6.86477 3.15788L10.6148 7.15788C10.7951 7.35021 10.7951 7.64949 10.6148 7.84182L6.86477 11.8418C6.67591 12.0433 6.35949 12.0535 6.15803 11.8646C5.95657 11.6758 5.94637 11.3593 6.13523 11.1579L9.56464 7.49985L6.13523 3.84182C5.94637 3.64036 5.95657 3.32394 6.15803 3.13508Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
        </svg>
      </button>
    );
  }

  return (
    <div className="bg-zinc-950 px-1 rounded min-w-70 h-150 flex flex-col text-xs">
      <div className="flex items-center justify-between px-3 py-2 border-b border-zinc-800">
        <button
          onClick={() => setLocked((v) => !v)}
          className="p-1 hover:bg-zinc-800 rounded"
        >
          {locked ? (
            <svg width="14" height="14" viewBox="0 0 15 15" fill="none">
              <path d="M7.5 9.125C7.91421 9.125 8.25 8.78921 8.25 8.375C8.25 7.96079 7.91421 7.625 7.5 7.625C7.08579 7.625 6.75 7.96079 6.75 8.375C6.75 8.78921 7.08579 9.125 7.5 9.125Z" fill="currentColor" />
              <path d="M5.5 4.5V3.5C5.5 2.39543 6.39543 1.5 7.5 1.5C8.60457 1.5 9.5 2.39543 9.5 3.5V4.5H11V3.5C11 1.567 9.433 0 7.5 0C5.567 0 4 1.567 4 3.5V4.5H5.5Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
              <path d="M2 6.5C2 5.94772 2.44772 5.5 3 5.5H12C12.5523 5.5 13 5.94772 13 6.5V13.5C13 14.0523 12.5523 14.5 12 14.5H3C2.44772 14.5 2 14.0523 2 13.5V6.5ZM3 6.5H12V13.5H3V6.5Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
            </svg>
          ) : (
            <svg width="14" height="14" viewBox="0 0 15 15" fill="none">
              <path d="M7.5 9.125C7.91421 9.125 8.25 8.78921 8.25 8.375C8.25 7.96079 7.91421 7.625 7.5 7.625C7.08579 7.625 6.75 7.96079 6.75 8.375C6.75 8.78921 7.08579 9.125 7.5 9.125Z" fill="currentColor" />
              <path d="M4 5.5V4.5C4 2.567 5.567 1 7.5 1C9.433 1 11 2.567 11 4.5V5.5H12.5V4.5C12.5 1.73858 10.2614 -0.5 7.5 -0.5C4.73858 -0.5 2.5 1.73858 2.5 4.5V5.5H4Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
              <path d="M2 6.5C2 5.94772 2.44772 5.5 3 5.5H12C12.5523 5.5 13 5.94772 13 6.5V13.5C13 14.0523 12.5523 14.5 12 14.5H3C2.44772 14.5 2 14.0523 2 13.5V6.5Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
            </svg>
          )}
        </button>
        <button
          onClick={() => setIsOpen(false)}
          className="p-1 hover:bg-zinc-800 rounded"
        >
          <svg width="14" height="14" viewBox="0 0 15 15" fill="none">
            <path d="M8.84182 3.13514C9.04327 3.32401 9.05348 3.64042 8.86462 3.84188L5.43521 7.49991L8.86462 11.1579C9.05348 11.3594 9.04327 11.6758 8.84182 11.8647C8.64036 12.0535 8.32394 12.0433 8.13508 11.8419L4.38508 7.84188C4.20477 7.64955 4.20477 7.35027 4.38508 7.15794L8.13508 3.15794C8.32394 2.95648 8.64036 2.94628 8.84182 3.13514Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
          </svg>
        </button>
      </div>

      <div className="px-3 py-1 flex justify-between text-zinc-500 border-b border-zinc-800">
        <span>Price (USD)</span>
        <span>Size ({coin})</span>
        <span>Total ({coin})</span>
      </div>

      <div className="h-full overflow-y-auto">
        <div className="px-3 border-b border-zinc-800">
          {asksWithTotal.map((ask, i) => (
            <div key={i} className="flex justify-between py-0.5 relative">
              <div
                className="absolute right-0 top-0 h-full bg-red-950/40"
                style={{ width: `${(ask.total / maxTotal) * 100}%` }}
              />
              <div
                className="absolute right-0 top-0 h-full bg-red-500/20"
                style={{ width: `${(ask.quantity / maxQty) * 100}%` }}
              />
              <span className="relative z-10 text-red-400 w-[33%]">{ask.price.toFixed(1)}</span>
              <span className="relative z-10 text-right w-[33%]">{ask.quantity.toFixed(3)}</span>
              <span className="relative z-10 text-right w-[33%]">{ask.total.toFixed(3)}</span>
            </div>
          ))}
        </div>

        <div className="flex items-center justify-between px-3 py-1">
          <span className="font-semibold text-base">
            {asks[0]?.price?.toFixed(2) ?? bids[0]?.price?.toFixed(2) ?? "—"}
          </span>
        </div>

        <div className="px-3 py-1 border-y border-zinc-800">
          {bidsWithTotal.map((bid, i) => (
            <div key={i} className="flex justify-between py-0.5 relative">
              <div
                className="absolute right-0 top-0 h-full bg-green-950/40"
                style={{ width: `${(bid.total / maxTotal) * 100}%` }}
              />
              <div
                className="absolute right-0 top-0 h-full bg-green-500/20"
                style={{ width: `${(bid.quantity / maxQty) * 100}%` }}
              />
              <span className="relative z-10 text-green-400 w-[33%]">{bid.price.toFixed(1)}</span>
              <span className="relative z-10 text-right w-[33%]">{bid.quantity.toFixed(3)}</span>
              <span className="relative z-10 text-right w-[33%]">{bid.total.toFixed(3)}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default OrderBook;
