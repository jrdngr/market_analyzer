<script>
    import { onMount } from 'svelte';
    import GammaMapChart from './charts/GammaMapChart.svelte'
    import { getGammaExposureStats, getQuote } from '../common/apis/internal';

    export let symbol = null;

    let data = null;
    let reducedData = null;

    let minPrice = 0;
    let maxPrice = 0;

    let brightness = 10;

	onMount(async () => {
        console.log("Fetching data");
        data = await getGammaExposureStats(symbol);
        const quote = await getQuote(symbol);
        data.quote = quote;

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

        reducedData.minPrice = minPrice;
        reducedData.maxPrice = maxPrice;
        reducedData.brightness = brightness;
    }
    
    function updateMinMaxPrice() {
        setData(data);
    }

    function updateBrightness() {
        setData(data);
    }
</script>

<main>
    <div class="controls">
        Min Price: <input type=number bind:value={minPrice} min=0 step=5 on:change={updateMinMaxPrice}>
        Max Price: <input type=number bind:value={maxPrice} min=0 step=5 on:change={updateMinMaxPrice}>
        Brightness: <input 
            type=number 
            bind:value={brightness} 
            min=0
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
