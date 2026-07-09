import TradePageNavbar from '@/components/appComponents/tradePageComponents/TradePageNavbar';
import React from 'react'

const page = () => {

    const spot = [
        { symbol: "BTC/USD", name: "Bitcoin", price: "65847.80", volume: "$28.4B", marketCap: "1.3T", chnagetwofour: "+3.56%" },
        { symbol: "ETH/USD", name: "Ethereum", price: "3482.15", volume: "$14.2B", marketCap: "418.5B", chnagetwofour: "+1.84%" },
        { symbol: "SOL/USD", name: "Solana", price: "148.60", volume: "$3.9B", marketCap: "68.9B", chnagetwofour: "-2.15%" },
        { symbol: "BNB/USD", name: "BNB", price: "574.30", volume: "$1.2B", marketCap: "84.1B", chnagetwofour: "+0.45%" },
        { symbol: "XRP/USD", name: "Ripple", price: "0.5840", volume: "$950M", marketCap: "32.6B", chnagetwofour: "+5.12%" },
        { symbol: "ADA/USD", name: "Cardano", price: "0.385", volume: "$410M", marketCap: "13.8B", chnagetwofour: "-1.08%" },
        { symbol: "AVAX/USD", name: "Avalanche", price: "28.15", volume: "$380M", marketCap: "11.1B", chnagetwofour: "-3.40%" },
        { symbol: "LINK/USD", name: "Chainlink", price: "14.90", volume: "$290M", marketCap: "9.2B", chnagetwofour: "+0.92%" }
    ];
    const perp = [
        { symbol: "BTC-PERP", price: "65851.20", openInterest: "$1.8B", chnagetwofour: "+3.54%" },
        { symbol: "ETH-PERP", price: "3482.90", openInterest: "$940M", chnagetwofour: "+1.81%" },
        { symbol: "SOL-PERP", price: "148.65", openInterest: "$320M", chnagetwofour: "-2.18%" },
        { symbol: "BNB-PERP", price: "574.45", openInterest: "$185M", chnagetwofour: "+0.41%" },
        { symbol: "XRP-PERP", price: "0.5845", openInterest: "$110M", chnagetwofour: "+5.08%" },
        { symbol: "ADA-PERP", price: "0.3852", openInterest: "$45M", chnagetwofour: "-1.10%" },
        { symbol: "AVAX-PERP", price: "28.18", openInterest: "$52M", chnagetwofour: "-3.44%" },
        { symbol: "LINK-PERP", price: "14.92", openInterest: "$38M", chnagetwofour: "+0.88%" }
    ];

    return (
        <div className='h-screen w-full flex flex-col gap-1'>
            <TradePageNavbar />
            <div className='w-[80%] mx-auto '>
                <main className='relative border h-80 bg-zinc-900/40 rounded-2xl'>
                    <div className='absolute top-0 bottom-0 right-0 left-0 h-full flex justify-between items-center bg-transparent'>
                        <span className='block border mx-7'>a</span>
                        <span className='block border mx-7'>a</span>
                    </div>
                    <div className='h-full border w-[80%] flex items-start justify-center flex-col px-25'>
                        <h1 className='text-4xl font-bold'>Join the gold rust</h1>
                        <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Iure repellendus consequuntur voluptatibus consectetur similique molestiae ab nesciunt quisquam voluptatum sint deleniti, dolores iusto cum eveniet tenetur, blanditiis iste magni perspiciatis.</p>
                    </div>
                </main>
            </div>
            <div className='w-[80%] mx-auto bg-zinc-900/40'>
                CARDS
            </div>
            <main className='w-[80%] mx-auto bg-zinc-900/40'>
                <div className='w-[95%] mx-auto'>
                    <div className='flex gap-3'>
                        <p>SPOT</p>
                        <p>FUTURE</p>
                    </div>
                    <div className='flex justify-between gap-3 '>
                        <p>symbol</p>
                        <p>price</p>
                        <p>openInterest</p>
                        <p>chnagetwofour</p>
                        <p>graph</p>
                    </div>
                    {
                        perp.map(({ symbol, price, openInterest, chnagetwofour }) => (
                            <div className='flex justify-between gap-3 '>

                                <p className='w-30  py-2 font-semibold text-lg'>{symbol}</p>
                                <p className='w-30  py-2 font-semibold text-lg'>{price}</p>
                                <p className='w-30  py-2 font-semibold text-lg'>{openInterest}</p>
                                <p className='w-30  py-2 font-semibold text-lg'>{chnagetwofour}</p>
                                <p className='w-30  py-2 font-semibold text-lg'>graph</p>
                            </div>
                        ))
                    }

                </div>
            </main>

        </div>
    )
}

export default page