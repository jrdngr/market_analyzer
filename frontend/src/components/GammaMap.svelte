<script>
    import { onMount } from 'svelte';
    import GammaMapChart from './charts/GammaMapChart.svelte'
    import { getGammaExposure, getOhlc, getQuote } from '../common/apis/internal';
import { quantileSorted } from 'd3-array';

    export let options = {
        tickers: null,
        priceLines: null,
        ohlcTicker: null,
        title: null,
        startStrike: 0,
        endStrike: 1,
        aggregate: false,
        fresh: false,
    };

    if (!options.startStrike) {
        options.startStrike = 0;
    }
    if (!options.endStrike) {
        options.endStrike = 1;
    }

    let data = null;
    let reducedData = null;

    let showControls = false;

    let minPriceIndex = options.startStrike;
    let maxPriceIndex = options.endStrike;
    let strikes = [0, 1];
    let brightness = 5;
    let highlightStrikes = true;
    let showGradient = false;
    let flipColors = false;

    let startDate = new Date();
    startDate.setHours(5);
    startDate.setMinutes(30);
    startDate.setMilliseconds(0);
  
    let endDate = new Date();
    endDate.setHours(12);
    endDate.setMinutes(0);
    endDate.setMilliseconds(0);

    let day = startDate.getDay();
    let dateOffset = 0;
    if (day === 0) {
        dateOffset = 2;
    } else if (day === 6) {
        dateOffset = 1;
    }
    startDate.setDate(startDate.getDate() - dateOffset);
    endDate.setDate(endDate.getDate() - dateOffset);

	onMount(async () => {
        console.log("Fetching data");

        startDate = startDate.toJSON().slice(0, -8);
        endDate = endDate.toJSON().slice(0, -8);

        let gexData = await getGexData();

        strikes = gexData.prices.map(p => Number(p.strike));
        strikes.sort((a, b) => a - b);

        gexData.quotes = await getQuotes();
        gexData.ohlc = await getOhlcData();

        data = gexData;

        if (strikes.length > 1) {
            const priceOffset = Math.max(1, data.quotes[0].last * 0.0025);
            const low = data.quotes[0].low - priceOffset;
            const high = data.quotes[0].high + priceOffset;

            for (let i = 1; i < strikes.length; i++) {
                if (low > strikes[i-1] && low <= strikes[i]) {
                    minPriceIndex = i - 1;
                }
                if (high > strikes[i-1] && high <= strikes[i]) {
                    maxPriceIndex = i;
                }
            }
        }

        setInterval(async () => {
            data.quotes = await getQuotes();
            setData();
        }, 30_000);

        setInterval(async () => {
            const ohlc = await getOhlcData();
            data.ohlc = ohlc;
            setData();
        }, 5 * 60_000);

        setInterval(async () => {
            let gexData = await getGexData();
            gexData.quotes = await getQuotes();
            gexData.ohlc = await getOhlcData();
            data = gexData;

            setData();
        }, 60 * 60_000);

        setData();
    });

    async function getGexData() {
        let gexData = null;
        for (const ticker of options.tickers) {
            let tickerGex = await getGammaExposure(ticker.symbol);
            if (!gexData) {
                gexData = tickerGex;
                if (ticker.multiplier) {
                    for (let price of gexData.prices) {
                        let strike = parseFloat(price.strike);
                        strike *= ticker.multiplier;
                        price.strike = strike.toString();
                    }
                }
            } else {
                // Update Strikes
                for (let price of tickerGex.prices) {
                    if (ticker.multiplier) {
                        let strike = parseFloat(price.strike);
                        strike *= ticker.multiplier;
                        price.strike = strike.toString();
                    }
                    for (const gexPrice of gexData.prices) {
                        if (price.strike === gexPrice.strike) {
                            gexPrice.gammaExposure += price.gammaExposure;
                            price.added = true;
                        }
                    }
                }
                for (const price of tickerGex.prices) {
                    if (!price.added) {
                        gexData.prices.push(price);
                    }
                }

                // Update stats
                if (ticker.multiplier) {
                    tickerGex.absoluteMinimumPrice *= ticker.multiplier;
                    tickerGex.weightedAverageAbsolutePrice *= ticker.multiplier;
                }
                gexData.absoluteMaximum = Math.max(gexData.absoluteMaximum, tickerGex.absoluteMaximum);
                gexData.absoluteMinimumPrice = Math.min(gexData.absoluteMinimumPrice, tickerGex.absoluteMinimumPrice);
                gexData.maximumGammaExposure = Math.max(gexData.maximumGammaExposure, tickerGex.maximumGammaExposure);
                gexData.minimumGammaExposure = Math.max(gexData.minimumGammaExposure, tickerGex.minimumGammaExposure);
                gexData.weightedAverageAbsolutePrice = Math.max(gexData.weightedAverageAbsolutePrice, tickerGex.weightedAverageAbsolutePrice);
            }
        }

        return gexData;
    }

    async function getQuotes() {
        const quotes = [];
        for (const ticker of options.priceLines) {
            const quote = await getQuote(ticker.symbol);
            if (ticker.multiplier) {
                quote.last *= ticker.multiplier;
                quote.change *= ticker.multiplier;
                quote.open *= ticker.multiplier;
                quote.high *= ticker.multiplier;
                quote.low *= ticker.multiplier;
                quote.close *= ticker.multiplier;
            }
            quote.color = ticker.color;
            quotes.push(quote);
        }
        return quotes;
    }

    async function getOhlcData() {
        let data = await getOhlc(options.ohlcTicker.symbol, "5min");
        if (options.ohlcTicker.multiplier) {
            for (const bar of data) {
                bar.open *= options.ohlcTicker.multiplier;
                bar.high *= options.ohlcTicker.multiplier;
                bar.low *= options.ohlcTicker.multiplier;
                bar.close *= options.ohlcTicker.multiplier;
                bar.price *= options.ohlcTicker.multiplier;
                bar.vwap *= options.ohlcTicker.multiplier;
            }
        }
        return data;
    }

    function setData() {
        reducedData = Object.assign({}, data);

        reducedData.prices = reducedData.prices
            .filter(d => d.strike >= strikes[minPriceIndex] && d.strike <= strikes[maxPriceIndex]);
            
        reducedData.brightness = brightness;
        reducedData.highlightStrikes = highlightStrikes;
        reducedData.showGradient = showGradient;
        reducedData.flipColors = flipColors;
        reducedData.startDate = startDate;
        reducedData.endDate = endDate;
    }

    function toggleControls() {
        showControls = !showControls;
    }
