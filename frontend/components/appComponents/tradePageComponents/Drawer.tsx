"use client";

import { drawerPostionHeader, position } from '@/lib/timeFrames';
import { useAppStore } from '@/store/store';
import React from 'react';
import { useRouter } from 'next/navigation';

function formatTime(iso: string) {
  if (!iso || iso === "-") return "—";
  const d = new Date(iso);
  return d.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function formatPrice(val: number | string) {
  if (val === "-" || val === undefined || val === null) return "—";
  const n = typeof val === "string" ? parseFloat(val) : val;
  if (isNaN(n)) return "—";
  const fixed = n < 1 ? n.toFixed(5) : n.toFixed(2);
  return "$" + Number(fixed).toLocaleString("en-US", {
    minimumFractionDigits: n < 1 ? 5 : 2,
    maximumFractionDigits: n < 1 ? 5 : 2,
  });
}

function formatQty(val: number | string) {
  if (val === "-" || val === undefined || val === null) return "—";
  const n = typeof val === "string" ? parseFloat(val) : val;
  if (isNaN(n)) return "—";
  if (n >= 1000) return n.toLocaleString("en-US", { maximumFractionDigits: 2 });
  if (n >= 1) return n.toFixed(2);
  return n.toFixed(4);
}

function formatPnl(val: number | string) {
  if (val === "-" || val === undefined || val === null) return "—";
  const n = typeof val === "string" ? parseFloat(val) : val;
  if (isNaN(n)) return "—";
  const prefix = n >= 0 ? "+" : "";
  return prefix + "$" + Math.abs(n).toLocaleString("en-US", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}

const columns = [
  { key: "symbol", label: "Symbol", grow: true },
  { key: "side", label: "Side", width: "w-14" },
  { key: "quantity", label: "Qty", width: "w-20 text-right" },
  { key: "op", label: "Open Price", width: "w-24 text-right" },
  { key: "cp", label: "Close Price", width: "w-24 text-right" },
  { key: "sl", label: "SL", width: "w-22 text-right" },
  { key: "tp", label: "TP", width: "w-22 text-right" },
  { key: "pnl", label: "Realized PnL", width: "w-28 text-right" },
  { key: "status", label: "Status", width: "w-22" },
  { key: "executionTime", label: "Time", width: "w-28 text-right" },
];

const Drawer = () => {
  const router = useRouter();
  const isDrawerOpen = useAppStore((state) => state.isDrawerOpen);
  const [activeTab, setActiveTab] = React.useState("All");

  const filtered =
    activeTab === "All"
      ? position
      : position.filter((p) => p.status === activeTab.toUpperCase());

  return (
    <div
      className={`flex flex-col mt-1 bg-zinc-950 rounded transition-all duration-500 overflow-hidden ${isDrawerOpen ? "max-h-[600px]" : "max-h-0"}`}
    >
      {/* Tabs */}
      <div className="flex items-center gap-0.5 px-3 pt-2 pb-1 border-b border-zinc-800">
        {drawerPostionHeader.map((tab) => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            className={`px-3 py-1.5 text-xs font-medium rounded-sm transition-colors ${
              activeTab === tab
                ? "bg-zinc-800 text-zinc-100"
                : "text-zinc-500 hover:text-zinc-300 hover:bg-zinc-900"
            }`}
          >
            {tab}
            {tab !== "All" && (
              <span className="ml-1.5 text-zinc-600">
                ({position.filter((p) => p.status === tab.toUpperCase()).length})
              </span>
            )}
          </button>
        ))}
      </div>

      {/* Column headers */}
      <div className="flex items-center gap-2 px-4 py-1.5 text-[11px] font-medium text-zinc-500 border-b border-zinc-800/60">
        {columns.map((col) => (
          <span
            key={col.key}
            className={`truncate ${col.grow ? "flex-1" : col.width || ""}`}
          >
            {col.label}
          </span>
        ))}
      </div>

      {/* Rows */}
      <div className="flex flex-col text-xs">
        {filtered.length === 0 ? (
          <div className="flex items-center justify-center py-12 text-zinc-600 text-sm">
            No orders found
          </div>
        ) : (
          filtered.map((item: any) => (
            <div
              key={item.id}
              onClick={() => router.push(`/trade/${item.symbol}`)}
              className="flex items-center gap-2 px-4 py-2 border-b border-zinc-800/40 hover:bg-zinc-900/50 cursor-pointer transition-colors"
            >
              <span className="flex-1 truncate font-medium text-zinc-200">
                {item.symbol}
              </span>
              <span
                className={`w-14 font-medium ${
                  item.side === "BUY" ? "text-emerald-500" : "text-red-500"
                }`}
              >
                {item.side}
              </span>
              <span className="w-20 text-right text-zinc-300">
                {formatQty(item.quantity)}
              </span>
              <span className="w-24 text-right text-zinc-300">
                {formatPrice(item.op)}
              </span>
              <span className="w-24 text-right text-zinc-300">
                {formatPrice(item.cp)}
              </span>
              <span className="w-22 text-right text-zinc-300">
                {formatPrice(item.sl)}
              </span>
              <span className="w-22 text-right text-zinc-300">
                {formatPrice(item.tp)}
              </span>
              <span
                className={`w-28 text-right font-medium ${
                  item.pnl === "-" || item.pnl === undefined
                    ? "text-zinc-500"
                    : item.pnl >= 0
                      ? "text-emerald-500"
                      : "text-red-500"
                }`}
              >
                {formatPnl(item.pnl)}
              </span>
              <span className="w-22 text-zinc-500">{item.status}</span>
              <span className="w-28 text-right text-zinc-500">
                {formatTime(item.executionTime)}
              </span>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default Drawer;
