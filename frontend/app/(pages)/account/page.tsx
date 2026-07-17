"use client"
import { formateTime } from '@/components/appComponents/tradePageComponents/Drawer';
import TradePageNavbar from '@/components/appComponents/tradePageComponents/TradePageNavbar';
import { position, positionDrawerColumnName } from '@/lib/timeFrames'
import { useAppStore } from '@/store/store'
import { useRouter } from 'next/navigation';
import React from 'react';

const page = () => {
  const router = useRouter();
  const hasPositions = position.length > 0;
  const [showOrders, setShowOrders] = React.useState<boolean>(false);

  const transactions = [
    { transaction_id: "2", order_id: "55", amount: "5000", type: "DEPOSIT", date: "11/02/26" },
    { transaction_id: "3", order_id: "15", amount: "5000", type: "DEPOSIT", date: "11/02/26" },
    { transaction_id: "5", order_id: "51", amount: "5000", type: "DEPOSIT", date: "11/02/26" },
    { transaction_id: "23", order_id: "52", amount: "5000", type: "DEPOSIT", date: "11/02/26" },
    { transaction_id: "26", order_id: "35", amount: "5000", type: "DEPOSIT", date: "11/02/26" }
  ]

  return (
    <div className='h-screen w-full'>
      <TradePageNavbar />
      <div className='w-9/10 mx-auto '>

        {/* transactions / orders */}
        <div className='flex items-center justify-between px-3 py-1.5 border border-zinc-800 rounded-t-md bg-[#101011]'>
          <div className='flex items-center gap-3'>
            <button onClick={() => setShowOrders(false)} className='text-sm px-3 py-1 rounded-md text-gray-400 hover:text-gray-100'>transactions</button>
            <button onClick={() => setShowOrders(true)} className='text-sm px-3 py-1 rounded-md text-gray-400 hover:text-gray-100'>orders</button>
          </div>
        </div>
        {showOrders ? (
          <main>
            {/* All Pending Open Close */}
            <div className='flex items-center justify-between px-3 py-1.5 border-b border-l border-r border-zinc-800 bg-[#101011]'>
              <div className='flex items-center gap-3'>
                <button className='text-sm px-2 py-1 rounded-md text-gray-400 hover:text-gray-100'>All</button>
                <button className='text-sm px-2 py-1 rounded-md text-gray-400 hover:text-gray-100'>Open</button>
                <button className='text-sm px-2 py-1 rounded-md text-gray-400 hover:text-gray-100'>Pending</button>
                <button className='text-sm px-2 py-1 rounded-md text-gray-400 hover:text-gray-100'>Close</button>
              </div>
            </div>

            {/* Grid Table Container */}
            <div className=' border-x border-b border-zinc-800 rounded-b-md overflow-hidden bg-[#101011]'>

              {/* COLUMN NAME ROW */}
              <div className='flex border-b border-zinc-800'>
                {[...positionDrawerColumnName].map((title, i) => (
                  <div key={i} className='border-r border-zinc-800 flex-1 my-2 px-2 text-xs text-zinc-500 text-center'>
                    {title}
                  </div>
                ))}
                <div className='border-zinc-800 my-2 px-2 flex-1 text-xs text-zinc-500 text-center'>
                  close-at
                </div>
              </div>

              {useAppStore.getState().userId !== "" ? (
                <div className='relative overflow-x-auto mb-10 w-full'>
                  <div>
                    {hasPositions ? (
                      position.map(({ symbol, quantity, side, op, cp, closeTime, sl, tp, pnl, executionTime, status }, idx) => (
                        <div key={idx} className='relative group z-0 border-b border-zinc-800 w-full flex'>
                          <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{symbol}</div>
                          <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{quantity}</div>
                          <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{side}</div>
                          <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{op}</div>
                          <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{cp}</div>
                          <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{sl}</div>
                          <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{tp}</div>
                          <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{pnl}</div>
                          <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{status}</div>
                          <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>
                            {closeTime !== "-" ? (
                              <span className='text-[10px]'>{formateTime(closeTime).split(",")[0]}/{formateTime(closeTime).split(",")[1]}</span>
                            ) : "-"}
                          </div>
                          <div className='text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>
                            {executionTime !== "-" ? (
                              <span className='text-[10px]'>{formateTime(executionTime).split(",")[0]}/{formateTime(executionTime).split(",")[1]}</span>
                            ) : "-"}
                          </div>
                          <div className='group-hover:block hidden absolute top-0 right-0 bottom-0 w-25 z-10 py-2 bg-[#101011]'>
                            <div className='flex items-center justify-around'>
                              <svg xmlns="http://www.w3.org/2000/svg" className='size-5 text-zinc-400 hover:text-gray-100 cursor-default' viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                                <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" />
                              </svg>

                              <svg xmlns="http://www.w3.org/2000/svg" className='size-6 text-zinc-400 hover:text-red-400 cursor-default' viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                                <path d="M18 6 6 18" /><path d="m6 6 12 12" />
                              </svg>
                            </div>
                          </div>
                        </div>
                      ))
                    ) : (
                      <div className='w-full h-70 flex justify-center items-center'>
                        <p className='font-semibold text-gray-500 hover:cursor-pointer' onClick={() => router.push("/auth")}>
                          No positions there
                        </p>
                      </div>
                    )}
                  </div>
                </div>
              )
                :
                (
                  <div className='w-full h-70 flex justify-center items-center'>
                    login or signup first
                  </div>
                )
              }
            </div>
          </main>
        )
          :
          (
            <main>

              {/* Deposit Withdraw Profit Loss */}
              <div className='flex items-center justify-between px-3 py-1.5 border-b border-l border-r border-zinc-800 bg-[#101011]'>
                <div className='flex items-center gap-3'>
                  <button className='text-sm px-2 py-1 rounded-md text-gray-400 hover:text-gray-100'>Deposit</button>
                  <button className='text-sm px-2 py-1 rounded-md text-gray-400 hover:text-gray-100'>Withdraw</button>
                  <button className='text-sm px-2 py-1 rounded-md text-gray-400 hover:text-gray-100'>Profit</button>
                  <button className='text-sm px-2 py-1 rounded-md text-gray-400 hover:text-gray-100'>Loss</button>
                </div>
              </div>

              <div className=' border-x border-b border-zinc-800 rounded-b-md overflow-hidden bg-[#101011]'>

                {/* COLUMN NAME ROW */}
                <div className='flex border-b border-zinc-800'>
                  {["transaction id", "order id", "amt", "type", "date"].map((title, i) => (
                    <div key={i} className='border-r border-zinc-800 flex-1 my-2 px-2 text-xs text-zinc-500 text-center'>
                      {title}
                    </div>
                  ))}
                </div>

                {useAppStore.getState().userId !== "" ? (
                  <div className='relative overflow-x-auto w-full'>
                    <div>
                      {hasPositions ? (
                        transactions.map(({ transaction_id, order_id, amount, type, date }) => (
                          <div key={transaction_id} className='relative z-0 border-b border-zinc-800 w-full flex'>
                            <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{transaction_id}</div>
                            <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{order_id}</div>
                            <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{amount}</div>
                            <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{type}</div>
                            <div className=' text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{date}</div>
                          </div>
                        ))
                      ) : (
                        <div className='w-full h-70 flex justify-center items-center'>
                          <p className='font-semibold text-gray-500 hover:cursor-pointer' onClick={() => router.push("/auth")}>
                            No positions there
                          </p>
                        </div>
                      )}
                    </div>
                  </div>
                )
                  :
                  (
                    <div className='w-full h-70 flex justify-center items-center'>
                      login or signup first
                    </div>
                  )
                }
              </div>

            </main>
          )
        }
      </div>
    </div >
  )
}

export default page