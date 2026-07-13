import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface AppStoreStateType {
    timeFrame: string,
    symbol: string,
    userId: string,
    userName: string,
    userEmail:string,
    totalBalance: number,
    isDrawerOpen: boolean,
    dropupOpen: boolean,
    expire: number,
    setUserId: (userIdFromServer: string) => void
    setUserName: (userNameFromServer: string) => void
    setUserEmail: (email: string) => void
    setBalance: (balance: number) => void
    setIsDrawerOpen: (isDrawerOpen: boolean) => void
    setDropupOpen: (dropupOpen: boolean) => void
}

export const useAppStore = create<AppStoreStateType>()(
    persist(
        (set, get) => ({
            timeFrame: "",
            userId: "",
            userName: "",
            userEmail:"",
            totalBalance: 0,
            symbol: "",
            isDrawerOpen: true,
            dropupOpen: false,
            expire: 0,
            setUserId: (userIdFromServer: string) => set({ userId: userIdFromServer, expire: Date.now() + 86400000 }),
            setUserName: (userNameFromServer: string) => set({ userName: userNameFromServer }),
            setUserEmail: (email: string) => set({ userEmail: email }),
            setBalance: (balance: number) => set({ totalBalance: balance }),
            setIsDrawerOpen: (isDrawerOpen: boolean) => set({ isDrawerOpen }),
            setDropupOpen: (dropupOpen: boolean) => set({ dropupOpen })
        }),
        {
            name: "perp-exchange",
            partialize: (state) => ({
                userId: state.userId,
                expire: state.expire,
            }),
            merge: (persisted, current) => {
                const p = persisted as { userId?: string; expire?: number }
                if (p.expire && p.expire < Date.now()) {
                    return current
                }
                return { ...current, ...p }
            }
        }
    )
)
