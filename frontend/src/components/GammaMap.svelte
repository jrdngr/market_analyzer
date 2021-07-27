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

    let brightness = 20;

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
        
        setData(data);
    });

    function setData(data) {
        reducedData = Object.assign({}, data);

        reducedData.prices = reducedData.prices
            .filter(d => d.strike >= minPrice && d.strike <= maxPrice);
            
        minPrice = Math.min(...reducedData.prices.map(d => d.strike));
        maxPrice = Math.max(...reducedData.prices.map(d => d.strike));

        reducedData.brightness = brightness;
    }
    
    function updateMinMaxPrice() {
        setData(data);
    }

    function updateBrightness() {
        setData(data);
    }

    function lowHighPrice() {
        let low;
        let high = 0;
        for (const slice of data.ohlc) {
            if (!low) {
                low = slice.low;
            }
            if (slice.low < low) {
                low = slice.low;
            }
            if (slice.high > high) {
                high = slice.high
            }
        }

        return [low, high]
    }
</script>

<main>
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
    .controls input {
        width: 150px;
    }

    .charts {
        width: 80%;
    }
</style>
