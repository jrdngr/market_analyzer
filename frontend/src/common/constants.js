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
                    symbol: "SPX",
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
            barTicker: {
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
                    symbol: "NDX",
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
            barTicker: {
                symbol: "QQQ",
                multiplier: 41,
            },
        },
    ],
];