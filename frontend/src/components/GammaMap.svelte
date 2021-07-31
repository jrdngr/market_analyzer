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

    let minPrice = 0;
    let maxPrice = 0;
    let brightness = 0;
    let highlightStrikes = false;

	onMount(async () => {
        console.log("Fetching data");

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
            
        minPrice = Math.min(...reducedData.prices.map(d => d.strike));
        maxPrice = Math.max(...reducedData.prices.map(d => d.strike));

        reducedData.brightness = brightness;
        reducedData.highlightStrikes = highlightStrikes;
    }
    
    function updateMinMaxPrice() {
        setData();
    }

    function updateBrightness() {
        setData();
    }

    function updateHighlightStrikes() {
        setData();
    }
</script>

<main>
    <h3>{symbol}</h3>
    <div class="controls">
        Min Price: <input type=number bind:value={minPrice} min=0 step=5 on:change={updateMinMaxPrice}>
        Max Price: <input type=number bind:value={maxPrice} min=0 step=5 on:change={updateMinMaxPrice}>
        Brightness: <input 
            type=number 
            bind:value={brightness} 
            min=-100
            max=100 
            step=1
            on:change={updateBrightness}
        >
        <input type=checkbox bind:checked={highlightStrikes} on:change={updateHighlightStrikes}> Highlight Strikes
    </div>

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
        padding: 5px;
        width: 50%;
    }

    h3 {
        font-weight: bold;
    }

    .controls input[type=number] {
        width: 150px;
    }
</style>
