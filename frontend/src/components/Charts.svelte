<script>
    import GammaExposure from './GammaExposure.svelte';
    import GammaMap from './GammaMap.svelte';

    let symbol = "SPY";
    let aggregate = false;

    let charts = [];

    function addExposureChart() {
        addChart("exposure");
    }

    function addGammaMap() {
        addChart("map");
    }

    function addChart(type) {
        charts = [ ...charts, { type, symbol, aggregate } ];
    }
</script>

Symbol:
<input bind:value={symbol}>
<input type=checkbox bind:checked={aggregate}> Aggregate
<button on:click={addGammaMap}>
    Map
</button>
<button on:click={addExposureChart}>
    Exposure
</button>

{#each charts as chart}
    {#if chart.type === "exposure"}
        <GammaExposure bind:symbol={chart.symbol} bind:aggregate={chart.aggregate}/>
    {:else if chart.type === "map"}
        <GammaMap bind:symbol={chart.symbol} bind:aggregate={chart.aggregate}/>
    {/if}
{/each}

<style>
</style>
