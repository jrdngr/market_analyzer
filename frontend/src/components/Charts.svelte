<script>
    import GammaExposure from './GammaExposure.svelte';
    import GammaMap from './GammaMap.svelte';

    const LAST_SEARCHED_KEY = "lastSearchedSymbol";

    let symbol = localStorage.getItem(LAST_SEARCHED_KEY) || "SPY";
    let aggregate = false;
    let fresh = false;

    let charts = [];

    function addExposureChart() {
        addChart("exposure");
    }

    function addGammaMap() {
        addChart("map");
    }

    function addChart(type) {
        const chart = {
            type,
            symbol,
            options: {
                aggregate,
                fresh,
            }
        };
        charts = [ ...charts, chart ];
        storeLastSearched();
        fresh = false;
    }

    function storeLastSearched() {
        localStorage.setItem(LAST_SEARCHED_KEY, symbol);
    }
</script>

Symbol:
<input bind:value={symbol}>
<input type=checkbox bind:checked={aggregate}> Aggregate
<input type=checkbox bind:checked={fresh}> Fresh
<button on:click={addGammaMap}>
    Map
</button>
<button on:click={addExposureChart}>
    Exposure
</button>

{#each charts as chart}
    {#if chart.type === "exposure"}
        <GammaExposure bind:symbol={chart.symbol} bind:options={chart.options}/>
    {:else if chart.type === "map"}
        <GammaMap bind:symbol={chart.symbol} bind:options={chart.options}/>
    {/if}
{/each}

<style>
</style>
