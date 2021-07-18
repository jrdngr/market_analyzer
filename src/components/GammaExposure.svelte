<script>
    import GammaExposure from './GammaExposureChart.svelte'
    import { getOptionChain } from '../common/td';
    import { gammaExposureByPrice } from '../common/math/gammaExposure';

    let symbol = null;
    let minStrike = 0;
    let maxStrike = 0;
    let data = null;
    let reducedData = null;
    let percentileFilter = 0.3;

    async function handleSubmit() {
        console.log("Fetching data");
        const optionChain = await getOptionChain(symbol);
        console.log(optionChain);
        data = gammaExposureByPrice(optionChain);
        console.log(data);
        minStrike = Math.min(...data.prices.map(d => d.strike));
        maxStrike = Math.max(...data.prices.map(d => d.strike));
        setData(data);
	}

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
</script>

<main>
    <div class="controls">
        Symbol:
        <input bind:value={symbol}>
        <button on:click={handleSubmit}>
            Submit
        </button>
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
            <GammaExposure bind:data={reducedData}/>
        {/if}
    </div>

</main>

<style>
    .controls input {
        width: 100px;
    }

    .charts {
        width: 50%;
    }
</style>
