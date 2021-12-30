export const DEFAULT_ROWS = [
    [
        {
            title: "S&P 500",
            tickers: [
                {
                    symbol: "SPY",
                    multiplier: 10,
                    step: 5,
                }, 
                {
                    symbol: "SPX",
                    step: 5,
                },
            ],
            priceLines: [
                {
                    symbol: "SPY",
                    multiplier: 10,
                    color: "yellow",
                },
                {
                    symbol: "SPX",
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
                    step: 5,
                }, 
                {
                    symbol: "NDX",
                    step: 5,
                },
            ],
            priceLines: [
                {
                    symbol: "QQQ",
                    multiplier: 41,
                    color: "yellow",
                },
                {
                    symbol: "NDX",
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
