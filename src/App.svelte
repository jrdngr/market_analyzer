<script>
    import { Router, Route } from "svelte-routing";
    import { storeAccessToken, accessToken } from './common/apis/tradier';
    import Charts from './components/Charts.svelte';

    export let url = "";
    
    let accessTokenText = "";
    let showAuth = false;

    function handleSaveAccessToken() {
        storeAccessToken(accessTokenText);
        accessTokenText = "";
    }
</script>

<div class="auth">
    <button on:click={() => showAuth = !showAuth}>Auth</button>
    {#if showAuth}
        <div class="client-id">
            Access Token: <input bind:value={accessTokenText} />
            <button on:click={handleSaveAccessToken}>
                Save
            </button>
        </div>
    {/if}
</div>

<Router url="{url}">
    <div>
        <Route path="/" component="{Charts}" />
    </div>
</Router>

<style>
    .auth {
        width: 100%;
        display: flex;
        justify-content: flex-end;
    }
</style>
