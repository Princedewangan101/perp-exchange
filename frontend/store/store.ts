import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface AppStoreStateType {
    timeFrame: string,
    symbol: string,
    userId: string,
    userName: string,
    totalBalance: number,
    availableBalance: number,
    lockedBalance: number,
    isDrawerOpen: boolean,
    dropupOpen: boolean,
    setUserId: (userIdFromServer: string) => void
    setUserName: (userNameFromServer: string) => void
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
            totalBalance: 0,
            availableBalance: 0,
            lockedBalance: 0,
            symbol: "",
            isDrawerOpen: true,
            dropupOpen: false,
            setUserId: (userIdFromServer: string) => set({ userId: userIdFromServer }),
            setUserName: (userNameFromServer: string) => set({ userName: userNameFromServer }),
            setBalance: (balance: number) => set({ totalBalance: balance }),
            setIsDrawerOpen: (isDrawerOpen: boolean) => set({ isDrawerOpen }),
            setDropupOpen: (dropupOpen: boolean) => set({ dropupOpen })
        }),
        {
            name: "tradingAppStorage"
        }
    )
)
