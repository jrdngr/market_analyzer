<script>
    import { DEFAULT_ROWS } from '../common/constants';
    import { Link } from "svelte-routing";
    import GammaMap from './GammaMap.svelte';

    const DASHBOARD_ROWS_KEY = "dashboardRows";

    let width = 80;
    let menuHidden = false;

    let chartRows = loadRows();
    saveRows();

    function saveRows() {
        // localStorage.setItem(DASHBOARD_ROWS_KEY, JSON.stringify(chartRows));
    }

    function loadRows() {
        return DEFAULT_ROWS;
        // let rowString = localStorage.getItem(DASHBOARD_ROWS_KEY) || "[[{symbol: \"SPY\",},{symbol:\"SPX\",},],[{symbol:\"QQQ\",},{symbol:\"NDX\",}]]"

        // try {
        //     return JSON.parse(rowString);
        // } catch {
        //     localStorage.removeItem(DASHBOARD_ROWS_KEY);
        //     return DEFAULT_ROWS;
        // }
    }
</script>

<main style="--main-width: {width}%">
    <div class="charts">
        {#each chartRows as row}
            <div class="row">
                {#each row as options}
                    <div class="map">
                        <GammaMap options={options}/>
                    </div>
                {/each}
            </div>
        {/each}
        <div class="chart-controls">
            <input type="range" min="10" max="100" step=1 bind:value={width}>
        </div>
    </div>

    <div class="menu">
        <button on:click={() => menuHidden = !menuHidden}>{menuHidden ? "+" : "-" }</button>
        {#if !menuHidden}
            <div class="links">
                <Link to="/search">Search</Link>
            </div>
        {/if}
    </div>
</main>

<style>
    main {
        width: var(--main-width);
        display: flex;
        justify-content: space-around;
    }

    .charts {
        width: 100%;
    }

    .menu {
        width: 1%;
    }

    .row {
        display: flex;
        width: 100%;
    }

    .map {
        width: 100%;
    }

    .chart-controls input {
        width: 100%;
    }
</style>
