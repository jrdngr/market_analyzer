<script>
    import { onMount } from 'svelte';
    import GammaExposureChart from './charts/GammaExposureChart.svelte'
    import { getGammaExposure, getGammaExposureAggregate, getQuote } from '../common/apis/internal';

    export let options = {
        symbol: null,
        aggregate: false,
        fresh: false,
    };

    let showControls = true;

    let minStrike = 0;
    let maxStrike = 0;
    let data = null;
    let reducedData = null;
    let percentileFilter = 0.0;

	onMount(async () => {
        console.log("Fetching data");
        let gexData;
        if (options.aggregate) {
            gexData = await getGammaExposureAggregate(options.symbol, options);
        } else {
            gexData = await getGammaExposure(options.symbol, options);
        }

        const quote = await getQuote(options.symbol);
        gexData.quote = quote;

        data = gexData;

        centerOnPrice(data);

        setData(data);
    });

    function updateStrikes() {
        setData(data);
    }

    function updatePercentileFilter() {
        setData(data);
    }

    function setData(data) {
        reducedData = Object.assign({}, data);

        reducedData = reducedData.prices
            .map(d => Object.assign({}, d))
            .filter(d => d.strike >= minStrike && d.strike <= maxStrike);

        reducedData.forEach(d => {
            if (Math.abs(d.gammaExposure) < data.absoluteMaximum * percentileFilter) {
                d.gammaExposure = 0;
            }
        });

        reducedData.sort((d1, d2) => d1.strike - d2.strike);
    }

    function centerOnPrice(data) {
        const price = data.quote.last;
        const offsetDigits = Math.floor(Math.log10(price)) - 1;
        
        if (offsetDigits < 0) {
            minStrike = Math.min(...data.prices.map(d => d.strike));
            maxStrike = Math.max(...data.prices.map(d => d.strike));
            return;
        }

        const priceOffset = Math.min(Math.max(2, data.quote.last * 0.1), 100);
        minStrike = Math.floor(price - priceOffset);
        maxStrike = Math.floor(price + priceOffset);
    }

    function toggleControls() {
        showControls = !showControls;
    }
</script>

<main>
    <div class="header">
        <h3>{options.symbol}</h3>
        <button on:click={toggleControls}>{showControls ? "-" : "+"}</button>
    </div>
    {#if showControls}
    <div class="controls">
        Min Strike: <input type=number bind:value={minStrike} min=0 step=1 on:change={updateStrikes}>
        Max Strike: <input type=number bind:value={maxStrike} min=0 step=1 on:change={updateStrikes}>
        Percentile Filter: <input 
            type=number 
            bind:value={percentileFilter} 
            min=0.0 
            max=1.0 
            step=0.01
            on:change={updatePercentileFilter}
        >
    </div>
    {/if}

    <div class="charts">
        {#if data}
            <p>Last: {data.quote.last} | Mid: {data.weightedAverageAbsolutePrice} | Price @ Minimum {data.absoluteMinimumPrice}</p>
            <GammaExposureChart bind:data={reducedData}/>
        {/if}
    </div>
</main>

<style>
    .header {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .header button {
        height: 30px;
    }

    .controls input {
        width: 150px;
    }
</style>
