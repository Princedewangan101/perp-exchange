export const timeFrame = [
    { "time": "1m" },
    { "time": "5m" },
    { "time": "15m" },
    { "time": "30m" },
    { "time": "1h" },
    { "time": "4h" },
    { "time": "1d" },
    { "time": "1w" },
    { "time": "1M" },
];

export const chartAdjuster = ["%", "log", "auto"];

export const drawerPostionHeader = ["All", "Pending", "Open", "Close"];

export const orderPanelPriceData = [
    { title: "Mark Price", price: "67859.90" },
    { title: "Order Price", price: "67859.90" },
    { title: "Margin Required", price: "678.59" },
    { title: "Fee", price: "20.67" },
];

export const positionDrawerColumnName = [
    "syl",
    "qty",
    "side",
    "open",
    "close",
    "sl",
    "tp",
    "pnl",
    "status",
    "exe-at",
];

export const position2: any[] = [];

export const position = [
    {
        "id": "trd-201",
        "symbol": "BTCUSDT",
        "quantity": 0.45,
        "side": "BUY",
        "status": "COMPLETED",
        "op": 67250.00,
        "cp": 68410.50,
        "sl": 66100.00,
        "tp": 69000.00,
        "pnl": 522.22,
        "executionTime": "2026-06-03T10:15:30Z",
        "closeTime": "2026-06-03T14:22:15Z",
    },
    {
        "id": "trd-202",
        "symbol": "AAPL",
        "quantity": 50,
        "side": "SELL",
        "status": "COMPLETED",
        "op": 182.40,
        "cp": 184.10,
        "sl": 185.00,
        "tp": 175.00,
        "pnl": -85.00,
        "executionTime": "2026-06-03T13:45:00Z",
        "closeTime": "2026-06-03T15:55:00Z",
    },
    {
        "id": "trd-203",
        "symbol": "EURUSD",
        "quantity": 100000,
        "side": "BUY",
        "status": "EXECUTED",
        "op": 1.08520,
        "cp": "-",
        "sl": 1.08100,
        "tp": 1.09200,
        "pnl": 130.00,
        "executionTime": "2026-06-04T02:10:00Z",
        "closeTime": "-",
    },
    {
        "id": "trd-204",
        "symbol": "ETHUSDT",
        "quantity": 2.50,
        "side": "SELL",
        "status": "COMPLETED",
        "op": 3540.00,
        "cp": 3410.00,
        "sl": 3620.00,
        "tp": 3300.00,
        "pnl": 325.00,
        "executionTime": "2026-06-02T18:20:12Z",
        "closeTime": "2026-06-03T01:45:50Z",
    },
    {
        "id": "trd-205",
        "symbol": "TSLA",
        "quantity": 10,
        "side": "BUY",
        "status": "PENDING",
        "op": 245.80,
        "cp": "-",
        "sl": 238.00,
        "tp": 260.00,
        "pnl": "-",
        "executionTime": "2026-06-04T08:05:22Z",
        "closeTime": "-",
    },
];
