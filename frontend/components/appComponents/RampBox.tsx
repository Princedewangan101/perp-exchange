"use client"

import { config } from "@/lib/config";
import { useAppStore } from "@/store/store";
import { useMutation } from "@tanstack/react-query";
import axios from "axios";
import React from 'react'
import toast from "react-hot-toast";

type RampBoxProps = {
  isOpen: boolean;
  onClose: () => void;
  defaultMode?: "deposit" | "withdraw";
};

const RampBox = ({ isOpen, onClose, defaultMode = "deposit" }: RampBoxProps) => {
  const [mode, setMode] = React.useState<"deposit" | "withdraw">(defaultMode);
  const [amount, setAmount] = React.useState<string>("");
  const setBalance = useAppStore((state) => state.setBalance);
  const totalBalance = useAppStore((state) => state.totalBalance);

  const depositMutation = useMutation({
    mutationFn: async (data: { amount: number }) => {
      const body = { amount: Number(data.amount) };
      const res = await axios.post('http://localhost:5000/api/deposit', body, config);
      return res.data;
    },
  });

  const withdrawMutation = useMutation({
    mutationFn: async (data: { amount: number }) => {
      const body = { amount: Number(data.amount) };
      const res = await axios.post('http://localhost:5000/api/withdraw', body, config);
      return res.data;
    },
  });

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    const parsedAmount = Number(amount);
    if (!parsedAmount || parsedAmount <= 0) {
      toast.error("Enter a valid amount.");
      return;
    }
    if (mode === "deposit") {
      const promise = depositMutation.mutateAsync({ amount: parsedAmount }).then((data) => {
        if (!data.success) throw new Error(data.message || 'Deposit failed.');
        setBalance(totalBalance + parsedAmount);
        setAmount("");
        return data;
      });
      toast.promise(promise, {
        loading: 'Depositing...',
        success: (data) => data.message || 'Deposit successful.',
        error: (err) => err.message || 'Deposit failed.',
      });
    } else {
      const promise = withdrawMutation.mutateAsync({ amount: parsedAmount }).then((data) => {
        if (!data.success) throw new Error(data.message || 'Withdraw failed.');
        setBalance(totalBalance - parsedAmount);
        setAmount("");
        return data;
      });
      toast.promise(promise, {
        loading: 'Withdrawing...',
        success: (data) => data.message || 'Withdraw successful.',
        error: (err) => err.message || 'Withdraw failed.',
      });
    }
  }

  if (!isOpen) return null;

  const isLoading = depositMutation.isPending || withdrawMutation.isPending;

  return (
    <div onClick={onClose} className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm">
      <div onClick={(e) => e.stopPropagation()} className="w-80 rounded-lg bg-[#101011] border border-white/5 shadow-xl">

        <form onSubmit={handleSubmit} className="rounded-b-lg p-4">
          {/* TOGGLE: DEPOSIT / WITHDRAW */}
          <div className="flex mb-4 border border-zinc-800 rounded-md overflow-hidden">
            <button
              type="button"
              onClick={() => setMode("deposit")}
              className={`flex-1 py-2 text-sm font-semibold transition-colors ${
                mode === "deposit"
                  ? "bg-lime-500/20 text-lime-400"
                  : "bg-transparent text-gray-500 hover:text-gray-300"
              }`}
            >
              DEPOSIT
            </button>
            <button
              type="button"
              onClick={() => setMode("withdraw")}
              className={`flex-1 py-2 text-sm font-semibold transition-colors ${
                mode === "withdraw"
                  ? "bg-red-500/20 text-red-400"
                  : "bg-transparent text-gray-500 hover:text-gray-300"
              }`}
            >
              WITHDRAW
            </button>
          </div>

          {/* AMOUNT INPUT */}
          <div className="mb-4">
            <label htmlFor="amount" className="block mb-1 font-bold text-xs text-gray-500">
              Amount
            </label>
            <input
              type="number"
              id="amount"
              name="amount"
              placeholder="1000"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              className="w-full p-2 bg-zinc-900/70 rounded-md focus:outline-none
                [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
              disabled={isLoading}
            />
          </div>

          {/* SUBMIT BTN */}
          <button
            type="submit"
            disabled={isLoading}
            className={`w-full px-4 py-2 mb-2 font-semibold text-sm rounded-md focus:outline-none disabled:opacity-50 transition-colors ${
              mode === "deposit"
                ? "bg-lime-600 hover:bg-lime-500 text-white"
                : "bg-red-600 hover:bg-red-500 text-white"
            }`}
          >
            {isLoading ? "PROCESSING..." : mode === "deposit" ? "DEPOSIT" : "WITHDRAW"}
          </button>

          {/* CANCEL BTN */}
          <button
            type="button"
            onClick={onClose}
            disabled={isLoading}
            className="w-full px-4 py-2 font-semibold text-sm bg-zinc-900 hover:bg-zinc-800 text-gray-500 hover:text-gray-300 rounded-md focus:outline-none disabled:opacity-50"
          >
            CANCEL
          </button>
        </form>
      </div>
    </div>
  )
}

export default RampBox
