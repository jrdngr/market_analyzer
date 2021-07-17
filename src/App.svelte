<script>
    import { Router, Route } from "svelte-routing";
    import { clientId, storeClientId } from './common/td';
    import Auth from './components/Auth.svelte';
    import GammaExposure from './components/GammaExposure.svelte';

    export let url = "";
    
    let clientIdText = "";

    function handleSaveClientId() {
        storeClientId(clientIdText);
        clientIdText = "";
    }

    function handleLogin() {
        const redirect_uri = encodeURIComponent("https://localhost:5000/auth");
        const url = `https://auth.tdameritrade.com/auth?response_type=code&redirect_uri=${redirect_uri}&client_id=${clientId()}%40AMER.OAUTHAP`;
        window.location.href = url;
    }

</script>
  
<Router url="{url}">
    <div>
        <Route path="/" component="{GammaExposure}" />
        <Route path="/auth" component="{Auth}" />
    </div>
</Router>

<div class="auth">
    <div class="client-id">
        Client ID: <input bind:value={clientIdText} />
        <button on:click={handleSaveClientId}>
            Save
        </button>
    </div>
    <div class="login">
        <button on:click={handleLogin}>Login</button>
    </div>
</div>

<style>
    .auth {
        margin-top: 100px;
    }
</style>
