<script>
    // import GammaExposure from './GammaExposure.svelte';
    import GammaMap from './GammaMap.svelte';

    let symbol = "SPX";
    let submittedSymbol = null;

    async function handleSubmit() {
        submittedSymbol = symbol;
	}
</script>

Symbol:
<input bind:value={symbol}>
<button on:click={handleSubmit}>
    Submit
</button>

{#key submittedSymbol}
    {#if submittedSymbol}
        <!-- <GammaExposure bind:symbol={submittedSymbol}/> -->
        <GammaMap bind:symbol={submittedSymbol}/>
    {/if}
{/key}

<style>
</style>
