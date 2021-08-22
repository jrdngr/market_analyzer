<script>
    import { Link } from "svelte-routing";
    import GammaMap from './GammaMap.svelte';

    let width = 100;
</script>

<main style="--main-width: {width}%">
    <div class="row">
        <div class="map">
            <GammaMap symbol="SPY"/>
        </div>
        <div class="map">
            <GammaMap symbol="SPX"/>
        </div>
    </div>
    <div class="row">
        <div class="map">
            <GammaMap symbol="QQQ"/>
        </div>
        <div class="map">
            <GammaMap symbol="NDX"/>
        </div>
    </div>
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
