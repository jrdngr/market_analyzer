<script>
    import GammaExposure from './GammaExposure.svelte'

    let symbol = null;
    let minStrike = 0;
    let maxStrike = 0;
    let data = null;
    let reducedData = null;
    let onlyExtremeValues = false;

    async function handleSubmit() {
        console.log("Fetching data");
        data = await (await fetch(`http://localhost:3030/gamma/${symbol}`)).json();
        minStrike = Math.min(...data.prices.map(d => d.strike));
        maxStrike = Math.max(...data.prices.map(d => d.strike));
        setData(data);
	}

    function updateStrikes() {
        setData(data);
    }

    function setData(data) {
        reducedData = data.prices
            .filter(d => d.strike >= minStrike && d.strike <= maxStrike)
            .filter(d => onlyExtremeValues ? Math.abs( d.gamma_exposure) >= data.average_absolute_exposure : true);
    }

</script>

<main>
    Symbol:
    <input bind:value={symbol}>
    <button on:click={handleSubmit}>
        Submit
    </button>
    Min Strike: <input type=number bind:value={minStrike} min=0 on:input={updateStrikes}>
    Max Strike: <input type=number bind:value={maxStrike} min=0 on:input={updateStrikes}>
    Extreme Values: <input type=checkbox bind:checked={onlyExtremeValues} on:change={updateStrikes}>

    {#if data}
        <GammaExposure bind:data={reducedData}/>
    {/if}
</main>

<style>
</style>
