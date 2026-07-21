"use client";

import { useAppStore } from '@/store/store';
import React from 'react';

const BalanceBox = () => {
    const totalBalance = useAppStore((state) => state.totalBalance);
    const [isProfit, setIsProfit] = React.useState<boolean>(true);
    const [balanceDetailDropDown, setbalanceDetailDropDown] = React.useState<boolean>(false);


    function getRandomPrice() {
        const min = 65000;
        const max = 70000;
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }

    const getPercentage = (price: number) => {
        return ((price - 67859.90) / 67859.90 * 100).toFixed(2);
    };

    return (
        <div className="relative font-semibold h-full px-4 min-w-25 flex justify-center items-center cursor-default">
            <div className='flex gap-1 justify-center items-center group'>
                <p>
                    {"$"} {340550.33}
                </p>
                <svg className={`${balanceDetailDropDown ? "rotate-180" : ""} text-gray-500 group-hover:text-gray-300`} width="17" height="17" viewBox="0 0 15 15" fill="none">
                    <path d="M3.13523 6.15803C3.3241 5.95657 3.64052 5.94637 3.84197 6.13523L7.5 9.56464L11.158 6.13523C11.3595 5.94637 11.6759 5.95657 11.8648 6.15803C12.0536 6.35949 12.0434 6.67591 11.842 6.86477L7.84197 10.6148C7.64964 10.7951 7.35036 10.7951 7.15803 10.6148L3.15803 6.86477C2.95657 6.67591 2.94637 6.35949 3.13523 6.15803Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
                </svg>
            </div>

            {/* DROP-DOWN */}
            {balanceDetailDropDown && (
                <div className='absolute right-0 top-10 z-50 shadow-md shadow-gray-800 font-normal text-[15px] px-3 py-1 rounded-md bg-[#18181b]'>
                    <div className='flex gap-2'>
                        <p className='w-20'>Balance</p>
                        :
                        <p className='w-22'>{340550.33}</p>
                    </div>
                    <div className='flex gap-2'>
                        <p className='w-20'>Equity</p>
                        :
                        <p className='w-22'>{3678.45}</p>
                    </div>
                </div>
            )}
        </div>
    );
};

export default BalanceBox;
