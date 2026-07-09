"use client";
import { drawerPostionHeader, position, positionHeader } from '@/lib/timeFrames';
import { useAppStore } from '@/store/store';
import React from 'react'
import { useRouter } from 'next/navigation'

const columnWidths = [
  "w-[80px]", "w-[64px]", "w-[56px]", "w-[80px]", "w-[80px]",
  "w-[80px]", "w-[80px]", "w-[72px]", "w-[88px]", "w-[80px]", "w-[80px]"
]

const Drawer = () => {
  const router = useRouter();
  const isDrawerOpen = useAppStore((state) => state.isDrawerOpen);
  const [activeTab, setActiveTab] = React.useState("All");

  function formateTime(timeAsParam: string) {
    if (timeAsParam === "-") return "-"

    const date = timeAsParam.split("T")[0].split("-")
    const time = timeAsParam.split("T")[1].split(":")

    return `${date[2]},${date[1]},${String(date[0]).slice(-2)},${time[0]},${time[1]}`
  }

  const hasPositions = position.length > 0;

  return (
    <div className='bg-zinc-950 rounded mt-1 flex flex-col'>
      <div className='flex items-center justify-between px-2 py-1 border-b border-zinc-800'>
        <div className='flex items-center gap-1'>
          {drawerPostionHeader.map((tab) => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={`text-sm px-3 py-1 rounded-md ${activeTab === tab ? "bg-zinc-800" : "hover:bg-zinc-800"}`}
            >
              {tab}
            </button>
          ))}
        </div>
      </div>

      {isDrawerOpen ? (
        <div className='overflow-x-auto '>
          <div className='border-b border-zinc-800 flex'>
            {[...positionHeader, "close-at"].map((title, i) => (
              <div key={i} className={`border-r border-zinc-800 my-2 px-2 ${columnWidths[i]} shrink-0 text-xs text-zinc-500 text-center`}>
                {title}
              </div>
            ))}
          </div>

          <div className='max-h-70 overflow-y-auto'>
            {hasPositions ? (
              position.map(({ symbol, quantity, side, op, cp, closeTime, sl, tp, pnl, executionTime, status }, idx) => (
                <div key={idx} className='border-b border-zinc-800 flex'>
                  <div className={`border-r border-zinc-800 text-xs text-gray-300 ${columnWidths[0]} shrink-0 my-2 py-1 px-2 text-center truncate`}>{symbol}</div>
                  <div className={`border-r border-zinc-800 text-xs text-gray-300 ${columnWidths[1]} shrink-0 my-2 py-1 px-2 text-center truncate`}>{quantity}</div>
                  <div className={`border-r border-zinc-800 text-xs text-gray-300 ${columnWidths[2]} shrink-0 my-2 py-1 px-2 text-center truncate`}>{side}</div>
                  <div className={`border-r border-zinc-800 text-xs text-gray-300 ${columnWidths[3]} shrink-0 my-2 py-1 px-2 text-center truncate`}>{op}</div>
                  <div className={`border-r border-zinc-800 text-xs text-gray-300 ${columnWidths[4]} shrink-0 my-2 py-1 px-2 text-center truncate`}>{cp}</div>
                  <div className={`border-r border-zinc-800 text-xs text-gray-300 ${columnWidths[5]} shrink-0 my-2 py-1 px-2 text-center truncate`}>{sl}</div>
                  <div className={`border-r border-zinc-800 text-xs text-gray-300 ${columnWidths[6]} shrink-0 my-2 py-1 px-2 text-center truncate`}>{tp}</div>
                  <div className={`border-r border-zinc-800 text-xs text-gray-300 ${columnWidths[7]} shrink-0 my-2 py-1 px-2 text-center truncate`}>{pnl}</div>
                  <div className={`border-r border-zinc-800 text-xs text-gray-300 ${columnWidths[8]} shrink-0 my-2 py-1 px-2 text-center truncate`}>{status}</div>
                  <div className={`border-r border-zinc-800 text-xs text-gray-300 ${columnWidths[9]} shrink-0 my-2 py-1 px-2 text-center truncate`}>
                    {closeTime !== "-" ? (
                      <span className='text-[10px]'>{formateTime(closeTime).split(",")[0]}/{formateTime(closeTime).split(",")[1]}</span>
                    ) : "-"}
                  </div>
                  <div className={`text-xs text-gray-300 ${columnWidths[10]} flex-shrink-0 my-2 py-1 px-2 text-center truncate`}>
                    {executionTime !== "-" ? (
                      <span className='text-[10px]'>{formateTime(executionTime).split(",")[0]}/{formateTime(executionTime).split(",")[1]}</span>
                    ) : "-"}
                  </div>
                </div>
              ))
            ) : (
              <div className="flex items-center justify-center py-8 text-zinc-500 text-sm">
                No positions are there
              </div>
            )}
          </div>
        </div>
      )
        :
        (
          <div className='w-full h-70 border flex justify-center items-center'>
            <p className='font-semibold text-gray-500 hover:text-gray-300 hover:cursor-pointer' onClick={() => router.push("/auth")}>
              login or signup first</p>
          </div>
        )
      }
    </div>
  )
}

export default Drawer
