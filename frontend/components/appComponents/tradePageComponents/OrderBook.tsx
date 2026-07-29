"use client";
import React from "react";
import { useOrderBook } from "@/hooks/useOrderBook";

const OrderBook = ({ symbol }: { symbol: string }) => {
  const orderBook = useOrderBook();
  const [isOpen, setIsOpen] = React.useState(true);

  const bids = orderBook?.bids ?? [];
  const asks = orderBook?.asks ?? [];

  const maxQty = Math.max(
    ...bids.map((b) => b.quantity),
    ...asks.map((a) => a.quantity),
    0,
  );

  return (
    <>
      {isOpen ? (
        <div className="bg-zinc-950 px-1 rounded min-w-70 h-150 flex flex-col text-xs">
          <div className="flex items-center justify-between px-3 py-2 border-b border-zinc-800">
            <span>lock</span>
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
            <span>Size ({symbol.slice(0, -3)})</span>
          </div>

          <div className="h-full overflow-y-auto">
            <div className="px-3 border-b border-zinc-800">
              {asks.slice().reverse().map((ask, i) => (
                <div key={i} className="flex justify-between py-0.5 relative">
                  <div
                    className="absolute right-0 top-0 h-full bg-red-950/40"
                    style={{ width: `${(ask.quantity / maxQty) * 100}%` }}
                  />
                  <span className="relative z-10 text-red-400">{ask.price.toFixed(1)}</span>
                  <span className="relative z-10">{ask.quantity.toFixed(2)}</span>
                </div>
              ))}
            </div>

            <div className="flex items-center justify-between px-3 py-1">
              <span className="font-semibold text-lg">
                {asks[0]?.price?.toFixed(1) ?? bids[0]?.price?.toFixed(1) ?? "—"}
              </span>
              <button className="flex items-center justify-center gap-1 py-1.5 px-1 text-zinc-400 hover:bg-zinc-900 rounded-b">
                <svg width="12" height="12" viewBox="0 0 15 15" fill="none">
                  <path d="M1.84998 7.49998C1.84998 4.66416 4.05979 1.84998 7.49998 1.84998C10.2783 1.84998 11.6515 3.9064 12.2367 5H10.5C10.2239 5 10 5.22386 10 5.5C10 5.77614 10.2239 6 10.5 6H13.5C13.7761 6 14 5.77614 14 5.5V2.5C14 2.22386 13.7761 2 13.5 2C13.2239 2 13 2.22386 13 2.5V4.08318C12.1223 2.60285 10.0556 0.849976 7.49998 0.849976C3.43716 0.849976 0.849976 4.18537 0.849976 7.49998C0.849976 10.8146 3.43716 14.15 7.49998 14.15C9.47524 14.15 11.0561 13.3449 12.2183 12.1797C12.4062 11.9911 12.4053 11.6815 12.2167 11.4937C12.028 11.3059 11.7185 11.3068 11.5307 11.4954C10.5741 12.4616 9.25282 13.15 7.49998 13.15C4.05979 13.15 1.84998 10.3358 1.84998 7.49998Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
                </svg>
                <p>Re-center</p>
              </button>
            </div>

            <div className="px-3 py-1 border-y border-zinc-800">
              {bids.map((bid, i) => (
                <div key={i} className="flex justify-between py-0.5 relative">
                  <div
                    className="absolute right-0 top-0 h-full bg-green-950/40"
                    style={{ width: `${(bid.quantity / maxQty) * 100}%` }}
                  />
                  <span className="relative z-10 text-green-400">{bid.price.toFixed(1)}</span>
                  <span className="relative z-10">{bid.quantity.toFixed(2)}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
      ) : (
        <button
          onClick={() => setIsOpen(true)}
          className="bg-zinc-950 rounded self-stretch flex items-center px-1 hover:bg-zinc-900"
        >
          <svg width="16" height="16" viewBox="0 0 15 15" fill="none">
            <path d="M6.15803 3.13508C6.35949 2.94621 6.67591 2.95642 6.86477 3.15788L10.6148 7.15788C10.7951 7.35021 10.7951 7.64949 10.6148 7.84182L6.86477 11.8418C6.67591 12.0433 6.35949 12.0535 6.15803 11.8646C5.95657 11.6758 5.94637 11.3593 6.13523 11.1579L9.56464 7.49985L6.13523 3.84182C5.94637 3.64036 5.95657 3.32394 6.15803 3.13508Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
          </svg>
        </button>
      )}
    </>
  );
};

export default OrderBook;
