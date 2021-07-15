<script>
    import GammaExposure from './GammaExposure.svelte'

    let symbol = null;
    let data = null;

    let el;

    async function handleSubmit() {
        data = await (await fetch(`http://localhost:3030/gamma/${symbol}`)).json();
        console.log(data);
	}
</script>

<main>
    Symbol:
    <input bind:value={symbol}>
    <button on:click={handleSubmit}>
        Submit
    </button>

    {#if data}
        <GammaExposure data={data}/>
    {/if}
</main>

<style>
</style>
