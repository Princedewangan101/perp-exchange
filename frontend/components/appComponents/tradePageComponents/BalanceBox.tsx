"use client";

import { useAppStore } from '@/store/store';
import axios from 'axios';
import React from 'react';

const BalanceBox = () => {
    const totalBalance = useAppStore((state) => state.totalBalance);
    const setBalance = useAppStore((state) => state.setBalance);
    const [balanceDetailDropDown, setbalanceDetailDropDown] = React.useState<boolean>(false);

    React.useEffect(() => {
        // console.log("\n>[BALANCE] fetching balance on mount/refresh");
        axios.get('http://localhost:5000/api/balance')
            .then((res) => {
                if (res.data.success) {
                    // console.log("\n>[BALANCE] balance fetched:", res.data.balance);
                    setBalance(res.data.balance);
                } else {
                    console.log("\n>[BALANCE] failed to fetch balance:", res.data.message);
                }
            })
            .catch((err) => {
                console.log("\n>[BALANCE] error fetching balance:", err.message);
            });
    }, [setBalance]);

    return (
        <div className="relative font-semibold h-full px-4 min-w-25 flex justify-center items-center cursor-default">
            {/* doc: balance display row with dropdown toggle */}
            <div className='flex gap-1 justify-center items-center group' onClick={() => setbalanceDetailDropDown((v) => !v)}>
                <p>
                    {"$"} {totalBalance.toFixed(2)}
                </p>
                <svg className={`${balanceDetailDropDown ? "rotate-180" : ""} text-gray-500 group-hover:text-gray-300`} width="17" height="17" viewBox="0 0 15 15" fill="none">
                    <path d="M3.13523 6.15803C3.3241 5.95657 3.64052 5.94637 3.84197 6.13523L7.5 9.56464L11.158 6.13523C11.3595 5.94637 11.6759 5.95657 11.8648 6.15803C12.0536 6.35949 12.0434 6.67591 11.842 6.86477L7.84197 10.6148C7.64964 10.7951 7.35036 10.7951 7.15803 10.6148L3.15803 6.86477C2.95657 6.67591 2.94637 6.35949 3.13523 6.15803Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
                </svg>
            </div>

            {/* doc: balance detail dropdown — Balance and Equity values */}
            {balanceDetailDropDown && (
                <div className='absolute right-0 top-10 z-50 shadow-md shadow-gray-800 font-normal text-[15px] px-3 py-1 rounded-md bg-[#18181b]'>
                    <div className='flex gap-2'>
                        <p className='w-20'>Balance</p>
                        :
                        <p className='w-22'>{totalBalance.toFixed(2)}</p>
                    </div>
                    <div className='flex gap-2'>
                        <p className='w-20'>Equity</p>
                        :
                        <p className='w-22'>{totalBalance.toFixed(2)}</p>
                    </div>
                </div>
            )}
        </div>
    );
};

export default BalanceBox;
