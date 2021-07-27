<script>
    import { onMount } from 'svelte';
    import GammaMapChart from './charts/GammaMapChart.svelte'
    import { getGammaExposureStats, getGammaExposureAggregate, getOhlc, getQuote } from '../common/apis/internal';

    export let symbol = null;
    export let aggregate = false;

    let data = null;
    let reducedData = null;

    let minPrice = 0;
    let maxPrice = 0;

    let brightness = 1;

	onMount(async () => {
        console.log("Fetching data");

        if (aggregate) {
            data = await getGammaExposureAggregate(symbol);
        } else {
            data = await getGammaExposureStats(symbol);
        }
        
        const quote = await getQuote(symbol);
        data.quote = quote;

        const ohlc = await getOhlc(symbol, "5min");
        data.ohlc = ohlc;

        const priceOffset = data.quote.last * .10;
        minPrice = data.quote.week_52_low - priceOffset;
        maxPrice = data.quote.week_52_high + priceOffset;
        
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
        width: 60%;
    }
</style>
