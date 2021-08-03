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

    let minPrice = 0;
    let maxPrice = 0;
    let brightness = 0;
    let highlightStrikes = true;
    let flipColors = false;

    let startDate = new Date();
    startDate.setHours(5);
    startDate.setMinutes(30);
    startDate.setMilliseconds(0);
    
    let endDate = new Date();
    endDate.setHours(12);
    endDate.setMinutes(0);
    endDate.setMilliseconds(0);

	onMount(async () => {
        console.log("Fetching data");

        startDate = startDate.toJSON().slice(0, -8);
        endDate = endDate.toJSON().slice(0, -8);

        let gexData = await getGammaExposure(symbol, options);

        const quote = await getQuote(symbol);
        gexData.quote = quote;

        const ohlc = await getOhlc(symbol, "5min");
        gexData.ohlc = ohlc;

        data = gexData;

        const priceOffset = data.quote.last * .01;
        minPrice = data.quote.low - priceOffset;
        maxPrice = data.quote.high + priceOffset;

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
            .filter(d => d.strike >= minPrice && d.strike <= maxPrice);
            
        minPrice = Math.floor(Math.min(...reducedData.prices.map(d => d.strike)));
        maxPrice = Math.floor(Math.max(...reducedData.prices.map(d => d.strike)));

        reducedData.brightness = brightness;
        reducedData.highlightStrikes = highlightStrikes;
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
        Min Price: <input type=number bind:value={minPrice} min=0 step=1 on:change={setData}>
        Max Price: <input type=number bind:value={maxPrice} min=0 step=1 on:change={setData}>
        Brightness: <input 
            type=number 
            bind:value={brightness} 
            min=-100
            max=100 
            step=1
            on:change={setData}
        >
    </div>
    <div class="controls">
        Start date: <input type=datetime-local bind:value={startDate} on:change={setData}>
        End date: <input type=datetime-local bind:value={endDate} on:change={setData}>
    </div>
    <div class="controls">
        <input type=checkbox bind:checked={highlightStrikes} on:change={setData}> Highlight Strikes
        <input type=checkbox bind:checked={flipColors} on:change={setData}> Flip colors
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
        margin: 20px;
        padding: 5px;
    }

    h3 {
        font-weight: bold;
    }

    .header {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .header button {
        height: 30px;
    }

    .controls input[type=number] {
        width: 150px;
    }
</style>
