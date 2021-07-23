<script>
    import { onMount } from 'svelte';
    import GammaMapChart from './charts/GammaMapChart.svelte'
    import { getGammaExposureStats, getQuote } from '../common/apis/internal';

    export let symbol = null;

    let data = null;
    let reducedData = null;

    let minPrice = 0;
    let maxPrice = 0;

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
    }
    
    function handleWheel(e) {
        e.preventDefault();

        const priceOffset = data.quote.last * 0.02;

        if (e.deltaY > 0) {
            maxPrice += priceOffset;
            minPrice -= priceOffset;
        }
        if (e.deltaY < 0) {
            if (maxPrice - minPrice < data.quote.last * 0.05) {
                return;
            }
            maxPrice -= priceOffset;
            minPrice += priceOffset;
        }

        minPrice = Math.max(minPrice, 0);
        maxPrice = Math.min(maxPrice, data.quote.week_52_high * data.quote.last * 1.10);

        setData(data)
    }

    function handleMouseMove(e) {
        if (e.buttons === 1) {
            if (e.movementY > 0) {
                maxPrice += 1;
                minPrice += 1;
            } else if (e.movementY < 0) {
                maxPrice -= 1;
                minPrice -= 1;
            }

            minPrice = Math.max(minPrice, 0);
            maxPrice = Math.min(maxPrice, data.quote.week_52_high * data.quote.last * 1.10);

            setData(data)
        }
    }
</script>

<main>
    <div class="charts">
        {#if reducedData}
        <div on:wheel={handleWheel} on:mousemove={handleMouseMove}>
            <GammaMapChart bind:data={reducedData}/>
        </div>
        {/if}
    </div>
</main>

<style>
    .charts {
        width: 80%;
    }
</style>
