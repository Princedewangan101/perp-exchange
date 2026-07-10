import Charts from '@/components/appComponents/tradePageComponents/Charts'
import Drawer from '@/components/appComponents/tradePageComponents/Drawer'
import OrderPanel from '@/components/appComponents/tradePageComponents/OrderPanel'
import OrderBook from '@/components/appComponents/tradePageComponents/OrderBook'
import Topbar from '@/components/appComponents/tradePageComponents/Topbar'
import { useAppStore } from '@/store/store'
import TradePageNavbar from '@/components/appComponents/tradePageComponents/TradePageNavbar'

export interface Params {
    params: Promise<{ symbol: string }>
}

const TradePage = async ({ params }: Params) => {
    const paramObj = await params

    useAppStore.setState({
        symbol: paramObj.symbol
    })

    return (
        <div className='flex flex-col gap-1 p-1 h-screen overflow-hidden bg-black'>
            <div className='h-full'>
                <TradePageNavbar />
                <div className='flex gap-1 h-full overflow-hidden'>
                    <div className='flex flex-col gap-1 flex-1 min-w-0 h-full overflow-hidden'>
                        <Topbar symbol={paramObj.symbol} />
                        <div className='h-full overflow-y-auto'>
                            <Charts symbol={paramObj.symbol} />
                            <Drawer />
                        </div>
                    </div>
                    <OrderBook symbol={paramObj.symbol} />
                    <OrderPanel symbol={paramObj.symbol} />
                </div>
            </div>
        </div>
    )
}

export default TradePage