</script>

<main>
    <div class="header">
        <h3>{options.title}</h3>
        <button on:click={toggleControls}>{showControls ? "-" : "+"}</button>
    </div>
    {#if showControls}
    <div class="controls">
        <span class="strike-label">{strikes[minPriceIndex]}</span><input type="range" min="0" max={maxPriceIndex - 1} step=1 bind:value={minPriceIndex} on:input={setData}>
        <input type="range" min={minPriceIndex + 1} max={strikes.length - 1} step=1 bind:value={maxPriceIndex} on:input={setData}><span class="strike-label">{strikes[maxPriceIndex]}</span>
    </div>
    <div class="controls">
        Start date: <input type=datetime-local bind:value={startDate} on:change={setData}>
        End date: <input type=datetime-local bind:value={endDate} on:change={setData}>
    </div>
    <div class="controls">
        <input type=checkbox bind:checked={highlightStrikes} on:change={setData}> Highlight Strikes
        <input type=checkbox bind:checked={showGradient} on:change={setData}> Show gradient
        <input type=checkbox bind:checked={flipColors} on:change={setData}> Flip colors
        <input 
            type=number 
            bind:value={brightness} 
            min=1
            max=10 
            step=1
            on:change={setData}
        > Brightness
    </div>
    {/if}
    <div class="charts">
        {#if reducedData}
        <div>
            <GammaMapChart bind:data={reducedData}/>
        </div>
        {/if}
    </div>
</main>

<style>
    main {
        background-color: lightgrey;
        border-radius: 10px;
        margin: 0px;
        padding: 5px;
    }

    h3 {
        font-weight: bold;
        margin: 0px;
    }

    .header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin: 0px;
    }

    .header button {
        margin: 0px;
        padding: 0px;
    }

    .controls input[type=range] {
        width: 45%;
    }

    .strike-label {
        font-size: 0.75em;
    }
</style>
