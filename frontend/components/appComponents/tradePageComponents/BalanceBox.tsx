"use client";

import { useAppStore } from '@/store/store';
import React from 'react';

const BalanceBox = () => {
    const totalBalance = useAppStore((state) => state.totalBalance);
    const [isProfit, setIsProfit] = React.useState<boolean>(true);

    function getRandomPrice() {
        const min = 65000;
        const max = 70000;
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }

    const getPercentage = (price: number) => {
        return ((price - 67859.90) / 67859.90 * 100).toFixed(2);
    };

    return (
        <div className="bg-muted px-8 p-4 w-[250px] h-full flex flex-col items-start justify-center">
            <div className="flex gap-2 text-lg">
                {isProfit ? (
                    <div className="flex flex-col h-full">
                        <p className="text-zinc-200/80">Balance</p>
                        <span className="flex items-center gap-2">
                            <p className="text-emerald-500 text-2xl">{"$"}{totalBalance.toFixed(2)}</p>
                            <span className="flex items-center gap-1 text-sm">
                                <p className="text-emerald-500">{"+"}{getRandomPrice()}</p>
                                <p className="text-emerald-500">{"("}{getPercentage(getRandomPrice())}{"%)"}</p>
                            </span>
                        </span>
                    </div>
                ) : (
                    <div className="flex flex-col h-full">
                        <p className="text-zinc-200/80">Balance</p>
                        <span className="flex items-center gap-2">
                            <p className="text-red-500 text-2xl">{"$"}{totalBalance.toFixed(2)}</p>
                            <span className="flex items-center gap-1 text-sm">
                                <p className="text-red-500">{"-"}{getRandomPrice()}</p>
                                <p className="text-red-500">{"("}{getPercentage(getRandomPrice())}{"%)"}</p>
                            </span>
                        </span>
                    </div>
                )}
            </div>
        </div>
    );
};

export default BalanceBox;
