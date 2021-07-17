<script>
    import GammaExposure from './components/GammaExposure.svelte'

    let symbol = null;
    let minStrike = 0;
    let maxStrike = 0;
    let data = null;
    let reducedData = null;
    let percentileFilter = 0.3;

    async function handleSubmit() {
        console.log("Fetching data");
        data = await (await fetch(`http://localhost:3030/gamma/${symbol}`)).json();
        console.log(data);
        minStrike = Math.min(...data.prices.map(d => d.strike));
        maxStrike = Math.max(...data.prices.map(d => d.strike));
        setData(data);
	}

    function updateStrikes() {
        setData(data);
    }

    function setData(data) {
        reducedData = Object.assign({}, data);

        // Trim leading and trailer GE = 0
        reducedData = reducedData.prices
            .map(d => Object.assign({}, d))
            .filter(d => d.strike >= minStrike && d.strike <= maxStrike);
            
        reducedData.forEach(d => {
            if (Math.abs(d.gamma_exposure) < data.absolute_maximum * percentileFilter) {
                d.gamma_exposure = 0;
            }
        });

        minStrike = Math.min(...reducedData.map(d => d.strike));
        maxStrike = Math.max(...reducedData.map(d => d.strike));
    }

</script>

<main>
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
        on:change={updateStrikes}
    >

    {#if data}
        <GammaExposure bind:data={reducedData}/>
    {/if}
</main>

<style>
    input {
        width: 100px;
    }
</style>
