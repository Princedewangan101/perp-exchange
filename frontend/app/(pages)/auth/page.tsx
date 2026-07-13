"use client"
import { useMutation } from '@tanstack/react-query';
import axios from 'axios';
import { useRouter } from 'next/navigation';
import React from 'react'
import toast from 'react-hot-toast';
import { useAppStore } from '@/store/store';
import { isValidEmail } from '@/app/utils/emailValidation';

const page = () => {
    const router = useRouter()

    const [email, setEmail] = React.useState<string>("");
    const [password, setPassword] = React.useState<string>("");
    const [openPasswordInput, setopenPasswordInput] = React.useState<boolean>(false);

    const mutation = useMutation({
        mutationFn: async (data: { email: string, password: string }) => {
            const response = await axios.post('http://localhost:5000/api/signup', data)
            return response.data
        },
        onSuccess: (data) => {
            useAppStore.getState().setUserId(data.user_id)

            console.log("> userId: ", useAppStore.getState().userId);
            console.log("> email: ", useAppStore.getState().userEmail);

            toast.success('Registered Successfully!')
            router.push("/market")
        },
        onError: (error: any) => {
            console.log("> error: ", error.message);
            if (error.message === "Request failed with status code 500") {
                toast.error('server error!')
            } else {
                toast.error('error while authenthication!')
            }
            useAppStore.getState().setUserEmail("")
        }
    })
    function handleAuth() {
        mutation.mutate({ email: email, password: password })
    }
    function handleSubmitEmail() {
        if (!isValidEmail(email)) {
            toast.error('invalid email!')
        } else {
            useAppStore.getState().setUserEmail(email)
            setopenPasswordInput(true)
        }
    }
    function handleChangeEmail(e: any) {
        setEmail(e.target.value)
    }
    function handleChangePassword(e: any) {
        setPassword(e.target.value)
    }
    function handleBack() {
        setopenPasswordInput(false)
    }
    return (
        <main className='h-screen w-full'>
            <div className='h-screen w-full flex flex-col justify-center items-center'>
                {
                    mutation.isPending && (
                        <div className=' flex gap-3'>
                            <p className='w-1 h-1 bg-gray-300 animate-bounce' />
                            <p className='w-1 h-1 bg-gray-300 animate-bounce' />
                            <p className='w-1 h-1 bg-gray-300 animate-bounce' />
                        </div>
                    )
                }
                {
                    mutation.isError && (
                        <div className=' flex gap-3'>
                            <p className='text-red-500'></p>
                        </div>
                    )
                }
                {
                    !openPasswordInput && (
                        <form className=''>
                            <input
                                type="email"
                                value={email}
                                onChange={handleChangeEmail}
                                placeholder='email' className='focus:outline-none w-60 bg-zinc-900/70 px-3 py-1' />
                            <p onClick={handleSubmitEmail} className='text-gray-500 hover:text-gray-300 cursor-default text-right'>enter</p>
                        </form>
                    )
                }
                {
                    openPasswordInput && (
                        <>
                            <p className='text-gray-500 text-sm cursor-default'>
                                {email}
                            </p>
                            <form className='mt-2'>
                                <input
                                    type="password"
                                    value={password}
                                    onChange={handleChangePassword}
                                    placeholder='password'
                                    className='focus:outline-none w-60 bg-zinc-900/70 px-3 py-1' />
                                <div className='flex gap-4 '>
                                    <p onClick={handleBack} className='text-gray-500 hover:text-gray-300 cursor-default text-right ml-auto'>back</p>
                                    <p
                                        onClick={handleAuth}
                                        className='text-gray-500 hover:text-gray-300 cursor-default text-right'>enter</p>
                                </div>
                            </form>
                        </>
                    )
                }
            </div>

        </main>
    )
}

export default page