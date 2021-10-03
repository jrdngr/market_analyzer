export const DEFAULT_ROWS = [
    [
        {
            title: "S&P 500",
            tickers: [
                {
                    symbol: "SPY",
                    multiplier: 10,
                }, 
                {
                    symbol: "$SPX.X",
                },
            ],
            priceLines: [
                {
                    symbol: "SPY",
                    multiplier: 10,
                    color: "yellow",
                },
                {
                    symbol: "$SPX.X",
                    color: "orange",
                }
            ],
            ohlcTicker: {
                symbol: "SPY",
                multiplier: 10,
            },
        },
        {
            title: "NASDAQ 100",
            tickers: [
                {
                    symbol: "QQQ",
                    multiplier: 41,
                }, 
                {
                    symbol: "$NDX.X",
                },
            ],
            priceLines: [
                {
                    symbol: "QQQ",
                    multiplier: 41,
                    color: "yellow",
                },
                {
                    symbol: "$NDX.X",
                    color: "orange",
                }
            ],
            ohlcTicker: {
                symbol: "QQQ",
                multiplier: 41,
            },
        },
    ],
];