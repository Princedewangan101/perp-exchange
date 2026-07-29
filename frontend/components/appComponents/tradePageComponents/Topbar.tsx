"use client";
import Image from 'next/image'
import { useLivePrice } from '@/hooks/useLivePrice';
import BalanceBox from './BalanceBox';

const Topbar = ({ symbol }: { symbol: string }) => {
  const livePrice = useLivePrice();
  const price = livePrice?.price ?? 0;

  return (
    <div className='flex justify-between px-2 py-1 bg-zinc-950 rounded h-10'>
      <div className='flex gap-3'>
        <div className='flex justify-center items-center rounded-md w-fit px-2 gap-2 hover:cursor-default'>
          <Image src="/solanalogo.png" alt='solana-coin-img' width={25} height={25} className='rounded-full' />
          <p className='font-bold'>{symbol.slice(0, -5)}/USD</p>
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="text-gray-400"><polyline points="6 9 12 15 18 9" /></svg>
        </div>

        <div className='flex items-center px-2'>
          <p>{price > 0 ? price.toFixed(2) : "—"}</p>
        </div>
      </div>

      <BalanceBox />
    </div>
  )
}

export default Topbar
