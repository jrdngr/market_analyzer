<script>
    import { Link } from "svelte-routing";
    import GammaMap from './GammaMap.svelte';

    let width = 80;

    let chartRows = [
        [
            {
                symbol: "SPY",
            },
            {
                symbol: "SPX",
            },
        ],
        [
            {
                symbol: "QQQ",
            },
            {
                symbol: "NDX",
            }
        ]
    ];

</script>

<main style="--main-width: {width}%">
    {#each chartRows as row}
        <div class="row">
            {#each row as options}
                <div class="map">
                    <GammaMap options={options}/>
                </div>
            {/each}
        </div>
    {/each}

    <div class="controls">
        <input type="range" min="10" max="100" step=1 bind:value={width}>
    </div>
    <Link to="/search">Search</Link>
</main>

<style>
    main {
        width: var(--main-width);
    }

    .row {
        display: flex;
        width: 100%;
    }

    .map {
        width: 100%;
    }

    .controls input {
        width: 100%;
    }
</style>
