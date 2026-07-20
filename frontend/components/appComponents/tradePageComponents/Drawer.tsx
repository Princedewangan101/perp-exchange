"use client";

import { drawerPostionHeader, position, position2, positionDrawerColumnName } from '@/lib/timeFrames';
import { useAppStore } from '@/store/store';
import React from 'react';
import { useRouter } from 'next/navigation';

const columnWidths = [
    "w-[80px]", "w-[64px]", "w-[56px]", "w-[80px]", "w-[80px]",
    "w-[80px]", "w-[80px]", "w-[72px]", "w-[88px]", "w-[80px]", "w-[80px]",
];

export function formateTime(timeAsParam: string) {
    return timeAsParam;
}

const Drawer = () => {
    const router = useRouter();
    const headerPosition = drawerPostionHeader;
    const positionData = position;
    const position2Data = position2;
    const isDrawerOpen = useAppStore((state) => state.isDrawerOpen);

    return (
        <div className={`flex flex-col ${isDrawerOpen ? 'w-full' : 'w-0'} transition-all duration-500 overflow-hidden`}>
            <div className="flex items-center justify-between rounded-sm bg-muted px-4 py-1">
                {headerPosition.map((item: string, index: number) => (
                    <p key={index} className="text-zinc-400">{item}</p>
                ))}
            </div>
            <div className="flex flex-col gap">
                {position2Data.length > 0 ? (
                    position2Data.map((item: any, index: number) => (
                        <div key={index} className="flex items-center justify-between rounded-sm px-4 py-2 border-b border-zinc-800">
                            {/* Render position data row */}
                        </div>
                    ))
                ) : (
                    positionData.map((item: any, index: number) => (
                        <div
                            key={index}
                            className={`flex flex-row items-center justify-between rounded-sm px-4 py-2 border-b border-zinc-800 cursor-pointer`}
                            onClick={() => router.push(`/trade/${item.symbol}`)}
                        >
                            <p className={columnWidths[0]}>{item.symbol}</p>
                            <p className={columnWidths[1]}>{item.quantity}</p>
                            <p className={`${columnWidths[2]} ${item.side === "BUY" ? "text-emerald-500" : "text-red-500"}`}>{item.side}</p>
                            <p className={columnWidths[3]}>{"$"}{item.op}</p>
                            <p className={columnWidths[4]}>{"$"}{item.cp}</p>
                            <p className={columnWidths[5]}>{item.sl}</p>
                            <p className={columnWidths[6]}>{item.tp}</p>
                            <p className={`${columnWidths[7]} ${item.pnl > 0 ? "text-emerald-500" : "text-red-500"}`}>{item.side === "BUY" ? "+" : "-"}{item.pnl}</p>
                            <p className={columnWidths[8]}>{item.status}</p>
                            <p className={columnWidths[9]}>{formateTime(item.executionTime)}</p>
                            <p className={columnWidths[10]}>{formateTime(item.closeTime)}</p>
                        </div>
                    ))
                )}
            </div>
        </div>
    );
};

export default Drawer;
