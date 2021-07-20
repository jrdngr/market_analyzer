<script>
    import { onMount } from 'svelte';
    import GammaExposureChart from './GammaExposureChart.svelte'
    import { getOptionChain, getQuote } from '../common/apis/tradier';
    import { gammaExposureByPrice } from '../common/math/gammaExposure';

    export let symbol = null;

    let minStrike = 0;
    let maxStrike = 0;
    let data = null;
    let reducedData = null;
    let percentileFilter = 0.3;

	onMount(async () => {
        console.log("Fetching data");
        const optionChain = await getOptionChain(symbol);
        const quote = await getQuote(symbol);
        data = gammaExposureByPrice(optionChain);
        data.quote = quote;

        centerOnPrice(quote.last);
        
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

        // Trim leading and trailer GE = 0
        reducedData = reducedData.prices
            .map(d => Object.assign({}, d))
            .filter(d => d.strike >= minStrike && d.strike <= maxStrike);
            
        reducedData.forEach(d => {
            if (Math.abs(d.gammaExposure) < data.absoluteMaximum * percentileFilter) {
                d.gammaExposure = 0;
            }
        });

        minStrike = Math.min(...reducedData.map(d => d.strike));
        maxStrike = Math.max(...reducedData.map(d => d.strike));
    }

    function centerOnPrice(price) {
        const offsetDigits = Math.floor(Math.log10(price)) - 1;
        const offset = 2 * Math.pow(10, offsetDigits);

        minStrike = price - offset;
        maxStrike = price + offset;
    }
</script>

<main>
    <div class="controls">
        Min Strike: <input type=number bind:value={minStrike} min=0 step=5 on:change={updateStrikes}>
        Max Strike: <input type=number bind:value={maxStrike} min=0 step=5 on:change={updateStrikes}>
        Percentile Filter: <input 
            type=number 
            bind:value={percentileFilter} 
            min=0.0 
            max=1.0 
            step=0.01
            on:change={updatePercentileFilter}
        >
    </div>

    <div class="charts">
        {#if data}
            <p>Last: {data.quote.last}</p>
            <GammaExposureChart bind:data={reducedData}/>
        {/if}
    </div>

</main>

<style>
    .controls input {
        width: 100px;
    }

    .charts {
        width: 80%;
    }
</style>
