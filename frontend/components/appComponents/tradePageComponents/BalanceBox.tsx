"use client";

import { useAppStore } from '@/store/store';
import React from 'react'

const BalanceBox = () => {
  let totalBalance = useAppStore((state) => state.totalBalance)

  const [isProfit, setIsProfit] = React.useState<boolean>(true);


  function getRandomPrice() {
    const min = 65000;
    const max = 66000;
    return Number((Math.random() * (max - min) + min).toFixed(2));
  }

  let price = 0;
  let diff;



  // setInterval(() => {

  //   let randomPrice = getRandomPrice();

  //   diff = randomPrice - price

  //   if (diff > 0) {
  //     totalBalance = Number(totalBalance) + diff
  //   } else {
  //     totalBalance = Number(totalBalance) - diff
  //   }
  //   if (randomPrice > 65500) {
  //     setIsProfit(true)
  //   }else{
  //     setIsProfit(false)
  //   }


  //   useAppStore.getState().setBalance(Number(totalBalance.toFixed(2)))
  // }, 3000)


  return (
    <div className='flex justify-center items-center rounded-md w-fit px-2 gap-2 hover:cursor-default'>
      <p className={`font-semibold ${isProfit === true ? "text-lime-400" : "text-red-500"}`}>{totalBalance}</p>
    </div>
  )
}

export default BalanceBox
