"use client"

import { Slider } from '@/components/ui/slider';
import React from 'react'
import { useMutation } from '@tanstack/react-query';
import axios from 'axios'
import toast from 'react-hot-toast';
import { config } from '@/lib/config';
import { handleError } from '@/app/utils/errorHandler';

type mutationFnData = {
  orderType: string;
  side: number;
  quantity: number;
  price: number;
  leverage: number;
  tp: number;
  sl: number;
  symbol: string;
}

const OrderPanel = ({ symbol }: { symbol: string }) => {
  const [quantity, setQuantity] = React.useState<number | string>("");
  const [leverage, setLeverage] = React.useState(50)
  const [side, setSide] = React.useState<"BUY" | "SELL" | "PROCESS">("BUY");
  const [orderType, setOrderType] = React.useState<"market" | "limit">("market");
  const [price, setPrice] = React.useState<number>();
  const [tp, setTp] = React.useState<number>();
  const [sl, setSl] = React.useState<number>();

  function handleSide() {
    return side === "BUY" ? setSide("SELL") : setSide("BUY")
  }
  function handleOrderType() {
    return orderType === "market" ? setOrderType("limit") : setOrderType("market")
  }
  function handleSliderValue(value: number[]) {
    return setLeverage(value[0])
  }

  const mutation = useMutation({
    mutationFn: async (data: mutationFnData) => {
      if (data.orderType === "market") {
        const body = {symbol: data.symbol, quantity: data.quantity, side: data.side, order_type: "market", tp: Number(data.tp) || 0, sl: Number(data.sl) || 0};
        const res = await axios.post('http://localhost:5000/api/market', body, config);
        return res.data;
      } else {
        const body = {symbol: data.symbol, quantity: data.quantity, side: data.side, price: data.price, order_type: "limit", leverage: data.leverage, tp: data.tp ? Number(data.tp) : null, sl: data.sl ? Number(data.sl) : null};
        const res = await axios.post('http://localhost:5000/api/limit', body, config);
        return res.data;
      }
    },
    onSuccess: (data) => {
      if (data.success) {
        toast.success(data.message || 'Order executed successfully.');
      } else {
        toast.error(data.message || 'Order failed.');
      }
    },
    onError: (error: any) => {
      toast.error(error.message);
    },
  });

  function handleOrderSubmit(e: React.SyntheticEvent<HTMLFormElement>) {
    e.preventDefault();

    console.log("Submitting Order Data:", {
      orderType,
      side: side === "BUY" ? 1 : 0,
      quantity: Number(quantity),
      price: price ? Number(price) : 0,
      leverage,
      tp: tp ? Number(tp) : 0,
      sl: sl ? Number(sl) : 0,
      symbol,
    });

    mutation.mutate({
      orderType,
      side: side === "BUY" ? 1 : 0,
      quantity: Number(quantity),
      price: price ? Number(price) : 0,
      leverage,
      tp: tp ? Number(tp) : 0,
      sl: sl ? Number(sl) : 0,
      symbol,
    });
  }

  return (
    <form onSubmit={handleOrderSubmit} className='bg-zinc-950 min-w-75 max-h-150 rounded px-2 py-1 h-full flex flex-col'>
      <div className='px-2 flex-1 overflow-y-auto'>
        <div className="space-y-5">
          {/* MARKET / LIMTI */}
          <div className="flex w-full bg-zinc-900/70 rounded-md focus:outline-none">
            <p onClick={handleOrderType} className={`w-full text-center p-2 rounded-md ${orderType === "market" && "bg-zinc-800"}`}>Market</p>
            <p onClick={handleOrderType} className={`w-full text-center p-2 rounded-md ${orderType === "limit" && "bg-zinc-800"}`} >Limit</p>
          </div>

          {/* BUY / SELL */}
          <div className="flex w-full bg-zinc-900/70 rounded-md focus:outline-none">
            <p onClick={handleSide} className={`w-full text-center p-2 rounded-md ${side === "BUY" && "bg-testbg"}`}>Buy/Long</p>
            <p onClick={handleSide} className={`w-full text-center p-2 rounded-md ${side === "SELL" && "bg-red-400"}`} >Sell/Short</p>
          </div>

          {/* QUANTITY  INPUT*/}
          <div>
            <label htmlFor="quantity" className="block mb-1 font-medium text-slate-300">Quantity</label>
            <input required type="number" id="quantity" name="quantity" placeholder="0.01"
              value={quantity}
              onChange={(e) => setQuantity(Number(e.target.value))}
              className="w-full p-2 bg-zinc-900/70 rounded-md focus:outline-none 
          [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none
          " />
          </div>

          {/* PRICE  INPUT */}
          {
            orderType === "limit" &&
            (
              <div>
                <label htmlFor="price" className="block mb-1 font-medium text-slate-300">Price</label>
                <input onChange={(e) => { setPrice(Number(e.target.value)) }} value={price} type="number" id="price" name="price" placeholder="63867.90" className="w-full p-2 bg-zinc-900/70 rounded-md focus:outline-none
              [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none
              " />
              </div>
            )
          }

          {/* LEVERAGE SLIDER */}
          <div>
            <div className="mx-auto grid w-full max-w-xs gap-1">
              <div className="flex items-center justify-between gap-2">
                <label htmlFor="quantity" className="block mb-1 font-medium text-slate-300">Leverage</label>
                <span className="text-sm text-muted-foreground">
                  {leverage}x
                </span>
              </div>
              <Slider
                value={[leverage]}
                onValueChange={handleSliderValue}
                min={50}
                max={400}
                step={50}
                className="mx-auto w-full max-w-xs"
              />
            </div>
          </div>

          {/* TAKE PROFIT INPUT */}
          <div>
            <label htmlFor="tp" className="block mb-1 font-medium text-slate-300">Take Profit</label>
            <input type="number" id="tp" name="tp" placeholder="70000"
              value={tp}
              onChange={(e) => setTp(e.target.value)}
              className="w-full p-2 bg-zinc-900/70 rounded-md focus:outline-none
            [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none
            " />
          </div>

          {/* STOP LOSS INPUT */}
          <div>
            <label htmlFor="sl" className="block mb-1 font-medium text-slate-300">Stop Loss</label>
            <input type="number" id="sl" name="sl" placeholder="65000"
              value={sl}
              onChange={(e) => setSl(e.target.value)}
              className="w-full p-2 bg-zinc-900/70 rounded-md focus:outline-none
            [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none
            " />
          </div>

          {/* PRICE DISPLAY BOX */}
          <div className='px-3 py-1.5 bg-zinc-s text-gray-300 rounded-md'>
            <div className='text-sm mb-1 flex justify-between'>
              <p>Mark Price</p>
              <p>67000<span className='text-xs'>$</span></p>
            </div>
            <div className='text-sm mb-1 flex justify-between'>
              <p>Order Price</p>
              <p>quantity : {quantity}</p>
              <p>{67000 * Number(quantity)}<span className='text-xs'>$</span></p>
            </div>
            <div className='text-sm mb-1 flex justify-between'>
              <p>Mrgin Required</p>
              <p>{((67000 * Number(quantity)) / leverage).toFixed(2)}<span className='text-xs'>$</span></p>
            </div>
            <div className='text-sm mb-1 flex justify-between'>
              <p>Fee</p>
              <p>20<span className='text-xs'>$</span></p>
            </div>
          </div>

          {/* ORDER PUNCH BTN */}
          <button type="submit" disabled={mutation.isPending} className={`w-full px-4 py-2.5 mb-3 font-medium text-white rounded-md ${side === "BUY" ? "bg-testbg hover:bg-testbg" : "bg-red-400 hover:bg-red-500"} focus:outline-none disabled:opacity-50`}>
            {mutation.isPending ? "PROCESSING..." : side}
          </button>
        </div>
      </div>
    </form>
  )
}

export default OrderPanel
