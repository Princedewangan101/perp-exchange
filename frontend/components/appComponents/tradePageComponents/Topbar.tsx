"use client";
import Image from 'next/image'
import BalanceBox from './BalanceBox';

const Topbar = ({ symbol }: { symbol: string }) => {

  return (
    <div className='flex justify-between px-2 py-1 bg-zinc-950 rounded h-10'>
      <div className='flex gap-3'>
        <div className='flex justify-center items-center rounded-md w-fit px-2 gap-2 hover:cursor-default border'>
          <Image src="/solanalogo.png" alt='solana-coin-img' width={25} height={25} className='rounded-full' />
          <p className='font-bold'>{symbol.slice(0, -5)}/USD</p>
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="text-gray-400"><polyline points="6 9 12 15 18 9" /></svg>
        </div>

        <div className='border flex items-center px-2'>
          <p>67100.09</p>
        </div>
        {/* <div className='flex flex-col items-start justify-center px-2'>
          <p className='font-bold text-xs'>24h change</p>
          <p className='font-bold '>+1.43%</p>
        </div> */}
      </div>

      <BalanceBox />
    </div>
  )
}

export default Topbar
