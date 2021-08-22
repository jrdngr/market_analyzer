<script>
    import { onMount } from 'svelte';
    import GammaMapChart from './charts/GammaMapChart.svelte'
    import { getGammaExposure, getOhlc, getQuote } from '../common/apis/internal';

    export let symbol = null;
    export let options = {
        aggregate: false,
        fresh: false,
    };

    let data = null;
    let reducedData = null;

    let showControls = false;

    let minPriceIndex = 0;
    let maxPriceIndex = 1;
    let strikes = [0, 1];
    let brightness = 0;
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

        let gexData = await getGammaExposure(symbol, options);
        
        strikes = gexData.prices.map(p => Number(p.strike));
        strikes.sort((a, b) => a - b);

        const quote = await getQuote(symbol);
        gexData.quote = quote;

        const ohlc = await getOhlc(symbol, "5min");
        gexData.ohlc = ohlc;

        data = gexData;

        if (strikes.length > 1) {
            const priceOffset = Math.max(1, data.quote.last * 0.0025);
            console.log(`${symbol}: ${priceOffset}`);
            const low = data.quote.low - priceOffset;
            const high = data.quote.high + priceOffset;

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
            const quote = await getQuote(data.quote.symbol);
            data.quote = quote
            setData();
        }, 30_000);

        setInterval(async () => {
            const ohlc = await getOhlc(symbol, "5min");
            data.ohlc = ohlc;
            setData();
        }, 5 * 60_000);

        setInterval(async () => {
            let gexData = await getGammaExposure(symbol, options);
            gexData.quote = await getQuote(data.quote.symbol);
            gexData.ohlc = await getOhlc(symbol, "5min");
            data = gexData;
            
            setData();
        }, 60 * 60_000);

        setData();
    });

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
        <h3>{symbol}</h3>
        <button on:click={toggleControls}>{showControls ? "Hide Controls" : "Show Controls"}</button>
    </div>
    {#if showControls}
    <div class="controls">
        <input type="range" min="0" max={maxPriceIndex - 1} step=1 bind:value={minPriceIndex} on:input={setData}>
        <input type="range" min={minPriceIndex + 1} max={strikes.length - 1} step=1 bind:value={maxPriceIndex} on:input={setData}>
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
            min=-100
            max=100 
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
        width: 49%;
    }
</style>
