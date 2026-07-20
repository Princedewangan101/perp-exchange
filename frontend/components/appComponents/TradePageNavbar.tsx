"use client";

import RampBox from '@/components/appComponents/RampBox';
import { useAppStore } from '@/store/store';
import { maskEmail } from '@/lib/utils';
import Image from 'next/image'
import React from 'react'
import Link from 'next/link';
import { useRouter } from 'next/navigation';

const TradePageNavbar = () => {
    const router = useRouter();

    const [moreOpen, setMoreOpen] = React.useState(false);
    const moreRef = React.useRef<HTMLDivElement>(null);
    const [profileOpen, setProfileOpen] = React.useState(false);
    const profileRef = React.useRef<HTMLDivElement>(null);
    const [hydrated, setHydrated] = React.useState(false);
    const [rampOpen, setRampOpen] = React.useState(false);
    const [rampMode, setRampMode] = React.useState<"deposit" | "withdraw">("deposit");

    const userId = useAppStore((state) => state.userId);
    const userEmail = useAppStore((state) => state.userEmail);

    React.useEffect(() => {
        setHydrated(true);
    }, []);

    React.useEffect(() => {
        const handler = (e: MouseEvent) => {
            if (moreRef.current && !moreRef.current.contains(e.target as Node)) {
                setMoreOpen(false);
            }
            if (profileRef.current && !profileRef.current.contains(e.target as Node)) {
                setProfileOpen(false);
            }
        };
        document.addEventListener('mousedown', handler);
        return () => document.removeEventListener('mousedown', handler);
    }, []);

    return (
        <nav className='h-10 flex items-center justify-between px-3  text-zinc-300 text-sm'>
            {/* LOGO */}
            <div className='flex items-center gap-2'>
                <Image src="/vercel.svg" alt='logo' width={18} height={18} />
                <p className='text-base font-bold text-white'>Exchange</p>
            </div>

            {/* LEFT DIV */}
            <div className='flex items-center gap-1 mr-auto ml-10'>
                <button onClick={() => { router.push("/market") }} className='px-3 py-1 rounded text-gray-400 hover:text-white'>SPOT</button>
                <button onClick={() => { router.push("/market") }} className='px-3 py-1 rounded text-gray-400 hover:text-white'>FUTURE</button>
                <div className='relative' ref={moreRef}>
                    <button
                        onClick={() => setMoreOpen(!moreOpen)}
                        className='flex items-center gap-1 px-3 py-1 rounded text-gray-400 hover:text-white'
                    >
                        MORE
                        <svg width="12" height="12" viewBox="0 0 15 15" fill="none">
                            <path d="M3.13523 6.15803C3.3241 5.95657 3.64052 5.94637 3.84197 6.13523L7.5 9.56464L11.158 6.13523C11.3595 5.94637 11.6759 5.95657 11.8648 6.15803C12.0536 6.35949 12.0434 6.67591 11.842 6.86477L7.84197 10.6148C7.64964 10.7951 7.35036 10.7951 7.15803 10.6148L3.15803 6.86477C2.95657 6.67591 2.94637 6.35949 3.13523 6.15803Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
                        </svg>
                    </button>
                    {moreOpen && (
                        <div className='absolute top-full right-0 mt-1 w-40 bg-zinc-900 border border-zinc-700 rounded shadow-lg z-50'>
                            <button className='w-full text-left px-3 py-2 hover:bg-zinc-800'>Markets</button>
                            <button className='w-full text-left px-3 py-2 hover:bg-zinc-800'>Favorites</button>
                            <button className='w-full text-left px-3 py-2 hover:bg-zinc-800'>Settings</button>
                        </div>
                    )}
                </div>
            </div>

            {/* RIGHT DIV */}
            <div className='flex items-center gap-2'>
                {/* SEARCH ICON */}
                <button className='p-1.5 rounded text-gray-500 hover:text-white'>
                    <svg width="20" height="20" viewBox="0 0 15 15" fill="none">
                        <path d="M10 6.5C10 8.433 8.433 10 6.5 10C4.567 10 3 8.433 3 6.5C3 4.567 4.567 3 6.5 3C8.433 3 10 4.567 10 6.5ZM9.30884 10.0159C8.53901 10.6318 7.56251 11 6.5 11C4.01472 11 2 8.98528 2 6.5C2 4.01472 4.01472 2 6.5 2C8.98528 2 11 4.01472 11 6.5C11 7.56251 10.6318 8.53901 10.0159 9.30884L12.8536 12.1464C13.0488 12.3417 13.0488 12.6583 12.8536 12.8536C12.6583 13.0488 12.3417 13.0488 12.1464 12.8536L9.30884 10.0159Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
                    </svg>
                </button>

                {/* SUN ICON */}
                <button className='p-1.5 rounded text-gray-500 hover:text-white'>
                    <svg
                        xmlns="http://w3.org"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="2"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    >
                        <circle cx="12" cy="12" r="4" />
                        <path d="M12 2v2" />
                        <path d="M12 20v2" />
                        <path d="m4.93 4.93 1.41 1.41" />
                        <path d="m17.66 17.66 1.41 1.41" />
                        <path d="M2 12h2" />
                        <path d="M20 12h2" />
                        <path d="m6.34 17.66-1.41 1.41" />
                        <path d="m19.07 4.93-1.41 1.41" />
                    </svg>
                </button>

                {/* MOON ICON */}
                <button className='p-1.5 rounded text-gray-500 hover:text-white'>
                    <svg
                        xmlns="http://w3.org"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="2"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    >
                        <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
                    </svg>
                </button>

                {!hydrated || userId === "" ? (
                    <>
                        <Link href="/auth"><button className='flex items-center gap-1 px-2 py-1 rounded font-semibold bg-zinc-800 text-white'>
                            Login
                        </button></Link>
                        <Link href="/auth"><button className='flex items-center gap-1 px-2 py-1 rounded font-semibold bg-zinc-900 text-white'>
                            Signup
                        </button></Link>
                    </>
                )
                    :
                    (
                    <>
                        <div className='flex gap-1'>
                            <button onClick={() => { setRampMode("deposit"); setRampOpen(true); }} className='flex items-center justify-center w-22 py-2 rounded-md font-semibold bg-[#141416] text-white'>
                                deposit
                            </button>
                            <button onClick={() => { setRampMode("withdraw"); setRampOpen(true); }} className='flex items-center justify-center w-22 py-2 rounded-md font-semibold bg-transparent hover:bg-[#141416] text-white'>
                                withdraw
                            </button>
                        </div>
                        <div className='relative' ref={profileRef}>
                            <button onClick={() => setProfileOpen(!profileOpen)} className='p-0'>
                                <Image src="/dp.png" alt='logo' width={30} height={30} className='rounded-full' />
                            </button>
                            {profileOpen && (
                                <div className='absolute top-full right-0 mt-1 w-48 bg-[#101011] border border-[#222225] rounded-xl shadow-lg z-50 p-3'>
                                    <p className='text-sm text-zinc-400 font-bold truncate cursor-default'>{maskEmail(userEmail)}</p>
                                    <div onClick={() => { router.push("/account") }} className='cursor-default text-gray-500 hover:text-gray-300 mt-2 font-bold'>Account</div>
                                    <div className='cursor-default text-gray-500 hover:text-gray-300 mt-2 font-bold'>Logout</div>
                                </div>
                            )}
                        </div>
                    </>
                    )
                }
            </div>
            <RampBox key={String(rampOpen) + rampMode} isOpen={rampOpen} onClose={() => setRampOpen(false)} defaultMode={rampMode} />
        </nav>
    )
}

export default TradePageNavbar