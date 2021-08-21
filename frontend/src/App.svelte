<script>
    import { Router, Route } from "svelte-routing";
    import Charts from './components/Charts.svelte';
    import Dashboard from './components/Dashboard.svelte';

    export let url = "";
</script>

<Router url="{url}">
    <div>
        <Route path="/" component="{Dashboard}" />
        <Route path="/search" component="{Charts}" />
    </div>
</Router>

<style>
</style>
