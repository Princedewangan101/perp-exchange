"use client";
import React from "react";

const dummyBids = [
  [67050.5, 1.23, 72.45],
  [67000.0, 2.45, 85.12],
  [66980.2, 0.89, 34.56],
  [66950.0, 3.12, 102.34],
  [66920.8, 1.67, 55.78],
  [66880.0, 0.45, 18.90],
  [66850.5, 2.01, 78.23],
  [66800.0, 1.34, 47.89],
  [66770.3, 0.78, 31.45],
  [66730.0, 2.89, 95.67],
  [66680.5, 1.56, 62.34],
  [66620.0, 0.67, 28.90],
  [66550.8, 3.45, 112.56],
  [66500.0, 1.12, 48.23],
  [66450.2, 0.89, 35.67],
  [67050.5, 1.23, 72.45],
  [67000.0, 2.45, 85.12],
  [66980.2, 0.89, 34.56],
  [66950.0, 3.12, 102.34],
  [66920.8, 1.67, 55.78],
  [66880.0, 0.45, 18.90],
  [66850.5, 2.01, 78.23],
  [66800.0, 1.34, 47.89],
  [66770.3, 0.78, 31.45],

];

const dummyAsks = [

  [67450.2, 2.12, 91.23],
  [67500.0, 1.01, 56.78],
  [67550.5, 0.34, 18.45],
  [67600.0, 2.56, 103.45],
  [67650.8, 0.89, 42.12],
  [67700.0, 1.45, 67.89],
  [67750.2, 0.56, 28.34],
  [67800.0, 3.01, 134.56],
  [67100.0, 1.56, 89.34],
  [67150.2, 0.78, 45.12],
  [67200.0, 2.34, 101.56],
  [67250.5, 1.12, 67.89],
  [67300.0, 0.67, 34.23],
  [67350.8, 1.89, 78.45],
  [67400.0, 0.45, 23.56],
  [67450.2, 2.12, 91.23],
  [67500.0, 1.01, 56.78],
  [67550.5, 0.34, 18.45],
  [67600.0, 2.56, 103.45],
  [67650.8, 0.89, 42.12],
  [67700.0, 1.45, 67.89],
  [67750.2, 0.56, 28.34],
  [67800.0, 3.01, 134.56],
];

const OrderBook = ({ symbol }: { symbol: string }) => {
  const [isOpen, setIsOpen] = React.useState(true);

  const maxTotal = Math.max(
    ...dummyBids.map(([, , t]) => t),
    ...dummyAsks.map(([, , t]) => t),
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
            <span>Qty ({symbol.slice(0, -3)})</span>
          </div>

          <div className="h-full overflow-y-auto">
            <div className="px-3 border-b border-zinc-800">
              {dummyAsks.slice().reverse().map(([price, qty], i) => (
                <div key={i} className="flex justify-between py-0.5 relative">
                  <div
                    className="absolute right-0 top-0 h-full bg-red-950/40"
                    style={{ width: `${(qty / maxTotal) * 100}%` }}
                  />
                  <span className="relative z-10 text-red-400">{price.toFixed(1)}</span>
                  <span className="relative z-10">{qty.toFixed(2)}</span>
                </div>
              ))}
            </div>

            <div className="flex items-center justify-between px-3 py-1">
              <span className="font-semibold text-lg">67100.09</span>
              <button className="flex items-center justify-center gap-1 py-1.5 px-1 text-zinc-400 hover:bg-zinc-900 rounded-b">
                <svg width="12" height="12" viewBox="0 0 15 15" fill="none">
                  <path d="M1.84998 7.49998C1.84998 4.66416 4.05979 1.84998 7.49998 1.84998C10.2783 1.84998 11.6515 3.9064 12.2367 5H10.5C10.2239 5 10 5.22386 10 5.5C10 5.77614 10.2239 6 10.5 6H13.5C13.7761 6 14 5.77614 14 5.5V2.5C14 2.22386 13.7761 2 13.5 2C13.2239 2 13 2.22386 13 2.5V4.08318C12.1223 2.60285 10.0556 0.849976 7.49998 0.849976C3.43716 0.849976 0.849976 4.18537 0.849976 7.49998C0.849976 10.8146 3.43716 14.15 7.49998 14.15C9.47524 14.15 11.0561 13.3449 12.2183 12.1797C12.4062 11.9911 12.4053 11.6815 12.2167 11.4937C12.028 11.3059 11.7185 11.3068 11.5307 11.4954C10.5741 12.4616 9.25282 13.15 7.49998 13.15C4.05979 13.15 1.84998 10.3358 1.84998 7.49998Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
                </svg>
                <p>Re-center</p>
              </button>
            </div>

            <div className="px-3 py-1 border-y border-zinc-800">
              {dummyBids.map(([price, qty], i) => (
                <div key={i} className="flex justify-between py-0.5 relative">
                  <div
                    className="absolute right-0 top-0 h-full bg-green-950/40"
                    style={{ width: `${(qty / maxTotal) * 100}%` }}
                  />
                  <span className="relative z-10 text-green-400">{price.toFixed(1)}</span>
                  <span className="relative z-10">{qty.toFixed(2)}</span>
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
