<script>
    import { Router, Route } from "svelte-routing";
    import Charts from './components/Charts.svelte';
    import Dashboard from './components/Dashboard.svelte';
    import Auth from './components/Auth.svelte';
    import { clientIdUrl } from './common/apis/td';

    export let url = "";

    function handleLogin() {
        const redirect_uri = encodeURIComponent("https://localhost:5000/auth");
        const url = `https://auth.tdameritrade.com/auth?response_type=code&redirect_uri=${redirect_uri}&client_id=${clientIdUrl()}`;
        window.location.href = url;
    }
</script>

<Router url="{url}">
    <div>
        <Route path="/" component="{Dashboard}" />
        <Route path="/search" component="{Charts}" />
        <Route path="/auth" component="{Auth}" />
    </div>
</Router>

<button on:click={handleLogin}>TDA Login</button>

<style>
</style>
