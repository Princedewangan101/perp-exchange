"use client"
import { config } from "@/lib/config";
import { useGSAP } from "@gsap/react";
import { useMutation } from "@tanstack/react-query";
import axios from "axios";
import { gsap } from "gsap";
import { Draggable } from "gsap/Draggable";
import { useRouter } from "next/navigation";
import React, { useRef } from 'react'
import toast from "react-hot-toast";

type ModifyBoxProps = {
  isOpen: boolean;
  onClose: () => void;
  orderId: string;
  symbol: string;
};

type MutationFnData = {
  orderId: string, symbol: string, tp: number, sl: number
};

const ModifyBox = ({ isOpen, onClose, orderId, symbol }: ModifyBoxProps) => {
  const router = useRouter();
  const containerRef = useRef<HTMLDivElement>(null);
  const boxRef = useRef<HTMLDivElement>(null);
  const handleRef = useRef<HTMLDivElement>(null);
  const [tp, setTp] = React.useState<number>();
  const [sl, setSl] = React.useState<number>();

  
  const orderCloseMutation = useMutation({
    mutationFn: async (data: { orderId: string }) => {
      const body = { order_id: String(data.orderId) };

      const res = await axios.post('http://localhost:5000/api/close', body, config);
      return res.data;
    },
    onSuccess: (data) => {
      if (data.success) {
        toast.success(data.message || 'Order closed successfully.');

      } else {
        console.log(`\n> order data: ${data}`);
        toast.error(data.message || 'Order failed to close.');
      }
    },
    onError: (error: any) => {
      toast.error(error.message);
    },
  });

  const mutation = useMutation({
    mutationFn: async (data: MutationFnData) => {
      const body = { order_id: String(data.orderId), symbol: String(data.symbol), tp: Number(data.tp) || 0, sl: Number(data.sl) || 0 };

      const res = await axios.post('http://localhost:5000/api/modify', body, config);
      return res.data;
    },
    onSuccess: (data) => {
      if (data.success) {
        toast.success(data.message || 'Order modified successfully.');

      } else {
        console.log(`\n> order data: ${data}`);
        toast.error(data.message || 'Order failed to modify.');
      }
    },
    onError: (error: any) => {
      toast.error(error.message);
    },
  });

  function handleModifyOrder(e: any) {
    e.preventDefault();
    mutation.mutate({ orderId, symbol, tp: tp ? Number(tp) : 0, sl: sl ? Number(sl) : 0 });
  }

  function handleOrderClose() {
    orderCloseMutation.mutate({ orderId });
  }

  useGSAP(() => {
    if (!isOpen || !boxRef.current) return;

    gsap.registerPlugin(Draggable);

    Draggable.create(boxRef.current, {
      type: "x,y",
      bounds: containerRef.current,
      trigger: handleRef.current,
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
      <main ref={boxRef} className='absolute z-500 shadow-sm shadow-gray-800 rounded-lg'>
        <div ref={handleRef} className='cursor-move rounded-t-lg bg-[#101011] px-3 py-1.5 text-center select-none'>
          <span className='tracking-widest text-gray-600 text-[10px] font-bold'>⋮ DRAG ⋮</span>
        </div>
        <form onSubmit={handleModifyOrder} className='bg-[#101011]  w-70 rounded-b-lg p-2'>
          {/* TAKE PROFIT INPUT */}
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
          <button type="submit" onClick={handleModifyOrder} className={`w-full px-4 py-2 mb-1 font-semibold text-sm bg-zinc-900 hover:bg-zinc-800 text-gray-500 hover:text-gray-300 rounded-md focus:outline-none disabled:opacity-50`}>
            MODIFY
          </button>

          {/* MODIFY ON CHART BTN */}
          <button type="button" onClick={() => { router.push(`/trade/BTC-PERP`) }} className={`w-full px-4 py-2 mb-1 font-semibold text-sm bg-zinc-900 hover:bg-zinc-800 text-gray-500 hover:text-gray-300 rounded-md focus:outline-none disabled:opacity-50`}>
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
          <button type="button" onClick={handleOrderClose} className={`w-full px-4 py-2 font-semibold text-sm bg-zinc-900 hover:bg-zinc-800 text-gray-500 hover:text-gray-300 rounded-md focus:outline-none disabled:opacity-50`}>
            CLOSE
          </button>
        </form>
      </main>

    </div>
  )
}

export default ModifyBox