<script>
    import { Router, Route } from "svelte-routing";
    import { clientIdUrl, storeClientId, logout } from './common/td';
    import Auth from './components/Auth.svelte';
    import GammaExposure from './components/GammaExposure.svelte';

    export let url = "";
    
    let clientIdText = "";
    let showAuth = false;

    function handleSaveClientId() {
        storeClientId(clientIdText);
        clientIdText = "";
    }

    function handleLogin() {
        const redirect_uri = encodeURIComponent("https://localhost:5000/auth");
        const url = `https://auth.tdameritrade.com/auth?response_type=code&redirect_uri=${redirect_uri}&client_id=${clientIdUrl()}`;
        window.location.href = url;
    }

    function handleLogout() {
        logout();
    }

</script>

<div class="auth">
    <button on:click={() => showAuth = !showAuth}>Auth</button>
    {#if showAuth}
        <div class="client-id">
            Client ID: <input bind:value={clientIdText} />
            <button on:click={handleSaveClientId}>
                Save
            </button>
        </div>
        <div class="login">
            <button on:click={handleLogin}>Login</button>
        </div>
        <div class="logout">
            <button on:click={handleLogout}>Logout</button>
        </div>
    {/if}
</div>

<Router url="{url}">
    <div>
        <Route path="/" component="{GammaExposure}" />
        <Route path="/auth" component="{Auth}" />
    </div>
</Router>

<style>
    .auth {
        width: 100%;
        display: flex;
        justify-content: flex-end;
    }
</style>
