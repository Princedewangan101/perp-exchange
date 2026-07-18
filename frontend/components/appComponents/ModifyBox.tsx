import { useGSAP } from "@gsap/react";
import { gsap } from "gsap";
import { Draggable } from "gsap/Draggable";
import { useRouter } from "next/navigation";
import React, { useRef } from 'react'

type ModifyBoxProps = {
  isOpen: boolean;
  onClose: () => void;
  orderId: string;
  symbol: string;
};

const ModifyBox = ({ isOpen, onClose, orderId, symbol}: ModifyBoxProps) => {
  const router = useRouter();
  const containerRef = useRef<HTMLDivElement>(null);
  const boxRef = useRef<HTMLDivElement>(null);
  const [tp, setTp] = React.useState<number>();
  const [sl, setSl] = React.useState<number>();


  function handleModifyOrder(e:any) {
    e.preventDefault(); 

    console.log("> orderId: MODIFY: ", orderId);
    console.log("> orderId: MODIFY: ", symbol);
  }

  useGSAP(() => {
    if (!isOpen || !boxRef.current) return;

    gsap.registerPlugin(Draggable);

    Draggable.create(boxRef.current, {
      type: "x,y",
      bounds: containerRef.current,
      onDragStart: function () {
        gsap.to(boxRef.current, { scale: 1.05, duration: 0.1 });
      },
      onDragEnd: function () {
        gsap.to(boxRef.current, { scale: 1, duration: 0.1 });
      },
    });
  }, { scope: containerRef, dependencies: [isOpen] });

  if (!isOpen) return null;

  return (
    <div ref={containerRef} className={`flex justify-center items-center absolute z-50 w-full h-[calc(100dvh-2.5rem)]`}>
      <div className='absolute inset-0 bg-black/20' onClick={onClose} />
      {/* fixed left-4 top-16 z-50 flex  items-center justify-center rounded-lg bg-blue-600 text-white shadow-lg select-none hover:bg-blue-500 active:scale-105 transition-colors duration-150 */}
      <main ref={boxRef} className='absolute z-500 cursor-move shadow-sm shadow-gray-800 rounded-lg'>
        <form onSubmit={handleModifyOrder} className='bg-[#101011] border-zinc-800 w-70 rounded-lg border p-2'>
          <div className='mb-2'>
            <label htmlFor="tp" className="block mb-1 font-bold text-xs text-gray-500">Take Profit</label>
            <input type="number" id="tp" name="tp" placeholder="67835.37"
              value={tp}
              onChange={(e) => setTp(Number(e.target.value))}
              className="w-full p-2 bg-zinc-900/70 rounded-md focus:outline-none
            [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none
            " />
          </div>

          {/* STOP LOSS INPUT */}
          <div className='mb-2'>
            <label htmlFor="sl" className="block mb-1 font-bold text-xs text-gray-500">Stop Loss</label>
            <input type="number" id="sl" name="sl" placeholder="67835.37"
              value={sl}
              onChange={(e) => setSl(Number(e.target.value))}
              className="w-full p-2 bg-zinc-900/70 rounded-md focus:outline-none
            [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none
            " />
          </div>

          {/* MODIFY BTN */}
          <button onClick={handleModifyOrder} className={`w-full px-4 py-2 mb-1 font-semibold text-sm bg-zinc-900 hover:bg-zinc-800 text-gray-500 hover:text-gray-300 rounded-md focus:outline-none disabled:opacity-50`}>
            MODIFY
          </button>

          {/* MODIFY ON CHART BTN */}
          <button onClick={() => { router.push(`/trade/BTC-PERP`) }} className={`w-full px-4 py-2 mb-1 font-semibold text-sm bg-zinc-900 hover:bg-zinc-800 text-gray-500 hover:text-gray-300 rounded-md focus:outline-none disabled:opacity-50`}>
            MODIFY ON CHART
          </button>

          <div className='flex w-full items-center px-2 my-2.5'>
            <hr className='text-gray-700 w-3/10 ml-auto ' />
            <p className='font-bold px-3 text-xs text-gray-600'>OR</p>
            <hr className='text-gray-700 w-3/10 mr-auto ' />
          </div>

          {/* CANCLE BTN */}
          <button type="button" onClick={onClose} className={`w-full px-4 py-2  font-semibold text-sm bg-zinc-900 hover:bg-zinc-800 text-gray-500 hover:text-gray-300 rounded-md focus:outline-none disabled:opacity-50`}>
            CANCLE
          </button>

          <div className='flex w-full items-center px-2 my-2.5'>
            <hr className='text-gray-700 w-3/10 ml-auto ' />
            <p className='font-bold px-3 text-xs text-gray-600'>OR</p>
            <hr className='text-gray-700 w-3/10 mr-auto ' />
          </div>

          {/* CANCLE BTN */}
          <button type="submit" className={`w-full px-4 py-2 font-semibold text-sm bg-zinc-900 hover:bg-zinc-800 text-gray-500 hover:text-gray-300 rounded-md focus:outline-none disabled:opacity-50`}>
            CLOSE
          </button>
        </form>
      </main>

    </div>
  )
}

export default ModifyBox