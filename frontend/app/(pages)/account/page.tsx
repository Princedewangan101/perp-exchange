"use client"
import { formateTime } from '@/components/appComponents/tradePageComponents/Drawer';
import TradePageNavbar from '@/components/appComponents/tradePageComponents/TradePageNavbar';
import { config } from '@/lib/config';
import { positionDrawerColumnName } from '@/lib/timeFrames'
import { useAppStore } from '@/store/store'
import { useQuery } from '@tanstack/react-query';
import axios from 'axios';
import { useRouter } from 'next/navigation';
import React from 'react';
import toast from 'react-hot-toast';

const page = () => {
  const router = useRouter();
  const [isMounted, setIsMounted] = React.useState<boolean>(false);
  const [showOrders, setShowOrders] = React.useState<boolean>(false);

  const userId = useAppStore((state) => state.userId);

  const { isPending: isFetchingOrdersPending, error: fetchingOrdersError, data: ordersData } = useQuery({
    queryKey: ["orderData"],
    queryFn: async () => {
      const res = await axios.get(`http://localhost:5000/api/orders`, config)
      return res.data
    },
    staleTime: 20000,
    retry: false,
  })

  const { isPending: isFetchingTransactionsPending, error: fetchingTransactionsError, data: transactionsData } = useQuery({
    queryKey: ["transactions"],
    queryFn: async () => {
      const res = await axios.post(`http://localhost:5000/api/transactions`, {}, config)
      return res.data
    },
    staleTime: 20000,
    retry: false,
  })

  React.useEffect(() => {
    if (fetchingOrdersError) toast.error(fetchingOrdersError.message)
  }, [fetchingOrdersError])

  React.useEffect(() => {
    if (fetchingTransactionsError) toast.error(fetchingTransactionsError.message)
  }, [fetchingTransactionsError])

  React.useEffect(() => {
    setIsMounted(true)
  }, []);

  return (
    <div className='h-screen w-full'>
      <TradePageNavbar />
      <div className='w-9/10 mx-auto '>

        {/* transactions / orders */}
        <div className='flex items-center justify-between px-3 py-1.5 border border-zinc-800 rounded-t-md bg-[#101011]'>
          <div className='flex items-center justify-between gap-3'>
            <button onClick={() => setShowOrders(false)} className='text-sm px-3 py-1 rounded-md text-gray-400 hover:text-gray-100'>transactions</button>
            <button onClick={() => setShowOrders(true)} className='text-sm px-3 py-1 rounded-md text-gray-400 hover:text-gray-100'>orders</button>
          </div>
          <div>
            <button className='text-sm px-3 py-1 rounded-md text-gray-400 hover:text-gray-100'>refresh</button>
          </div>
        </div>

        {showOrders ? (
          // orders
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

              {!isMounted ? (
                <div className='w-full h-100 flex justify-center items-center'>
                  loading state . . .
                </div>
              )
                :
                (userId !== "" ? (
                  isFetchingOrdersPending ? (
                    <div className='w-full h-100 flex justify-center items-center'>
                      fetching orders . . .
                    </div>
                  ) :
                    (
                      <div className='relative overflow-x-auto mb-10 w-full'>
                        <div>
                          {ordersData?.orders && ordersData.orders.length > 0 ? (
                            ordersData.orders.map((order: any, idx: number) => (
                              <div key={idx} className='relative group z-0 border-b border-zinc-800 w-full flex'>
                                <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{order.symbol}</div>
                                <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{order.quantity}</div>
                                <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{order.side === 0 ? "SELL" : "BUY"}</div>
                                <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{order.open}</div>
                                <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{order.close ?? "-"}</div>
                                <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{order.sl ?? "-"}</div>
                                <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{order.tp ?? "-"}</div>
                                <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{order.pnl ?? "-"}</div>
                                <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{order.status}</div>
                                <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>
                                  {order.updated_at ? (
                                    <span className='text-[10px]'>{formateTime(order.updated_at).split(",")[0]}/{formateTime(order.updated_at).split(",")[1]}</span>
                                  ) : "-"}
                                </div>
                                <div className='text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>
                                  {order.created_at ? (
                                    <span className='text-[10px]'>{formateTime(order.created_at).split(",")[0]}/{formateTime(order.created_at).split(",")[1]}</span>
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
                                No orders there
                              </p>
                            </div>
                          )}
                        </div>
                      </div>
                    )
                )
                  :
                  (
                    <div className='w-full h-70 flex justify-center items-center'>
                      login or signup first
                    </div>
                  )
                )
              }
            </div>
          </main>
        )
          :
          (
            // transactions 
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

                {!isMounted ? (
                  <div className='w-full h-100 flex justify-center items-center'>
                    loading state . . .
                  </div>
                )
                  :
                  (userId !== "" ? (
                    isFetchingTransactionsPending ? (
                      <div className='w-full h-100 flex justify-center items-center'>
                        fetching transactions . . .
                      </div>
                    ) :
                      (
                        <div className='relative overflow-x-auto w-full'>
                          <div>
                            {transactionsData?.transactions && transactionsData.transactions.length > 0 ? (
                              transactionsData.transactions.map((tx: any) => (
                                <div key={tx.transaction_id} className='relative z-0 border-b border-zinc-800 w-full flex'>
                                  <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{tx.transaction_id}</div>
                                  <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{tx.order_id ?? "-"}</div>
                                  <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{tx.balance}</div>
                                  <div className='border-r border-zinc-800 text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>{tx.transaction_type}</div>
                                  <div className=' text-xs text-gray-300 flex-1 my-2 py-1 px-2 text-center'>
                                    {tx.created_at ? new Date(tx.created_at * 1000).toLocaleDateString() : "-"}
                                  </div>
                                </div>
                              ))
                            ) : (
                              <div className='w-full h-70 flex justify-center items-center'>
                                <p className='font-semibold text-gray-500 hover:cursor-pointer' onClick={() => router.push("/auth")}>
                                  No transactions there
                                </p>
                              </div>
                            )}
                          </div>
                        </div>
                      )
                  )
                    :
                    (
                      <div className='w-full h-70 flex justify-center items-center'>
                        login or signup first
                      </div>
                    )
                  )}
              </div>

            </main>
          )
        }
      </div>
    </div >
  )
}

export default page