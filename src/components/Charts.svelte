<script>
    import GammaExposure from './GammaExposure.svelte';
    import GammaPrice from './GammaPrice.svelte';

    let symbol = "SPY";
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
        <GammaExposure bind:symbol={submittedSymbol}/>
        <!-- <GammaPrice bind:symbol={submittedSymbol}/> -->
    {/if}
{/key}

<style>
</style>
