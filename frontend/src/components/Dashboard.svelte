<script>
    import GammaMap from './GammaMap.svelte';
</script>

<main>
    <div class="row">
        <div class="map">
            <GammaMap symbol="SPY"/>
        </div>
        <div class="map">
            <GammaMap symbol="SPY"/>
        </div>
    </div>
    <div class="row">
        <div class="map">
            <GammaMap symbol="SPX"/>
        </div>
        <div class="map">
            <GammaMap symbol="NDX"/>
        </div>
    </div>
</main>

<style>
    main {
        margin-right: 10%;
    }

    .row {
        display: flex;
    }

    .map {
        width: 100%;
    }
</style>
