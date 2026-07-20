"use client";
import TradePageNavbar from '@/components/appComponents/TradePageNavbar';
import Image from 'next/image';
import { useRouter } from 'next/navigation';


const page = () => {
    const router = useRouter();

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
        <div className='w-full flex flex-col gap-1'>
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
            <main className='w-[80%] mx-auto  bg-zinc-900/40'>
                <div className='w-[95%] mx-auto cursor-default'>
                    <div className='flex gap-3 my-4 cursor-default'>
                        <p className='px-2 py-1 w-25 text-center rounded-md text-gray-600 hover:text-gray-300'>SPOT</p>
                        <p className='px-2 py-1 w-25 text-center rounded-md text-gray-600 hover:text-gray-300'>FUTURE</p>
                    </div>
                    <div className='flex justify-between gap-3 my-2 text-gray-500 pl-4'>
                        <p className='w-50'>symbol</p>
                        <p className='w-30'>price</p>
                        <p className='w-30'>openInterest</p>
                        <p className='w-30'>chnagetwofour</p>
                        <p className='w-30'>graph</p>
                    </div>
                    {
                        perp.map(({ symbol, price, openInterest, chnagetwofour }) => (
                            <div onClick={() => { router.push(`/trade/${symbol}`) }} key={symbol} className='flex justify-between gap-3 hover:bg-zinc-900/80 pl-4'>

                                <div className='w-50  py-2 font-semibold text-lg flex gap-2'>
                                    <Image src="/solanalogo.png" alt='logo' height={20} width={25} 
                                    className=''
                                    />
                                    <p>{symbol}</p>
                                </div>
                                <p className='w-30  py-2 font-semibold text-lg'>{price}</p>
                                <p className='w-30  py-2 font-semibold text-lg'>{openInterest}</p>
                                <p className='w-30  py-2 font-semibold text-lg'>{chnagetwofour}</p>
                                <p className='w-30  py-2 font-semibold text-lg'>graph</p>
                            </div>
                        ))
                    }
                </div>
            </main>
            <footer className='h-100 w-full bg-zinc-900/40'>
d
            </footer>

        </div>
    )
}

export default page