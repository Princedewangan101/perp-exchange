"use client";

import React from "react";
import { useEffect, useRef } from "react";
import { useAppStore } from "@/store/store";

import { debounce } from "@/app/utils/deBounce";
import { createChart, ColorType, CandlestickSeries } from "lightweight-charts";
import { chartAdjuster, timeFrame } from "@/lib/timeFrames";
import DotLoader from "./DotLoader";
import { barColour } from "@/lib/barColor";
import { chartData } from "@/lib/candleDummyData";


const Charts = ({ symbol }: { symbol: string }) => {

  const chartContainerRef = useRef<HTMLDivElement>(null);
  const isChartReady = useRef<boolean>(false)
  const [isChartLoaded, setIsChartLoaded] = React.useState<boolean>(false);
  const [isFetching, setIsFetching] = React.useState<boolean>(false);
  const [chartTimeFrame, setChartTimeFrame] = React.useState<string>("1m");
  const [timeframeExpanded, setTimeframeExpanded] = React.useState(false);

  const isDrawerOpen = useAppStore((state) => state.isDrawerOpen);
  const setIsDrawerOpen = useAppStore((state) => state.setIsDrawerOpen);
  const dropupOpen = useAppStore((state) => state.dropupOpen);
  const setDropupOpen = useAppStore((state) => state.setDropupOpen);

  const symbolWithoutSlash = symbol;
  const symbolWithUnderScore = `${symbolWithoutSlash.slice(0, -3)}_USD`

  function chart() {
    if (!chartContainerRef.current) return;

    const chart = createChart(chartContainerRef.current, {
      layout: {
        background: { type: ColorType.Solid, color: "#09090b" },
        textColor: "gray",
      },
      timeScale: {
        timeVisible: true,
        secondsVisible: false,
      },
      width: chartContainerRef.current.clientWidth,
      height: 430,
    })

    chart.applyOptions({
      grid: {
        vertLines: {
          color: '#18181bb2',
        },
        horzLines: {
          color: '#18181bb2',
        },
      },
    });

    const candlestickSeries = chart.addSeries(CandlestickSeries, barColour);

    console.log("FETCH DATA FROM DB...");

    initCandleData()

    function initCandleData() {
      const formattedData = chartData.map((candle: any) => ({
        time: candle.time,
        open: Number(candle.open),
        high: Number(candle.high),
        low: Number(candle.low),
        close: Number(candle.close),
      }));
      formattedData.sort((a: any, b: any) => (typeof a.time === 'string' ? a.time.localeCompare(b.time) : a.time - b.time))

      candlestickSeries.setData([...formattedData]);

      isChartReady.current = true

    }

    const debouncedScroll = debounce(handleScrollLeftOfChart, 1000)

    chart.timeScale().subscribeVisibleLogicalRangeChange(() => { debouncedScroll(); });

    function handleScrollLeftOfChart() {
      const visibleRange = chart.timeScale().getVisibleRange();
      if (!visibleRange) return;

      if (visibleRange.from < (candlestickSeries.data()[0]?.time as any) && !isFetching) {
        setIsFetching(true);

        const olderData = chartData

        const combinedData = [...olderData, ...candlestickSeries.data()]
        candlestickSeries.setData(combinedData as any);

        setIsFetching(false);
      }
    }

    const resizeObserver = new ResizeObserver(() => {
      chart.applyOptions({ width: chartContainerRef.current?.clientWidth })
    });
    if (chartContainerRef.current) {
      resizeObserver.observe(chartContainerRef.current);
    }

    return () => {
      resizeObserver.disconnect();
      chart.remove();
    }
  }

  useEffect(() => {
    setIsChartLoaded(false)
    chart()
    setIsChartLoaded(true)
  }, [symbolWithoutSlash]);

  return (
    // doc: charts container — candlestick chart + toolbar
    <div className="relative flex flex-col w-full h-fit bg-zinc-950 px-2 pt-2 rounded overflow-y-auto">
      {/* doc: canvas target for lightweight-charts */}
      <div ref={chartContainerRef} className="z-0" />

      {/* doc: loading spinner shown while chart initialises */}
      {!isChartLoaded && (<DotLoader />)}

      {/* doc: dropdown arrow overlay (top-right) */}
      <div className="absolute z-10 top-0.5 right-22 rounded-full  bg-zinc-800">
        <div className=" px-3">
           <svg width="14" height="14" viewBox="0 0 15 15" fill="none">
              <path d="M3.13523 6.15803C3.3241 5.95657 3.64052 5.94637 3.84197 6.13523L7.5 9.56464L11.158 6.13523C11.3595 5.94637 11.6759 5.95657 11.8648 6.15803C12.0536 6.35949 12.0434 6.67591 11.842 6.86477L7.84197 10.6148C7.64964 10.7951 7.35036 10.7951 7.15803 10.6148L3.15803 6.86477C2.95657 6.67591 2.94637 6.35949 3.13523 6.15803Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
            </svg>
        </div>
      </div>

      {/* doc: collapse arrow overlay (bottom-left) */}
      <div className="absolute z-10 bottom-30 left-0.5 rounded-full bg-zinc-800">
        <div className="py-3">
          <svg width="14" height="14" viewBox="0 0 15 15" fill="none" className="rotate-180 block">
            <path d="M8.84182 3.13514C9.04327 3.32401 9.05348 3.64042 8.86462 3.84188L5.43521 7.49991L8.86462 11.1579C9.05348 11.3594 9.04327 11.6758 8.84182 11.8647C8.64036 12.0535 8.32394 12.0433 8.13508 11.8419L4.38508 7.84188C4.20477 7.64955 4.20477 7.35027 4.38508 7.15794L8.13508 3.15794C8.32394 2.95648 8.64036 2.94628 8.84182 3.13514Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
          </svg>
        </div>
      </div>

      {/* doc: bottom toolbar — timeframe selector + chart adjusters + drawer toggle */}
      <div className="w-full flex bg-zinc-s rounded py-1">
        {/* doc: timeframe buttons (1m, 5m, 15m, … + expand) */}
        <div className="h-full flex gap-1 ml-3">
          {
            (timeframeExpanded ? timeFrame : timeFrame.slice(0, 5)).map(({ time }) => (
              <div
                onClick={() => { setChartTimeFrame(time) }}
                key={time}
                className={`${chartTimeFrame === time && "bg-zinc-800"} flex items-center justify-center text-sm p-1 w-8 h-full rounded-sm hover:bg-zinc-800 hover:cursor-pointer`}>
                {time}
              </div>
            ))
          }
          <button
            onClick={() => setTimeframeExpanded(!timeframeExpanded)}
            className="flex items-center justify-center text-sm p-1 w-6 h-full rounded-sm hover:bg-zinc-800 hover:cursor-pointer transition-transform"
          >
            <svg
              className={`${timeframeExpanded ? "rotate-180" : "rotate-0"} transition-transform`}
              width="12" height="12" viewBox="0 0 15 15" fill="none"
            >
              <path d="M3.13523 6.15803C3.3241 5.95657 3.64052 5.94637 3.84197 6.13523L7.5 9.56464L11.158 6.13523C11.3595 5.94637 11.6759 5.95657 11.8648 6.15803C12.0536 6.35949 12.0434 6.67591 11.842 6.86477L7.84197 10.6148C7.64964 10.7951 7.35036 10.7951 7.15803 10.6148L3.15803 6.86477C2.95657 6.67591 2.94637 6.35949 3.13523 6.15803Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
            </svg>
          </button>
        </div>
        {/* doc: right toolbar — chart adjusters (% / log / auto) + Close overlay + drawer toggle */}
        <div className="flex gap-1 ml-auto mr-3 h-full">
          {
            chartAdjuster.map((x) =>
              <div
                key={x}
                className="flex items-center justify-center h-full p-1 text-sm rounded-sm hover:bg-zinc-800 hover:cursor-pointer">
                {x}
              </div>
            )
          }
          {/* doc: Close overlay dropdown */}
          <div className='relative'>
            <button
              onClick={() => setDropupOpen(!dropupOpen)}
              className="text-sm px-2 py-1 hover:bg-zinc-800 rounded-md"
            >
              Close
            </button>
            <div className={`${dropupOpen ? "block" : "hidden"} absolute bottom-8 right-0 border border-zinc-700 bg-zinc-900 h-60 w-60 rounded`} />
          </div>
          {/* doc: drawer toggle button */}
          <button
            onClick={() => setIsDrawerOpen(!isDrawerOpen)}
            className={`${isDrawerOpen ? "rotate-180" : "rotate-0"} text-sm p-1 hover:bg-zinc-800 rounded-md transition-transform`}
          >
            <svg width="16" height="16" viewBox="0 0 15 15" fill="none">
              <path d="M3.13523 6.15803C3.3241 5.95657 3.64052 5.94637 3.84197 6.13523L7.5 9.56464L11.158 6.13523C11.3595 5.94637 11.6759 5.95657 11.8648 6.15803C12.0536 6.35949 12.0434 6.67591 11.842 6.86477L7.84197 10.6148C7.64964 10.7951 7.35036 10.7951 7.15803 10.6148L3.15803 6.86477C2.95657 6.67591 2.94637 6.35949 3.13523 6.15803Z" fill="currentColor" fillRule="evenodd" clipRule="evenodd" />
            </svg>
          </button>
        </div>
      </div>


    </div>
  )
}

export default Charts
