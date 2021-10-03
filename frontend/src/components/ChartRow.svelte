<script>
    import GammaExposure from './GammaExposure.svelte';
    import GammaMap from './GammaMap.svelte';

    const LAST_SEARCHED_KEY = "lastSearchedSymbol";

    let searchSymbol = localStorage.getItem(LAST_SEARCHED_KEY) || "SPY";
    let aggregate = false;
    let fresh = false;

    let charts = [];
    let chartId = 0;

    function addExposureChart() {
        addChart("exposure");
    }

    function addGammaMap() {
        addChart("map");
    }

    function addChart(type) {
        chartId += 1;
    
        const chart = {
            id: chartId,
            type,
            options: {
                title: searchSymbol,
                tickers: [{symbol: searchSymbol}],
                priceLines: [{symbol: searchSymbol, color: "yellow"}],
                ohlcTicker: {symbol: searchSymbol},
            }
        };
        charts = [ ...charts, chart ];
        storeLastSearched();
        fresh = false;
    }

    function storeLastSearched() {
        localStorage.setItem(LAST_SEARCHED_KEY, searchSymbol);
    }
    
    function deleteChart(id) {
        for (let i = 0; i < charts.length; i++) {
            if (charts[i].id == id) {
                charts.splice(i, 1);
            }
        }
        charts = [ ...charts ];
    }
</script>

Symbol:
<input bind:value={searchSymbol}>
<input type=checkbox bind:checked={aggregate}> Aggregate
<input type=checkbox bind:checked={fresh}> Fresh
<button on:click={addGammaMap}>
    Map
</button>
<button on:click={addExposureChart}>
    Exposure
</button>

<div class="charts">
{#each charts as chart}
    <div class="chart">
        {#if chart.type === "exposure"}
            <GammaExposure bind:options={chart.options}/>
        {:else if chart.type === "map"}
            <GammaMap bind:options={chart.options}/>
        {/if}
        <button on:click={deleteChart(chart.id)}>Delete</button>
    </div>
{/each}
</div>

<style>
    .charts {
        display: flex;
    }

    .chart {
        display: flex;
        flex-flow: column;
        width: 60%;
    }

    .chart button {
        width: 100px;
    }
</style>
