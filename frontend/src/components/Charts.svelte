<script>
    // import GammaExposure from './GammaExposure.svelte';
    import GammaMap from './GammaMap.svelte';

    let symbol = "SPY";
    let submittedSymbol = null;
    let aggregate = false;

    async function handleSubmit() {
        submittedSymbol = symbol;
	}
</script>

Symbol:
<input bind:value={symbol}>
<input type=checkbox bind:checked={aggregate}> Aggregate
<button on:click={handleSubmit}>
    Submit
</button>

{#key submittedSymbol}
    {#if submittedSymbol}
        <!-- <GammaExposure bind:symbol={submittedSymbol} bind:aggregate={aggregate}/> -->
        <GammaMap bind:symbol={submittedSymbol} bind:aggregate={aggregate}/>
    {/if}
{/key}

<style>
</style>
